// Workspace ↔ brain.db thin client.
// Asurada 가 schema migration owner. Workspace 는 ws_* 테이블에만 읽기/쓰기.
#![allow(dead_code, clippy::too_many_arguments)]

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

pub fn open(path: &Path) -> Result<Connection> {
    if !path.exists() {
        return Err(anyhow!(
            "brain.db not found at {}.\nasurada init 을 먼저 실행하세요.",
            path.display()
        ));
    }
    let conn = Connection::open(path).with_context(|| format!("open {}", path.display()))?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "busy_timeout", 5000)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    for table in &[
        "ws_assets",
        "ws_contacts",
        "ws_transactions",
        "ws_documents",
        "ws_schedules",
        "ws_tasks",
        "ws_grants",
    ] {
        let exists: i64 = conn
            .query_row(
                &format!(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
                    table
                ),
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        if exists == 0 {
            return Err(anyhow!(
                "{} 테이블이 없습니다.\nasurada 최신 버전(v0.3.2+)으로 업그레이드 후 재시도하세요.",
                table
            ));
        }
    }
    Ok(conn)
}

pub fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    let rand = std::ptr::addr_of!(nanos) as usize;
    format!("{:016x}-{:08x}-{:08x}", nanos, pid, rand)
}

// ── 자산 ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub category: String,
    pub purchase_date: Option<String>,
    pub purchase_price: Option<i64>,
    pub currency: String,
    pub vendor: Option<String>,
    pub serial_number: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

pub fn asset_insert(
    conn: &Connection,
    user_id: &str,
    name: &str,
    category: &str,
    purchase_date: Option<&str>,
    purchase_price: Option<i64>,
    currency: &str,
    vendor: Option<&str>,
    serial_number: Option<&str>,
    notes: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_assets
           (id, user_id, name, category, purchase_date, purchase_price, currency,
            vendor, serial_number, status, notes, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,'active',?10,'{}',?11,?11)"#,
        params![
            id,
            user_id,
            name,
            category,
            purchase_date,
            purchase_price,
            currency,
            vendor,
            serial_number,
            notes,
            now
        ],
    )?;
    Ok(id)
}

pub fn asset_list(conn: &Connection, user_id: &str) -> Result<Vec<Asset>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, name, category, purchase_date, purchase_price, currency,
                  vendor, serial_number, status, notes
           FROM ws_assets WHERE user_id = ?1 AND status != 'disposed'
           ORDER BY created_at DESC"#,
    )?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Asset {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                purchase_date: row.get(3)?,
                purchase_price: row.get(4)?,
                currency: row.get(5)?,
                vendor: row.get(6)?,
                serial_number: row.get(7)?,
                status: row.get(8)?,
                notes: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

// ── 구독 ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub category: String,
    pub billing_cycle: String,
    pub amount: i64,
    pub currency: String,
    pub next_billing_date: Option<String>,
    pub started_at: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

pub fn sub_insert(
    conn: &Connection,
    user_id: &str,
    name: &str,
    category: &str,
    billing_cycle: &str,
    amount: i64,
    currency: &str,
    next_billing_date: Option<&str>,
    started_at: Option<&str>,
    notes: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_subscriptions
           (id, user_id, name, category, billing_cycle, amount, currency,
            next_billing_date, started_at, status, notes, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,'active',?10,'{}',?11,?11)"#,
        params![
            id,
            user_id,
            name,
            category,
            billing_cycle,
            amount,
            currency,
            next_billing_date,
            started_at,
            notes,
            now
        ],
    )?;
    Ok(id)
}

pub fn sub_list(conn: &Connection, user_id: &str) -> Result<Vec<Subscription>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, name, category, billing_cycle, amount, currency,
                  next_billing_date, started_at, status, notes
           FROM ws_subscriptions WHERE user_id = ?1 AND status = 'active'
           ORDER BY next_billing_date ASC NULLS LAST"#,
    )?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Subscription {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                billing_cycle: row.get(3)?,
                amount: row.get(4)?,
                currency: row.get(5)?,
                next_billing_date: row.get(6)?,
                started_at: row.get(7)?,
                status: row.get(8)?,
                notes: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

// ── 거래처 ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub company: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

pub fn contact_insert(
    conn: &Connection,
    user_id: &str,
    name: &str,
    contact_type: &str,
    company: Option<&str>,
    role: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    notes: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_contacts
           (id, user_id, name, type, company, role, email, phone, notes,
            tags, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,'[]','{}',?10,?10)"#,
        params![
            id,
            user_id,
            name,
            contact_type,
            company,
            role,
            email,
            phone,
            notes,
            now
        ],
    )?;
    Ok(id)
}

pub fn contact_list(conn: &Connection, user_id: &str) -> Result<Vec<Contact>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, type, company, role, email, phone, notes
         FROM ws_contacts WHERE user_id=?1 ORDER BY name ASC",
    )?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
                r#type: row.get(2)?,
                company: row.get(3)?,
                role: row.get(4)?,
                email: row.get(5)?,
                phone: row.get(6)?,
                notes: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

