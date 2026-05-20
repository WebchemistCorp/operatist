-- 프로젝트 rename: Workspace → Administrate
-- ws_* 테이블을 op_* 로 rename. 0001_init.sql 을 이미 적용한 환경에서만 실행.
--
-- 적용:
--   psql "$ASURADA_DATABASE_URL" -f supabase/migrations/0002_rename_ws_to_adm.sql

ALTER TABLE IF EXISTS public.ws_contacts     RENAME TO op_contacts;
ALTER TABLE IF EXISTS public.ws_transactions RENAME TO op_transactions;
ALTER TABLE IF EXISTS public.ws_documents    RENAME TO op_documents;
ALTER TABLE IF EXISTS public.ws_schedules    RENAME TO op_schedules;
ALTER TABLE IF EXISTS public.ws_tasks        RENAME TO op_tasks;
ALTER TABLE IF EXISTS public.ws_grants       RENAME TO op_grants;

-- 인덱스 rename
ALTER INDEX IF EXISTS idx_ws_contacts_user    RENAME TO idx_op_contacts_user;
ALTER INDEX IF EXISTS idx_ws_contacts_updated RENAME TO idx_op_contacts_updated;
ALTER INDEX IF EXISTS idx_ws_tx_user          RENAME TO idx_op_tx_user;
ALTER INDEX IF EXISTS idx_ws_tx_user_date     RENAME TO idx_op_tx_user_date;
ALTER INDEX IF EXISTS idx_ws_tx_updated       RENAME TO idx_op_tx_updated;
ALTER INDEX IF EXISTS idx_ws_docs_user        RENAME TO idx_op_docs_user;
ALTER INDEX IF EXISTS idx_ws_docs_user_status RENAME TO idx_op_docs_user_status;
ALTER INDEX IF EXISTS idx_ws_docs_expires     RENAME TO idx_op_docs_expires;
ALTER INDEX IF EXISTS idx_ws_docs_updated     RENAME TO idx_op_docs_updated;
ALTER INDEX IF EXISTS idx_ws_sched_user       RENAME TO idx_op_sched_user;
ALTER INDEX IF EXISTS idx_ws_sched_user_start RENAME TO idx_op_sched_user_start;
ALTER INDEX IF EXISTS idx_ws_sched_updated    RENAME TO idx_op_sched_updated;
ALTER INDEX IF EXISTS idx_ws_tasks_user       RENAME TO idx_op_tasks_user;
ALTER INDEX IF EXISTS idx_ws_tasks_user_status RENAME TO idx_op_tasks_user_status;
ALTER INDEX IF EXISTS idx_ws_tasks_due        RENAME TO idx_op_tasks_due;
ALTER INDEX IF EXISTS idx_ws_tasks_updated    RENAME TO idx_op_tasks_updated;
ALTER INDEX IF EXISTS idx_ws_grants_user      RENAME TO idx_op_grants_user;
ALTER INDEX IF EXISTS idx_ws_grants_user_status RENAME TO idx_op_grants_user_status;
ALTER INDEX IF EXISTS idx_ws_grants_deadline  RENAME TO idx_op_grants_deadline;
ALTER INDEX IF EXISTS idx_ws_grants_updated   RENAME TO idx_op_grants_updated;
