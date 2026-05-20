use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum TaskCmd {
    /// 태스크 추가
    Add {
        /// 제목
        title: String,
        /// 우선순위: urgent | high | normal | low
        #[arg(short, long, default_value = "normal")]
        priority: String,
        /// 마감일 (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,
        /// 설명
        #[arg(long)]
        description: Option<String>,
    },
    /// 태스크 목록
    List {
        /// 완료된 태스크 보기
        #[arg(long)]
        done: bool,
    },
    /// 태스크 완료 처리
    Done {
        /// 태스크 ID
        id: String,
    },
}

pub fn run(cmd: TaskCmd) -> Result<()> {
    match cmd {
        TaskCmd::Add {
            title,
            priority,
            due,
            description,
        } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let id = brain::task_insert(
                &conn,
                &user_id,
                &title,
                &priority,
                due.as_deref(),
                description.as_deref(),
            )?;
            println!(
                "{} [{}] {} ({})",
                style("✓").green(),
                priority_label(&priority),
                style(&title).bold(),
                style(&id).dim()
            );
            Ok(())
        }
        TaskCmd::List { done } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let tasks = brain::task_list(&conn, &user_id, done)?;
            if tasks.is_empty() {
                let msg = if done {
                    "완료된 태스크가 없습니다."
                } else {
                    "진행 중인 태스크가 없습니다."
                };
                println!("{}", style(msg).dim());
                return Ok(());
            }
            for t in tasks {
                let pri = priority_label(&t.priority);
                let due = t.due_at.map(|d| format!(" ~ {}", d)).unwrap_or_default();
                println!(
                    "{} {} {}{}",
                    pri,
                    t.title,
                    style(&t.id).dim(),
                    style(&due).yellow()
                );
            }
            Ok(())
        }
        TaskCmd::Done { id } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            if brain::task_done(&conn, &id, &user_id)? {
                println!("{} 완료 처리됨", style("✓").green());
            } else {
                println!("{} 해당 ID 없음", style("✗").red());
            }
            Ok(())
        }
    }
}

fn priority_label(priority: &str) -> console::StyledObject<&str> {
    match priority {
        "urgent" => style("🔴").for_stderr(),
        "high" => style("🟠").for_stderr(),
        "normal" => style("🟡").for_stderr(),
        _ => style("⚪").for_stderr(),
    }
}
