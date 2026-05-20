-- Workspace Supabase schema.
-- brain.db 의 ws_* 테이블과 1:1 대응. Asurada sync 루프가 updated_at 기준 양방향 동기화.
-- 네이밍: public.ws_<table> — Asurada 기존 public.* 패턴과 일치.
--
-- 적용:
--   psql "$ASURADA_DATABASE_URL" -f supabase/migrations/0001_init.sql

-- ── ws_contacts (거래처 / 연락처) ─────────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_contacts (
    id           TEXT PRIMARY KEY,
    user_id      TEXT NOT NULL,
    name         TEXT NOT NULL,
    type         TEXT NOT NULL DEFAULT 'individual', -- 'individual' | 'company'
    company      TEXT,
    role         TEXT,
    email        TEXT,
    phone        TEXT,
    address      TEXT,
    tags         JSONB NOT NULL DEFAULT '[]'::jsonb,
    notes        TEXT,
    metadata     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at   TIMESTAMPTZ NOT NULL,
    updated_at   TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_contacts_user    ON public.ws_contacts(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_contacts_updated ON public.ws_contacts(updated_at);

-- ── ws_transactions (자금 관리) ───────────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_transactions (
    id               TEXT PRIMARY KEY,
    user_id          TEXT NOT NULL,
    type             TEXT NOT NULL,              -- 'income' | 'expense'
    amount           NUMERIC(15, 2) NOT NULL,
    currency         TEXT NOT NULL DEFAULT 'KRW',
    category         TEXT NOT NULL,
    description      TEXT,
    counterpart_id   TEXT REFERENCES public.ws_contacts(id) ON DELETE SET NULL,
    counterpart_name TEXT,
    date             DATE NOT NULL,
    receipt_url      TEXT,
    tax_deductible   BOOLEAN NOT NULL DEFAULT false,
    metadata         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_tx_user         ON public.ws_transactions(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_tx_user_date    ON public.ws_transactions(user_id, date DESC);
CREATE INDEX IF NOT EXISTS idx_ws_tx_updated      ON public.ws_transactions(updated_at);

-- ── ws_documents (문서 작업 / 관리) ──────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_documents (
    id               TEXT PRIMARY KEY,
    user_id          TEXT NOT NULL,
    title            TEXT NOT NULL,
    type             TEXT NOT NULL,              -- 'contract' | 'proposal' | 'report' | 'invoice' | 'receipt' | 'other'
    status           TEXT NOT NULL DEFAULT 'draft', -- 'draft' | 'final' | 'archived'
    content          TEXT,
    file_url         TEXT,
    tags             JSONB NOT NULL DEFAULT '[]'::jsonb,
    counterpart_id   TEXT REFERENCES public.ws_contacts(id) ON DELETE SET NULL,
    counterpart_name TEXT,
    expires_at       TIMESTAMPTZ,
    metadata         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_docs_user        ON public.ws_documents(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_docs_user_status ON public.ws_documents(user_id, status);
CREATE INDEX IF NOT EXISTS idx_ws_docs_expires     ON public.ws_documents(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_ws_docs_updated     ON public.ws_documents(updated_at);

-- ── ws_schedules (일정 조율) ──────────────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_schedules (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL,
    title       TEXT NOT NULL,
    type        TEXT NOT NULL DEFAULT 'general', -- 'meeting' | 'deadline' | 'payment' | 'general'
    start_at    TIMESTAMPTZ NOT NULL,
    end_at      TIMESTAMPTZ,
    all_day     BOOLEAN NOT NULL DEFAULT false,
    location    TEXT,
    description TEXT,
    recurrence  JSONB,
    reminder_at TIMESTAMPTZ,
    status      TEXT NOT NULL DEFAULT 'scheduled', -- 'scheduled' | 'done' | 'cancelled'
    metadata    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_sched_user       ON public.ws_schedules(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_sched_user_start ON public.ws_schedules(user_id, start_at);
CREATE INDEX IF NOT EXISTS idx_ws_sched_updated    ON public.ws_schedules(updated_at);

-- ── ws_tasks (운영 실무) ──────────────────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_tasks (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,
    status      TEXT NOT NULL DEFAULT 'todo',   -- 'todo' | 'in_progress' | 'done' | 'cancelled'
    priority    TEXT NOT NULL DEFAULT 'normal', -- 'urgent' | 'high' | 'normal' | 'low'
    due_at      TIMESTAMPTZ,
    done_at     TIMESTAMPTZ,
    schedule_id TEXT REFERENCES public.ws_schedules(id) ON DELETE SET NULL,
    tags        JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_tasks_user        ON public.ws_tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_tasks_user_status ON public.ws_tasks(user_id, status);
CREATE INDEX IF NOT EXISTS idx_ws_tasks_due         ON public.ws_tasks(due_at) WHERE due_at IS NOT NULL AND status != 'done';
CREATE INDEX IF NOT EXISTS idx_ws_tasks_updated     ON public.ws_tasks(updated_at);

-- ── ws_grants (지원사업 신청) ─────────────────────────────────
CREATE TABLE IF NOT EXISTS public.ws_grants (
    id           TEXT PRIMARY KEY,
    user_id      TEXT NOT NULL,
    name         TEXT NOT NULL,
    agency       TEXT NOT NULL,
    amount       NUMERIC(15, 2),
    currency     TEXT NOT NULL DEFAULT 'KRW',
    category     TEXT NOT NULL,                 -- '창업' | 'R&D' | '마케팅' | '고용' | ...
    status       TEXT NOT NULL DEFAULT 'discovered', -- 'discovered' | 'preparing' | 'submitted' | 'approved' | 'rejected' | 'cancelled'
    deadline_at  TIMESTAMPTZ,
    announced_at TIMESTAMPTZ,
    url          TEXT,
    requirements JSONB NOT NULL DEFAULT '[]'::jsonb, -- [{ text, done }]
    notes        TEXT,
    metadata     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at   TIMESTAMPTZ NOT NULL,
    updated_at   TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ws_grants_user        ON public.ws_grants(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_grants_user_status ON public.ws_grants(user_id, status);
CREATE INDEX IF NOT EXISTS idx_ws_grants_deadline    ON public.ws_grants(deadline_at)
    WHERE deadline_at IS NOT NULL AND status IN ('discovered', 'preparing');
CREATE INDEX IF NOT EXISTS idx_ws_grants_updated     ON public.ws_grants(updated_at);

-- ── RLS ──────────────────────────────────────────────────────
ALTER TABLE public.ws_contacts     ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.ws_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.ws_documents    ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.ws_schedules    ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.ws_tasks        ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.ws_grants       ENABLE ROW LEVEL SECURITY;

DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY[
        'public.ws_contacts', 'public.ws_transactions', 'public.ws_documents',
        'public.ws_schedules', 'public.ws_tasks', 'public.ws_grants'
    ] LOOP
        EXECUTE format(
            'DROP POLICY IF EXISTS "authenticated rw" ON %s;
             CREATE POLICY "authenticated rw" ON %s FOR ALL TO authenticated USING (true) WITH CHECK (true);',
            t, t
        );
    END LOOP;
END $$;
