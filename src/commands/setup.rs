use anyhow::{bail, Context, Result};
use console::style;
use std::io::Write;
use std::process::{Command, Stdio};

const MIGRATION_SQL: &str = include_str!("../../supabase/migrations/0001_init.sql");

pub fn run() -> Result<()> {
    let db_url = resolve_database_url()?;

    println!("{} Supabase 마이그레이션 실행 중...", style("→").cyan());

    // psql 설치 확인
    if Command::new("psql").arg("--version").output().is_err() {
        bail!(
            "psql 을 찾을 수 없습니다.\n\
             macOS: brew install libpq && brew link --force libpq\n\
             또는 ~/.asurada/config.toml [supabase] database_url 과 psql 설치 후 재시도"
        );
    }

    let mut child = Command::new("psql")
        .arg(&db_url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("psql 실행 실패")?;

    child
        .stdin
        .take()
        .unwrap()
        .write_all(MIGRATION_SQL.as_bytes())
        .context("SQL 전송 실패")?;

    let output = child.wait_with_output().context("psql 대기 실패")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("마이그레이션 실패:\n{}", stderr);
    }

    println!("{} ws_contacts, ws_transactions, ws_documents, ws_schedules, ws_tasks, ws_grants 테이블 적용 완료", style("✓").green());

    // Storage 버킷 생성 시도
    create_storage_bucket()?;

    println!(
        "{} workspace setup 완료 — `workspace doc add` 등 커맨드를 사용할 수 있습니다.",
        style("✓").green().bold()
    );
    Ok(())
}

fn resolve_database_url() -> Result<String> {
    let raw = std::fs::read_to_string(crate::paths::asurada_config()?)
        .context("config.toml 읽기 실패 — asurada init 필요")?;
    let doc: toml::Value = toml::from_str(&raw)?;

    doc.get("supabase")
        .and_then(|s| s.get("database_url"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "database_url 이 설정되지 않았습니다.\n\
             \n\
             Supabase 대시보드 → Project Settings → Database → Connection string (URI) 복사 후:\n\
             \n\
             workspace config set database-url \"postgresql://postgres.[ref]:[pw]@...\""
            )
        })
}

fn create_storage_bucket() -> Result<()> {
    let raw = match std::fs::read_to_string(crate::paths::asurada_config()?) {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };
    let doc: toml::Value = match toml::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };
    let sb = match doc.get("supabase") {
        Some(v) => v,
        None => return Ok(()),
    };
    let url = match sb.get("url").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return Ok(()),
    };
    // service_role_key 우선, 없으면 anon_key (버킷 생성은 service_role 필요)
    let key = sb
        .get("service_role_key")
        .or_else(|| sb.get("anon_key"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let bucket = sb
        .get("storage_bucket")
        .and_then(|v| v.as_str())
        .unwrap_or("workspace-docs")
        .to_string();

    let body = format!(r#"{{"id":"{bucket}","name":"{bucket}","public":false}}"#);
    let resp = ureq::post(&format!("{}/storage/v1/bucket", url))
        .header("apikey", &key)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .send(body.as_bytes());

    match resp {
        Ok(r) if r.status().as_u16() < 400 => {
            println!("{} Storage 버킷 '{}' 생성 완료", style("✓").green(), bucket);
        }
        Ok(r) if r.status().as_u16() == 400 => {
            // 이미 존재 — 정상
            println!("{} Storage 버킷 '{}' 이미 존재", style("·").dim(), bucket);
        }
        _ => {
            println!(
                "{} Storage 버킷 생성 실패 (service_role 키 필요) — Supabase 대시보드에서 '{}' 버킷을 수동 생성하세요.",
                style("!").yellow(),
                bucket
            );
        }
    }

    Ok(())
}
