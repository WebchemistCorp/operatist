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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Asset { cmd } => commands::asset::run(cmd),
        Commands::Sub { cmd } => commands::subscription::run(cmd),
    }
}
