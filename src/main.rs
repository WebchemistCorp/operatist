mod commands;
mod paths;
mod worker;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "workspace")]
#[command(version, about = "Company operations CLI for Webchemist — powered by Asurada")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 회사 자산 관리 (노트북, 차량, 가구 등)
    Asset {
        #[command(subcommand)]
        cmd: commands::asset::AssetCmd,
    },
    /// 외부 서비스 구독 관리 (결제일, 금액)
    Sub {
        #[command(subcommand)]
        cmd: commands::subscription::SubCmd,
    },
    /// 거래처 / 연락처 관리
    Contact {
        #[command(subcommand)]
        cmd: commands::contact::ContactCmd,
    },
    /// 자금 입출금 기록 (수입/지출)
    Tx {
        #[command(subcommand)]
        cmd: commands::tx::TxCmd,
    },
    /// 문서 관리 (계약서, 제안서, 보고서)
    Doc {
        #[command(subcommand)]
        cmd: commands::doc::DocCmd,
    },
    /// 일정 관리 (미팅, 마감, 납부일)
    Schedule {
        #[command(subcommand)]
        cmd: commands::schedule::ScheduleCmd,
    },
    /// 운영 태스크 관리
    Task {
        #[command(subcommand)]
        cmd: commands::task::TaskCmd,
    },
    /// 지원사업 신청 관리
    Grant {
        #[command(subcommand)]
        cmd: commands::grant::GrantCmd,
    },
    /// Supabase 테이블/버킷 초기 설정 (최초 1회)
    Setup,
    /// 설정 관리 (~/.asurada/config.toml)
    Config {
        #[command(subcommand)]
        cmd: commands::config::ConfigCmd,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Asset { cmd }    => commands::asset::run(cmd),
        Commands::Sub { cmd }      => commands::subscription::run(cmd),
        Commands::Contact { cmd }  => commands::contact::run(cmd),
        Commands::Tx { cmd }       => commands::tx::run(cmd),
        Commands::Doc { cmd }      => commands::doc::run(cmd),
        Commands::Schedule { cmd } => commands::schedule::run(cmd),
        Commands::Task { cmd }     => commands::task::run(cmd),
        Commands::Grant { cmd }    => commands::grant::run(cmd),
        Commands::Setup            => commands::setup::run(),
        Commands::Config { cmd }   => commands::config::run(cmd),
    }
}
