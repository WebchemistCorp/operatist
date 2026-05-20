use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum ContactCmd {
    /// 거래처 추가
    Add {
        /// 이름
        name: String,
        /// 타입: individual | company
        #[arg(short, long, default_value = "individual")]
        r#type: String,
        /// 회사명
        #[arg(long)]
        company: Option<String>,
        /// 직책/역할
        #[arg(long)]
        role: Option<String>,
        /// 이메일
        #[arg(long)]
        email: Option<String>,
        /// 전화번호
        #[arg(long)]
        phone: Option<String>,
        /// 메모
        #[arg(long)]
        notes: Option<String>,
    },
    /// 거래처 목록
    List,
    /// 거래처 삭제
    Remove {
        /// 거래처 ID
        id: String,
    },
}

pub fn run(cmd: ContactCmd) -> Result<()> {
    match cmd {
        ContactCmd::Add { name, r#type, company, role, email, phone, notes } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let id = brain::contact_insert(
                &conn, &user_id, &name, &r#type,
                company.as_deref(), role.as_deref(),
                email.as_deref(), phone.as_deref(), notes.as_deref(),
            )?;
            println!("{} {}", style("✓").green(), style(format!("거래처 추가: {} ({})", name, id)).dim());
            Ok(())
        }
        ContactCmd::List => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let contacts = brain::contact_list(&conn, &user_id)?;
            if contacts.is_empty() {
                println!("{}", style("등록된 거래처가 없습니다.").dim());
                return Ok(());
            }
            println!("{:<36}  {:<20}  {:<16}  {}", style("ID").bold(), style("이름").bold(), style("회사").bold(), style("연락처").bold());
            println!("{}", "-".repeat(90));
            for c in contacts {
                let company = c.company.unwrap_or_default();
                let contact = c.phone.or(c.email).unwrap_or_default();
                println!("{:<36}  {:<20}  {:<16}  {}", c.id, c.name, company, contact);
            }
            Ok(())
        }
        ContactCmd::Remove { id } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            if brain::contact_delete(&conn, &id, &user_id)? {
                println!("{} 삭제 완료", style("✓").green());
            } else {
                println!("{} 해당 ID 없음", style("✗").red());
            }
            Ok(())
        }
    }
}
