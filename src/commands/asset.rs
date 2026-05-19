use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum AssetCmd {
    /// 자산 추가
    Add {
        /// 자산명 (예: "맥북 프로 M3")
        name: String,
        /// 카테고리: equipment | furniture | vehicle | software
        #[arg(short, long, default_value = "equipment")]
        category: String,
        /// 구매일 (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// 구매금액 (원 단위)
        #[arg(long)]
        price: Option<i64>,
        /// 구매처
        #[arg(long)]
        vendor: Option<String>,
        /// 시리얼 번호
        #[arg(long)]
        serial: Option<String>,
        /// 메모
        #[arg(long)]
        notes: Option<String>,
    },
    /// 자산 목록
    List,
}

pub fn run(cmd: AssetCmd) -> Result<()> {
    match cmd {
        AssetCmd::Add { name, category, date, price, vendor, serial, notes } => {
            add(name, category, date, price, vendor, serial, notes)
        }
        AssetCmd::List => list(),
    }
}

fn add(
    name: String,
    category: String,
    date: Option<String>,
    price: Option<i64>,
    vendor: Option<String>,
    serial: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    let conn = brain::open(&crate::paths::brain_db()?)?;
    let user_id = crate::paths::load_user_id()?;

    let id = brain::asset_insert(
        &conn, &user_id, &name, &category,
        date.as_deref(), price, "KRW",
        vendor.as_deref(), serial.as_deref(), notes.as_deref(),
    )?;

    println!("{} 자산 추가됨: {}", style("✓").green(), style(&name).bold());
    println!("  ID: {}", &id[..8.min(id.len())]);
    if let Some(p) = price {
        println!("  금액: {}원", format_amount(p));
    }
    Ok(())
}

fn list() -> Result<()> {
    let conn = brain::open(&crate::paths::brain_db()?)?;
    let user_id = crate::paths::load_user_id()?;
    let assets = brain::asset_list(&conn, &user_id)?;

    if assets.is_empty() {
        println!("(등록된 자산 없음)");
        return Ok(());
    }
    println!("{:<8}  {:<20}  {:<12}  {:<14}  {}",
        "ID", "이름", "카테고리", "금액", "구매일");
    println!("{}", "─".repeat(72));
    for a in assets {
        let price = a.purchase_price
            .map(|p| format!("{}원", format_amount(p)))
            .unwrap_or_else(|| "-".to_string());
        let date = a.purchase_date.as_deref().unwrap_or("-");
        println!("{:<8}  {:<20}  {:<12}  {:<14}  {}",
            &a.id[..8.min(a.id.len())],
            truncate(&a.name, 20),
            a.category,
            price,
            date,
        );
    }
    Ok(())
}

fn format_amount(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() }
    else { format!("{}…", s.chars().take(max - 1).collect::<String>()) }
}
