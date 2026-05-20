use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::worker::brain;

#[derive(Subcommand)]
pub enum ScheduleCmd {
    /// 일정 추가
    Add {
        /// 제목
        title: String,
        /// 시작 일시 (YYYY-MM-DD HH:MM 또는 YYYY-MM-DD)
        #[arg(long)]
        at: String,
        /// 타입: meeting | deadline | payment | general
        #[arg(short, long, default_value = "general")]
        r#type: String,
        /// 장소
        #[arg(long)]
        location: Option<String>,
        /// 메모
        #[arg(long)]
        description: Option<String>,
    },
    /// 예정 일정 목록
    List {
        /// 지난 일정 포함
        #[arg(long)]
        all: bool,
    },
}

pub fn run(cmd: ScheduleCmd) -> Result<()> {
    match cmd {
        ScheduleCmd::Add {
            title,
            at,
            r#type,
            location,
            description,
        } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let start_at = normalize_datetime(&at);
            let id = brain::schedule_insert(
                &conn,
                &user_id,
                &title,
                &r#type,
                &start_at,
                location.as_deref(),
                description.as_deref(),
            )?;
            println!(
                "{} {} @ {} ({})",
                style("✓").green(),
                style(&title).bold(),
                style(&start_at).cyan(),
                style(&id).dim()
            );
            Ok(())
        }
        ScheduleCmd::List { all } => {
            let conn = brain::open(&crate::paths::brain_db()?)?;
            let user_id = crate::paths::load_user_id()?;
            let schedules = brain::schedule_list(&conn, &user_id, !all)?;
            if schedules.is_empty() {
                println!("{}", style("예정된 일정이 없습니다.").dim());
                return Ok(());
            }
            println!(
                "{:<20}  {:<10}  {:<20}  {}",
                style("일시").bold(),
                style("타입").bold(),
                style("장소").bold(),
                style("제목").bold()
            );
            println!("{}", "-".repeat(70));
            for s in schedules {
                let location = s.location.unwrap_or_default();
                let start = s.start_at.get(..16).unwrap_or(&s.start_at).to_string();
                println!(
                    "{:<20}  {:<10}  {:<20}  {}",
                    start, s.r#type, location, s.title
                );
            }
            Ok(())
        }
    }
}

fn normalize_datetime(input: &str) -> String {
    if input.len() == 10 {
        format!("{}T00:00:00+09:00", input)
    } else if input.contains(' ') {
        format!("{}:00+09:00", input.replace(' ', "T"))
    } else {
        input.to_string()
    }
}
