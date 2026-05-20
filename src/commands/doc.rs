use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum DocCmd {
    /// 문서 추가
    Add {
        /// 문서 제목
        title: String,
        /// 타입: contract | proposal | report | invoice | receipt | other
        #[arg(short, long, default_value = "other")]
        r#type: String,
        /// 관련 거래처
        #[arg(long)]
        with: Option<String>,
        /// 만료일 (YYYY-MM-DD)
        #[arg(long)]
        expires: Option<String>,
        /// 내용 (짧은 텍스트)
        #[arg(long)]
        content: Option<String>,
    },
    /// 문서 목록
    List,
}

pub fn run(cmd: DocCmd) -> Result<()> {
    match cmd {
        DocCmd::Add { title, r#type, with, expires, content } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let id = brain::doc_insert(
                &conn, &user_id, &title, &r#type,
                content.as_deref(), with.as_deref(), expires.as_deref(),
            )?;
            println!("{} {} ({})", style("✓").green(), style(&title).bold(), style(&id).dim());
            Ok(())
        }
        DocCmd::List => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let docs = brain::doc_list(&conn, &user_id)?;
            if docs.is_empty() {
                println!("{}", style("등록된 문서가 없습니다.").dim());
                return Ok(());
            }
            println!("{:<36}  {:<12}  {:<8}  {:<20}  {}",
                style("ID").bold(), style("타입").bold(), style("상태").bold(),
                style("거래처").bold(), style("제목").bold());
            println!("{}", "-".repeat(90));
            for d in docs {
                let counterpart = d.counterpart_name.unwrap_or_default();
                let status_styled = match d.status.as_str() {
                    "final" => style(d.status.clone()).green(),
                    "draft" => style(d.status.clone()).yellow(),
                    _ => style(d.status.clone()).dim(),
                };
                println!("{:<36}  {:<12}  {:<8}  {:<20}  {}", d.id, d.r#type, status_styled, counterpart, d.title);
            }
            Ok(())
        }
    }
}
