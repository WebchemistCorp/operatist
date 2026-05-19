// Workspace ↔ brain.db thin client.
// Asurada 가 schema migration owner. Workspace 는 ws_* 테이블에만 읽기/쓰기.

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

    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='ws_assets'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if exists == 0 {
        return Err(anyhow!(
            "ws_assets 테이블이 없습니다.\nasurada 최신 버전으로 업그레이드 후 재시도하세요."
        ));
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
        params![id, user_id, name, category, purchase_date, purchase_price, currency,
                vendor, serial_number, notes, now],
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
        params![id, user_id, name, category, billing_cycle, amount, currency,
                next_billing_date, started_at, notes, now],
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
