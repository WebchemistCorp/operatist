use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum GrantCmd {
    /// 지원사업 추가
    Add {
        /// 사업명
        name: String,
        /// 주관기관
        #[arg(long)]
        agency: String,
        /// 카테고리 (창업 | R&D | 마케팅 | 고용 | 기타)
        #[arg(short, long, default_value = "기타")]
        category: String,
        /// 지원금액 (원, 미정이면 생략)
        #[arg(long)]
        amount: Option<f64>,
        /// 신청 마감일 (YYYY-MM-DD)
        #[arg(long)]
        deadline: Option<String>,
        /// 공고 URL
        #[arg(long)]
        url: Option<String>,
        /// 메모
        #[arg(long)]
        notes: Option<String>,
    },
    /// 지원사업 목록
    List {
        /// 상태 필터 (discovered | preparing | submitted | approved)
        #[arg(long)]
        status: Option<String>,
    },
    /// 상태 변경 (preparing → submitted → approved 등)
    Status {
        /// 지원사업 ID
        id: String,
        /// 새 상태
        status: String,
    },
}

pub fn run(cmd: GrantCmd) -> Result<()> {
    match cmd {
        GrantCmd::Add { name, agency, category, amount, deadline, url, notes } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let id = brain::grant_insert(
                &conn, &user_id, &name, &agency, &category,
                amount, deadline.as_deref(), url.as_deref(), notes.as_deref(),
            )?;
            let amount_str = amount.map(|a| format!(" / {}원", a as i64)).unwrap_or_default();
            println!("{} {} — {}{} ({})", style("✓").green(), style(&name).bold(), agency, amount_str, style(&id).dim());
            Ok(())
        }
        GrantCmd::List { status } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let grants = brain::grant_list(&conn, &user_id, status.as_deref())?;
            if grants.is_empty() {
                println!("{}", style("등록된 지원사업이 없습니다.").dim());
                return Ok(());
            }
            println!("{:<36}  {:<12}  {:<16}  {:<12}  {}",
                style("ID").bold(), style("상태").bold(), style("마감일").bold(),
                style("기관").bold(), style("사업명").bold());
            println!("{}", "-".repeat(90));
            for g in grants {
                let deadline = g.deadline_at.as_deref().and_then(|d| d.get(..10)).unwrap_or("-").to_string();
                let status_styled = match g.status.as_str() {
                    "approved"   => style(g.status.clone()).green(),
                    "submitted"  => style(g.status.clone()).cyan(),
                    "preparing"  => style(g.status.clone()).yellow(),
                    "discovered" => style(g.status.clone()).dim(),
                    _            => style(g.status.clone()).red(),
                };
                println!("{:<36}  {:<12}  {:<16}  {:<12}  {}", g.id, status_styled, deadline, g.agency, g.name);
            }
            Ok(())
        }
        GrantCmd::Status { id, status } => {
            let valid = ["discovered", "preparing", "submitted", "approved", "rejected", "cancelled"];
            if !valid.contains(&status.as_str()) {
                anyhow::bail!("유효한 상태: {}", valid.join(" | "));
            }
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            if brain::grant_update_status(&conn, &id, &user_id, &status)? {
                println!("{} 상태 → {}", style("✓").green(), style(&status).bold());
            } else {
                println!("{} 해당 ID 없음", style("✗").red());
            }
            Ok(())
        }
    }
}
