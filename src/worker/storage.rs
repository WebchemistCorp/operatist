// Supabase Storage 업로드 클라이언트.
// ~/.asurada/config.toml [supabase] 섹션의 url, anon_key, storage_bucket 사용.

use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
    pub bucket: String,
}

pub fn load_config() -> Result<SupabaseConfig> {
    let raw = std::fs::read_to_string(crate::paths::asurada_config()?)
        .context("config.toml 읽기 실패 — asurada init 필요")?;
    let doc: toml::Value = toml::from_str(&raw)?;
    let sb = doc.get("supabase")
        .context("[supabase] 섹션 없음 — config.toml 에 추가 필요")?;
    let url = sb.get("url").and_then(|v| v.as_str())
        .context("[supabase].url 없음")?.to_string();
    let anon_key = sb.get("anon_key").and_then(|v| v.as_str())
        .context("[supabase].anon_key 없음")?.to_string();
    let bucket = sb.get("storage_bucket").and_then(|v| v.as_str())
        .unwrap_or("workspace-docs").to_string();
    Ok(SupabaseConfig { url, anon_key, bucket })
}

/// 파일을 Supabase Storage에 업로드하고 public URL 반환.
/// 경로: {bucket}/{user_id}/{doc_id}/{filename}
pub fn upload(config: &SupabaseConfig, user_id: &str, doc_id: &str, file_path: &Path) -> Result<String> {
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("파일명 없음")?;

    let content_type = mime_guess::from_path(file_path)
        .first_or_octet_stream()
        .to_string();

    let object_path = format!("{}/{}/{}", user_id, doc_id, filename);
    let upload_url = format!(
        "{}/storage/v1/object/{}/{}",
        config.url, config.bucket, object_path
    );

    let body = std::fs::read(file_path)
        .with_context(|| format!("파일 읽기 실패: {}", file_path.display()))?;

    let mut response = ureq::post(&upload_url)
        .header("Authorization", format!("Bearer {}", config.anon_key))
        .header("Content-Type", &content_type)
        .header("x-upsert", "true")
        .send(&body)
        .with_context(|| format!("Storage 업로드 실패: {}", upload_url))?;

    if response.status().as_u16() >= 400 {
        let status = response.status().as_u16();
        let body = response.body_mut().read_to_string().unwrap_or_default();
        anyhow::bail!("Storage 업로드 오류 {}: {}", status, body);
    }

    let public_url = format!(
        "{}/storage/v1/object/public/{}/{}",
        config.url, config.bucket, object_path
    );
    Ok(public_url)
}
