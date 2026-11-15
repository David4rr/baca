use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub version: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    pub html_url: String,
    pub apk_url: Option<String>,
    pub apk_size: Option<u64>,
    pub desktop_url: Option<String>,
    pub desktop_size: Option<u64>,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    published_at: Option<String>,
    html_url: String,
    #[serde(default)]
    assets: Vec<GithubAsset>,
}

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

/// Compares two semver strings (e.g. "v3.3.2" and "3.3.1").
/// Returns true if remote is strictly newer than current.
pub fn is_newer_version(remote: &str, current: &str) -> bool {
    let parse_parts = |v: &str| -> Vec<u32> {
        let clean = v.trim().trim_start_matches('v').trim_start_matches('V');
        let semver_core = clean.split('-').next().unwrap_or(clean);
        semver_core
            .split('.')
            .filter_map(|p| p.parse::<u32>().ok())
            .collect()
    };

    let r_parts = parse_parts(remote);
    let c_parts = parse_parts(current);

    let max_len = r_parts.len().max(c_parts.len());
    for i in 0..max_len {
        let r_val = r_parts.get(i).copied().unwrap_or(0);
        let c_val = c_parts.get(i).copied().unwrap_or(0);
        if r_val > c_val {
            return true;
        } else if r_val < c_val {
            return false;
        }
    }

    false
}

/// Checks GitHub Releases API for the latest published version of Baca.
pub async fn check_latest_release(current_version: &str) -> Result<Option<ReleaseInfo>, String> {
    let url = "https://api.github.com/repos/David4rr/baca/releases/latest";
    let client = reqwest::Client::builder()
        .user_agent("Baca-Reader-App")
        .build()
        .map_err(|e| format!("Gagal inisialisasi HTTP client: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi server update: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Server mengembalikan status: {}", res.status()));
    }

    let release: GithubRelease = res
        .json()
        .await
        .map_err(|e| format!("Gagal membaca data update: {}", e))?;

    let clean_remote_version = release.tag_name.trim_start_matches(['v', 'V']).to_string();
    let mut apk_url = None;
    let mut apk_size = None;
    let mut desktop_url = None;
    let mut desktop_size = None;

    for asset in &release.assets {
        let lower = asset.name.to_lowercase();
        if lower.ends_with(".apk") {
            apk_url = Some(asset.browser_download_url.clone());
            apk_size = Some(asset.size);
        } else if lower.ends_with(".tar.gz") || lower.ends_with(".zip") || lower.ends_with(".dmg") {
            desktop_url = Some(asset.browser_download_url.clone());
            desktop_size = Some(asset.size);
        }
    }

    if is_newer_version(&release.tag_name, current_version) {
        Ok(Some(ReleaseInfo {
            tag_name: release.tag_name.clone(),
            version: clean_remote_version,
            name: release.name.unwrap_or_else(|| release.tag_name.clone()),
            body: release.body.unwrap_or_default(),
            published_at: release.published_at.unwrap_or_default(),
            html_url: release.html_url,
            apk_url,
            apk_size,
            desktop_url,
            desktop_size,
        }))
    } else {
        Ok(None)
    }
}

/// Resolves directory where update files are temporarily stored.
pub fn get_update_storage_dir() -> PathBuf {
    let mut candidates = Vec::new();

    #[cfg(target_os = "android")]
    {
        candidates.push(PathBuf::from("/sdcard/Download"));
        candidates.push(PathBuf::from("/storage/emulated/0/Download"));
        let app_dir = crate::db::Database::get_app_dir();
        candidates.push(app_dir.join("updates"));
        candidates.push(app_dir);
    }

    #[cfg(not(target_os = "android"))]
    {
        let app_dir = crate::db::Database::get_app_dir();
        candidates.push(app_dir.join("updates"));
        if let Some(data) = dirs::data_local_dir() {
            candidates.push(data.join("baca").join("updates"));
        }
        if let Some(dl) = dirs::download_dir() {
            candidates.push(dl.join("baca_updates"));
        }
    }

    candidates.push(std::env::temp_dir().join("baca_updates"));
    candidates.push(std::env::temp_dir());

    for dir in candidates {
        if std::fs::create_dir_all(&dir).is_ok() {
            let test_file = dir.join(".write_test_baca");
            if std::fs::write(&test_file, b"ok").is_ok() {
                let _ = std::fs::remove_file(&test_file);
                return dir;
            }
        }
    }

    std::env::temp_dir()
}

/// Downloads an update asset with real-time percentage progress reporting.
pub async fn download_update_package<F>(
    download_url: &str,
    target_filename: &str,
    on_progress: F,
) -> Result<PathBuf, String>
where
    F: Fn(f32) + Send + 'static,
{
    let target_dir = get_update_storage_dir();
    let _ = std::fs::create_dir_all(&target_dir);
    let mut file_path = target_dir.join(target_filename);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Baca-Reader/3.3; +https://github.com/David4rr/baca)")
        .redirect(reqwest::redirect::Policy::limited(15))
        .build()
        .map_err(|e| format!("HTTP Client Error: {}", e))?;

    let mut res = client
        .get(download_url)
        .send()
        .await
        .map_err(|e| format!("Gagal mengunduh berkas update: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Gagal mengunduh berkas, status: {}", res.status()));
    }

    let total_size = res.content_length().unwrap_or(0);
    use std::io::Write;
    let mut file = match std::fs::File::create(&file_path) {
        Ok(f) => f,
        Err(_) => {
            let fallback_dir = std::env::temp_dir();
            let _ = std::fs::create_dir_all(&fallback_dir);
            file_path = fallback_dir.join(target_filename);
            std::fs::File::create(&file_path)
                .map_err(|e| format!("Gagal membuat berkas lokal: {}", e))?
        }
    };

    let mut downloaded: u64 = 0;

    while let Some(chunk) = res
        .chunk()
        .await
        .map_err(|e| format!("Error membaca data unduhan: {}", e))?
    {
        file.write_all(&chunk)
            .map_err(|e| format!("Gagal menulis data ke berkas: {}", e))?;

        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let pct = (downloaded as f32 / total_size as f32) * 100.0;
            on_progress(pct.min(99.0));
        } else {
            on_progress(50.0);
        }
    }

    file.flush()
        .map_err(|e| format!("Gagal memfinalisasi berkas: {}", e))?;
    on_progress(100.0);

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparisons() {
        assert!(is_newer_version("v3.3.2", "3.3.1"));
        assert!(is_newer_version("3.4.0", "3.3.1"));
        assert!(is_newer_version("4.0.0", "3.3.1"));
        assert!(is_newer_version("v3.3.1.1", "3.3.1"));

        assert!(!is_newer_version("v3.3.1", "3.3.1"));
        assert!(!is_newer_version("3.3.1", "3.3.1"));
        assert!(!is_newer_version("3.3.0", "3.3.1"));
        assert!(!is_newer_version("2.9.9", "3.3.1"));
    }
}
