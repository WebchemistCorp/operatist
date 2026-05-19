use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn brain_db() -> Result<PathBuf> {
    let home = dirs::home_dir().context("홈 디렉토리 없음")?;
    Ok(home.join(".asurada").join("brain.db"))
}

pub fn asurada_config() -> Result<PathBuf> {
    let home = dirs::home_dir().context("홈 디렉토리 없음")?;
    Ok(home.join(".asurada").join("config.toml"))
}

pub fn load_user_id() -> Result<String> {
    let raw = std::fs::read_to_string(asurada_config()?)
        .context("config.toml 읽기 실패 — asurada init 필요")?;
    let doc: toml::Value = toml::from_str(&raw)?;
    doc.get("user")
        .and_then(|u| u.get("id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("[user].id 없음 — asurada init 필요"))
}