pub fn contact_delete(conn: &Connection, id: &str, user_id: &str) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM ws_contacts WHERE id=?1 AND user_id=?2",
        params![id, user_id],
    )?;
    Ok(n > 0)
}

// ── 자금 ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub r#type: String,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub description: Option<String>,
    pub counterpart_name: Option<String>,
    pub date: String,
    pub tax_deductible: bool,
}

pub fn tx_insert(
    conn: &Connection,
    user_id: &str,
    tx_type: &str,
    amount: f64,
    category: &str,
    date: &str,
    description: Option<&str>,
    counterpart_name: Option<&str>,
    tax_deductible: bool,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_transactions
           (id, user_id, type, amount, currency, category, date, description,
            counterpart_name, tax_deductible, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,'KRW',?5,?6,?7,?8,?9,'{}',?10,?10)"#,
        params![
            id,
            user_id,
            tx_type,
            amount,
            category,
            date,
            description,
            counterpart_name,
            tax_deductible as i64,
            now
        ],
    )?;
    Ok(id)
}

pub fn tx_list(conn: &Connection, user_id: &str, limit: usize) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, type, amount, currency, category, description, counterpart_name, date, tax_deductible
         FROM ws_transactions WHERE user_id=?1 ORDER BY date DESC, created_at DESC LIMIT ?2",
    )?;
    let rows = stmt
        .query_map(params![user_id, limit as i64], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                r#type: row.get(1)?,
                amount: row.get(2)?,
                currency: row.get(3)?,
                category: row.get(4)?,
                description: row.get(5)?,
                counterpart_name: row.get(6)?,
                date: row.get(7)?,
                tax_deductible: {
                    let v: i64 = row.get(8)?;
                    v != 0
                },
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

pub fn tx_summary(conn: &Connection, user_id: &str, month: &str) -> Result<(f64, f64)> {
    let prefix = format!("{}%", month);
    let income: f64 = conn.query_row(
        "SELECT COALESCE(SUM(amount),0) FROM ws_transactions WHERE user_id=?1 AND type='income' AND date LIKE ?2",
        params![user_id, prefix], |r| r.get(0),
    ).unwrap_or(0.0);
    let expense: f64 = conn.query_row(
        "SELECT COALESCE(SUM(amount),0) FROM ws_transactions WHERE user_id=?1 AND type='expense' AND date LIKE ?2",
        params![user_id, prefix], |r| r.get(0),
    ).unwrap_or(0.0);
    Ok((income, expense))
}

// ── 문서 ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub status: String,
    pub counterpart_name: Option<String>,
    pub expires_at: Option<String>,
    pub updated_at: String,
}

pub fn doc_insert(
    conn: &Connection,
    user_id: &str,
    title: &str,
    doc_type: &str,
    content: Option<&str>,
    counterpart_name: Option<&str>,
    expires_at: Option<&str>,
    file_url: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_documents
           (id, user_id, title, type, status, content, counterpart_name,
            expires_at, file_url, tags, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,'draft',?5,?6,?7,?8,'[]','{}',?9,?9)"#,
        params![
            id,
            user_id,
            title,
            doc_type,
            content,
            counterpart_name,
            expires_at,
            file_url,
            now
        ],
    )?;
    Ok(id)
}

pub fn doc_list(conn: &Connection, user_id: &str) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, type, status, counterpart_name, expires_at, updated_at
         FROM ws_documents WHERE user_id=?1 AND status!='archived'
         ORDER BY updated_at DESC",
    )?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Document {
                id: row.get(0)?,
                title: row.get(1)?,
                r#type: row.get(2)?,
                status: row.get(3)?,
                counterpart_name: row.get(4)?,
                expires_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

// ── 일정 ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Schedule {
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub start_at: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
}

pub fn schedule_insert(
    conn: &Connection,
    user_id: &str,
    title: &str,
    sched_type: &str,
    start_at: &str,
    location: Option<&str>,
    description: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_schedules
           (id, user_id, title, type, start_at, location, description,
            metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,?5,?6,?7,'{}',?8,?8)"#,
        params![
            id,
            user_id,
            title,
            sched_type,
            start_at,
            location,
            description,
            now
        ],
    )?;
    Ok(id)
}

