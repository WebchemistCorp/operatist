use anyhow::Result;
use chrono::Local;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum TxCmd {
    /// 수입 기록
    Income {
        /// 금액 (원)
        amount: f64,
        /// 카테고리 (예: 매출, 프리랜서, 배당)
        #[arg(short, long)]
        category: String,
        /// 날짜 (YYYY-MM-DD, 기본: 오늘)
        #[arg(long)]
        date: Option<String>,
        /// 메모
        #[arg(long)]
        description: Option<String>,
        /// 거래처
        #[arg(long)]
        from: Option<String>,
    },
    /// 지출 기록
    Expense {
        /// 금액 (원)
        amount: f64,
        /// 카테고리 (예: 인건비, 광고비, 임대료, 식비)
        #[arg(short, long)]
        category: String,
        /// 날짜 (YYYY-MM-DD, 기본: 오늘)
        #[arg(long)]
        date: Option<String>,
        /// 메모
        #[arg(long)]
        description: Option<String>,
        /// 거래처
        #[arg(long)]
        to: Option<String>,
        /// 세금 공제 대상
        #[arg(long)]
        tax: bool,
    },
    /// 거래 목록
    List {
        /// 조회 건수 (기본: 20)
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// 월별 요약
    Summary {
        /// 월 (YYYY-MM, 기본: 이번 달)
        #[arg(long)]
        month: Option<String>,
    },
}

pub fn run(cmd: TxCmd) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    match cmd {
        TxCmd::Income { amount, category, date, description, from } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let date = date.unwrap_or_else(|| today.clone());
            brain::tx_insert(&conn, &user_id, "income", amount, &category, &date,
                description.as_deref(), from.as_deref(), false)?;
            println!("{} {} {}원 수입 기록", style("✓").green(), style(&date).dim(), style(format_amount(amount)).cyan());
            Ok(())
        }
        TxCmd::Expense { amount, category, date, description, to, tax } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let date = date.unwrap_or_else(|| today.clone());
            brain::tx_insert(&conn, &user_id, "expense", amount, &category, &date,
                description.as_deref(), to.as_deref(), tax)?;
            let tax_mark = if tax { " (공제)" } else { "" };
            println!("{} {} {}원 지출 기록{}", style("✓").green(), style(&date).dim(), style(format_amount(amount)).yellow(), style(tax_mark).dim());
            Ok(())
        }
        TxCmd::List { limit } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let txs = brain::tx_list(&conn, &user_id, limit)?;
            if txs.is_empty() {
                println!("{}", style("거래 내역이 없습니다.").dim());
                return Ok(());
            }
            println!("{:<12}  {:<8}  {:>14}  {:<12}  {}", style("날짜").bold(), style("구분").bold(), style("금액").bold(), style("카테고리").bold(), style("메모").bold());
            println!("{}", "-".repeat(70));
            for t in txs {
                let type_str = if t.r#type == "income" { style("수입").cyan() } else { style("지출").yellow() };
                let amount_str = format_amount(t.amount);
                let desc = t.description.or(t.counterpart_name).unwrap_or_default();
                println!("{:<12}  {:<8}  {:>14}  {:<12}  {}", t.date, type_str, amount_str, t.category, desc);
            }
            Ok(())
        }
        TxCmd::Summary { month } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let month = month.unwrap_or_else(|| Local::now().format("%Y-%m").to_string());
            let (income, expense) = brain::tx_summary(&conn, &user_id, &month)?;
            let net = income - expense;
            println!("── {} 요약 ──────────────────", style(&month).bold());
            println!("  수입:   {}", style(format_amount(income)).cyan());
            println!("  지출:   {}", style(format_amount(expense)).yellow());
            println!("  순이익: {}", if net >= 0.0 { style(format_amount(net)).green() } else { style(format_amount(net)).red() });
            Ok(())
        }
    }
}

fn format_amount(amount: f64) -> String {
    let n = amount as i64;
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    format!("{}원", result.chars().rev().collect::<String>())
}
