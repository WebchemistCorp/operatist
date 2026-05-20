use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum SubCmd {
    /// 구독 추가
    Add {
        /// 서비스명 (예: "Higgsfield", "Claude Code")
        name: String,
        /// 월 금액 (원 단위)
        amount: i64,
        /// 결제 주기: monthly | annual | one_time
        #[arg(short, long, default_value = "monthly")]
        cycle: String,
        /// 카테고리: ai_tool | infra | design | service
        #[arg(long, default_value = "service")]
        category: String,
        /// 다음 결제일 (YYYY-MM-DD)
        #[arg(long)]
        next: Option<String>,
        /// 구독 시작일 (YYYY-MM-DD)
        #[arg(long)]
        started: Option<String>,
        /// 메모
        #[arg(long)]
        notes: Option<String>,
    },
    /// 구독 목록 (다음 결제일 순)
    List,
}

pub fn run(cmd: SubCmd) -> Result<()> {
    match cmd {
        SubCmd::Add {
            name,
            amount,
            cycle,
            category,
            next,
            started,
            notes,
        } => add(name, amount, cycle, category, next, started, notes),
        SubCmd::List => list(),
    }
}

fn add(
    name: String,
    amount: i64,
    cycle: String,
    category: String,
    next: Option<String>,
    started: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    let conn = brain::open(&crate::paths::brain_db()?)?;
    let user_id = crate::paths::load_user_id()?;

    let id = brain::sub_insert(
        &conn,
        &user_id,
        &name,
        &category,
        &cycle,
        amount,
        "KRW",
        next.as_deref(),
        started.as_deref(),
        notes.as_deref(),
    )?;

    println!(
        "{} 구독 추가됨: {}",
        style("✓").green(),
        style(&name).bold()
    );
    println!("  ID: {}", &id[..8.min(id.len())]);
    println!("  금액: {}원 / {}", format_amount(amount), cycle);
    if let Some(d) = &next {
        println!("  다음 결제일: {}", d);
    }
    Ok(())
}

fn list() -> Result<()> {
    let conn = brain::open(&crate::paths::brain_db()?)?;
    let user_id = crate::paths::load_user_id()?;
    let subs = brain::sub_list(&conn, &user_id)?;

    if subs.is_empty() {
        println!("(등록된 구독 없음)");
        return Ok(());
    }

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut monthly_total: i64 = 0;

    println!("{:<8}  {:<20}  {:<12}  {:<14}  다음 결제일", "ID", "서비스명", "카테고리", "금액");
    println!("{}", "─".repeat(72));

    for s in &subs {
        let next = s.next_billing_date.as_deref().unwrap_or("-");
        let overdue = s
            .next_billing_date
            .as_deref()
            .map(|d| d < today.as_str())
            .unwrap_or(false);
        let next_display = if overdue {
            format!("{} !", style(next).red())
        } else {
            next.to_string()
        };
        println!(
            "{:<8}  {:<20}  {:<12}  {:<14}  {}",
            &s.id[..8.min(s.id.len())],
            truncate(&s.name, 20),
            s.category,
            format!("{}원", format_amount(s.amount)),
            next_display,
        );
        if s.billing_cycle == "monthly" {
            monthly_total += s.amount;
        } else if s.billing_cycle == "annual" {
            monthly_total += s.amount / 12;
        }
    }
    println!("{}", "─".repeat(72));
    println!(
        "  월 합계 (연간 → 월할): {}원",
        format_amount(monthly_total)
    );
    Ok(())
}

fn format_amount(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    }
}