pub fn schedule_list(
    conn: &Connection,
    user_id: &str,
    upcoming_only: bool,
) -> Result<Vec<Schedule>> {
    let sql = if upcoming_only {
        "SELECT id, title, type, start_at, location, description, status
         FROM ws_schedules WHERE user_id=?1 AND status='scheduled' AND start_at >= datetime('now')
         ORDER BY start_at ASC LIMIT 20"
    } else {
        "SELECT id, title, type, start_at, location, description, status
         FROM ws_schedules WHERE user_id=?1
         ORDER BY start_at DESC LIMIT 30"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Schedule {
                id: row.get(0)?,
                title: row.get(1)?,
                r#type: row.get(2)?,
                start_at: row.get(3)?,
                location: row.get(4)?,
                description: row.get(5)?,
                status: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

// ── 태스크 ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub due_at: Option<String>,
    pub description: Option<String>,
}

pub fn task_insert(
    conn: &Connection,
    user_id: &str,
    title: &str,
    priority: &str,
    due_at: Option<&str>,
    description: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_tasks
           (id, user_id, title, status, priority, due_at, description,
            tags, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,'todo',?4,?5,?6,'[]','{}',?7,?7)"#,
        params![id, user_id, title, priority, due_at, description, now],
    )?;
    Ok(id)
}

pub fn task_list(conn: &Connection, user_id: &str, done: bool) -> Result<Vec<Task>> {
    let sql = if done {
        "SELECT id, title, status, priority, due_at, description
         FROM ws_tasks WHERE user_id=?1 AND status='done'
         ORDER BY done_at DESC LIMIT 20"
    } else {
        "SELECT id, title, status, priority, due_at, description
         FROM ws_tasks WHERE user_id=?1 AND status IN ('todo','in_progress')
         ORDER BY CASE priority WHEN 'urgent' THEN 0 WHEN 'high' THEN 1 WHEN 'normal' THEN 2 ELSE 3 END,
                  due_at ASC NULLS LAST"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt
        .query_map(params![user_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                priority: row.get(3)?,
                due_at: row.get(4)?,
                description: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

pub fn task_done(conn: &Connection, id: &str, user_id: &str) -> Result<bool> {
    let now = Utc::now().to_rfc3339();
    let n = conn.execute(
        "UPDATE ws_tasks SET status='done', done_at=?1, updated_at=?1 WHERE id=?2 AND user_id=?3",
        params![now, id, user_id],
    )?;
    Ok(n > 0)
}

// ── 지원사업 ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Grant {
    pub id: String,
    pub name: String,
    pub agency: String,
    pub amount: Option<f64>,
    pub currency: String,
    pub category: String,
    pub status: String,
    pub deadline_at: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
}

pub fn grant_insert(
    conn: &Connection,
    user_id: &str,
    name: &str,
    agency: &str,
    category: &str,
    amount: Option<f64>,
    deadline_at: Option<&str>,
    url: Option<&str>,
    notes: Option<&str>,
) -> Result<String> {
    let id = uuid_like();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        r#"INSERT INTO ws_grants
           (id, user_id, name, agency, category, amount, currency, status,
            deadline_at, url, notes, requirements, metadata, created_at, updated_at)
           VALUES (?1,?2,?3,?4,?5,?6,'KRW','discovered',?7,?8,?9,'[]','{}',?10,?10)"#,
        params![
            id,
            user_id,
            name,
            agency,
            category,
            amount,
            deadline_at,
            url,
            notes,
            now
        ],
    )?;
    Ok(id)
}

pub fn grant_list(conn: &Connection, user_id: &str, status: Option<&str>) -> Result<Vec<Grant>> {
    let (sql, bind_status) = if let Some(s) = status {
        (
            "SELECT id, name, agency, amount, currency, category, status, deadline_at, url, notes
             FROM ws_grants WHERE user_id=?1 AND status=?2
             ORDER BY deadline_at ASC NULLS LAST",
            Some(s.to_string()),
        )
    } else {
        (
            "SELECT id, name, agency, amount, currency, category, status, deadline_at, url, notes
             FROM ws_grants WHERE user_id=?1 AND status NOT IN ('rejected','cancelled')
             ORDER BY deadline_at ASC NULLS LAST",
            None,
        )
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(s) = bind_status {
        stmt.query_map(params![user_id, s], row_to_grant)?
            .filter_map(|r| r.ok())
            .collect()
    } else {
        stmt.query_map(params![user_id], row_to_grant)?
            .filter_map(|r| r.ok())
            .collect()
    };
    Ok(rows)
}

pub fn grant_update_status(
    conn: &Connection,
    id: &str,
    user_id: &str,
    status: &str,
) -> Result<bool> {
    let now = Utc::now().to_rfc3339();
    let n = conn.execute(
        "UPDATE ws_grants SET status=?1, updated_at=?2 WHERE id=?3 AND user_id=?4",
        params![status, now, id, user_id],
    )?;
    Ok(n > 0)
}

fn row_to_grant(row: &rusqlite::Row<'_>) -> rusqlite::Result<Grant> {
    Ok(Grant {
        id: row.get(0)?,
        name: row.get(1)?,
        agency: row.get(2)?,
        amount: row.get(3)?,
        currency: row.get(4)?,
        category: row.get(5)?,
        status: row.get(6)?,
        deadline_at: row.get(7)?,
        url: row.get(8)?,
        notes: row.get(9)?,
    })
}
