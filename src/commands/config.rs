use anyhow::{bail, Context, Result};
use clap::Subcommand;
use console::style;

#[derive(Subcommand)]
pub enum ConfigCmd {
    /// 설정 값 저장 (키: database-url | storage-bucket | anon-key | service-role-key | url)
    Set { key: String, value: String },
    /// 현재 설정 출력
    Show,
}

pub fn run(cmd: ConfigCmd) -> Result<()> {
    match cmd {
        ConfigCmd::Set { key, value } => {
            let toml_key = match key.as_str() {
                "database-url"     => "database_url",
                "storage-bucket"   => "storage_bucket",
                "anon-key"         => "anon_key",
                "service-role-key" => "service_role_key",
                "url"              => "url",
                other => bail!("알 수 없는 키: {other}\n유효한 키: database-url | storage-bucket | anon-key | service-role-key | url"),
            };
            save_supabase_key(toml_key, &value)?;
            println!("{} [supabase].{toml_key} 저장 완료", style("✓").green());
            Ok(())
        }
        ConfigCmd::Show => {
            let path = crate::paths::asurada_config()?;
            let raw = std::fs::read_to_string(&path)
                .with_context(|| format!("config 읽기 실패: {}", path.display()))?;
            let doc: toml::Value = toml::from_str(&raw)?;

            println!("{}", style("~/.asurada/config.toml [supabase]").bold());
            if let Some(sb) = doc.get("supabase") {
                for (k, v) in sb.as_table().into_iter().flatten() {
                    let display =
                        if k == "anon_key" || k == "database_url" || k == "service_role_key" {
                            mask(v.as_str().unwrap_or(""))
                        } else {
                            v.as_str().unwrap_or("").to_string()
                        };
                    println!("  {:<20} = {}", style(k).cyan(), display);
                }
            } else {
                println!("  {}", style("(비어있음)").dim());
            }
            Ok(())
        }
    }
}

fn mask(s: &str) -> String {
    if s.len() <= 8 {
        return "****".to_string();
    }
    format!("{}****{}", &s[..4], &s[s.len() - 4..])
}

fn save_supabase_key(key: &str, value: &str) -> Result<()> {
    let path = crate::paths::asurada_config()?;
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("config 읽기 실패: {}", path.display()))?;

    let mut doc: toml::value::Table = toml::from_str(&raw).unwrap_or_default();

    let sb = doc
        .entry("supabase")
        .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));

    if let toml::Value::Table(t) = sb {
        t.insert(key.to_string(), toml::Value::String(value.to_string()));
    }

    let out = toml::to_string_pretty(&toml::Value::Table(doc)).context("TOML 직렬화 실패")?;
    std::fs::write(&path, out).with_context(|| format!("config 쓰기 실패: {}", path.display()))?;
    Ok(())
}
