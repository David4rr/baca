use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

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
    let parse = |s: &str| -> Vec<u64> {
        s.trim_start_matches(['v', 'V'])
            .split('.')
            .map(|p| {
                p.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap_or(0)
            })
            .collect()
    };

    let remote_parts = parse(remote);
    let current_parts = parse(current);
    let max_len = remote_parts.len().max(current_parts.len());

    for i in 0..max_len {
        let r_val = *remote_parts.get(i).unwrap_or(&0);
        let c_val = *current_parts.get(i).unwrap_or(&0);
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
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Gagal inisialisasi HTTP client: {e}"))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi server update: {e}"))?;

    if !res.status().is_success() {
        return Err(format!("Server mengembalikan status: {}", res.status()));
    }

    let release: GithubRelease = res
        .json()
        .await
        .map_err(|e| format!("Gagal membaca data update: {e}"))?;

    let clean_remote_version = release.tag_name.trim_start_matches(['v', 'V']).to_string();
    let mut apk_url = None;
    let mut apk_size = None;
    let mut desktop_url = None;
    let mut desktop_size = None;

    let os_hint = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    };
    let mut desktop_candidates: Vec<&GithubAsset> = Vec::new();

    for asset in &release.assets {
        let lower = asset.name.to_lowercase();
        if lower.ends_with(".apk") {
            apk_url = Some(asset.browser_download_url.clone());
            apk_size = Some(asset.size);
        } else if lower.ends_with(".tar.gz") || lower.ends_with(".zip") || lower.ends_with(".dmg") {
            desktop_candidates.push(asset);
        }
    }

    if let Some(asset) = desktop_candidates
        .iter()
        .find(|a| a.name.to_lowercase().contains(os_hint))
        .or_else(|| desktop_candidates.first())
    {
        desktop_url = Some(asset.browser_download_url.clone());
        desktop_size = Some(asset.size);
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

/// Directory used for staging update artifacts. Always app-private; never the
/// shared Download folder where another app could swap the payload.
pub fn get_update_storage_dir() -> Result<PathBuf, String> {
    let dir = crate::db::Database::get_app_dir()?.join("updates");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Gagal menyiapkan folder update: {e}"))?;
    Ok(dir)
}

fn parse_checksum(raw: &str) -> Option<String> {
    let token = raw.split_whitespace().next()?;
    let candidate = token.trim().to_ascii_lowercase();
    if candidate.len() == 64 && candidate.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(candidate)
    } else {
        None
    }
}

fn checksum_for_file(body: &str, filename: &str) -> Option<String> {
    body.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let hash = parts.next()?;
        let name = parts.next().unwrap_or("");
        if name.trim_start_matches('*') == filename {
            parse_checksum(hash)
        } else {
            None
        }
    })
}

async fn fetch_expected_checksum(url: &str) -> Result<String, String> {
    let filename = url.rsplit('/').next().unwrap_or("");
    let dir = url.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    let client = reqwest::Client::builder()
        .user_agent("Baca-Reader-App")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Gagal inisialisasi HTTP client: {e}"))?;

    // Release workbook layout: a sibling SHA256SUMS.txt.
    let sums_url = format!("{dir}/SHA256SUMS.txt");
    if let Ok(res) = client.get(&sums_url).send().await {
        if res.status().is_success() {
            if let Ok(body) = res.text().await {
                if let Some(hash) = checksum_for_file(&body, filename) {
                    return Ok(hash);
                }
            }
        }
    }

    // Optional per-asset layout: <asset>.sha256.
    let checksum_url = format!("{url}.sha256");
    let res = client
        .get(&checksum_url)
        .send()
        .await
        .map_err(|e| format!("Gagal mengunduh checksum: {e}"))?;
    if !res.status().is_success() {
        return Err(format!("Checksum tidak tersedia (status {})", res.status()));
    }
    let body = res.text().await.map_err(|e| format!("Gagal membaca checksum: {e}"))?;
    parse_checksum(&body).ok_or_else(|| "Format checksum tidak valid".to_string())
}

/// Downloads an update asset, verifies its SHA-256 against the published
/// `.sha256` sibling asset, then atomically stores it. Fails closed.
pub async fn download_update_package<F>(
    download_url: &str,
    target_filename: &str,
    on_progress: F,
) -> Result<PathBuf, String>
where
    F: Fn(f32) + Send + 'static,
{
    let expected = fetch_expected_checksum(download_url).await?;
    let target_dir = get_update_storage_dir()?;
    let file_path = target_dir.join(target_filename);
    let part_path = target_dir.join(format!("{target_filename}.part"));

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Baca-Reader/3.4; +https://github.com/David4rr/baca)")
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|e| format!("HTTP Client Error: {e}"))?;

    let mut res = client
        .get(download_url)
        .send()
        .await
        .map_err(|e| format!("Gagal mengunduh berkas update: {e}"))?;

    if !res.status().is_success() {
        return Err(format!("Gagal mengunduh berkas, status: {}", res.status()));
    }

    let total_size = res.content_length().unwrap_or(0);
    let mut file = std::fs::File::create(&part_path)
        .map_err(|e| format!("Gagal membuat berkas lokal: {e}"))?;

    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = res
        .chunk()
        .await
        .map_err(|e| format!("Error membaca data unduhan: {e}"))?
    {
        file.write_all(&chunk)
            .map_err(|e| format!("Gagal menulis data ke berkas: {e}"))?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let pct = (downloaded as f32 / total_size as f32) * 100.0;
            on_progress(pct.min(99.0));
        }
    }

    file.flush()
        .map_err(|e| format!("Gagal memfinalisasi berkas: {e}"))?;
    drop(file);

    let actual = hex::encode(hasher.finalize());
    if actual != expected {
        let _ = std::fs::remove_file(&part_path);
        return Err("Checksum berkas update tidak cocok; unduhan dibatalkan".to_string());
    }

    std::fs::rename(&part_path, &file_path)
        .map_err(|e| format!("Gagal menyimpan berkas update: {e}"))?;
    on_progress(100.0);

    Ok(file_path)
}

/// Installs a verified update artifact. On success the current process is
/// replaced and never returns.
pub fn install_update(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        return android_install::install_apk(path);
    }
    #[cfg(not(target_os = "android"))]
    {
        desktop_install::install(path)
    }
}

#[cfg(not(target_os = "android"))]
mod desktop_install {
    use super::*;

    pub fn install(path: &Path) -> Result<(), String> {
        let staging = std::env::temp_dir().join(format!("baca-update-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&staging)
            .map_err(|e| format!("Gagal menyiapkan folder ekstraksi: {e}"))?;

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.ends_with(".tar.gz") {
            let status = std::process::Command::new("tar")
                .args(["-xzf"])
                .arg(path)
                .arg("-C")
                .arg(&staging)
                .status()
                .map_err(|e| format!("Gagal mengekstrak update: {e}"))?;
            if !status.success() {
                return Err("Ekstraksi arsip update gagal".to_string());
            }
        } else if name.ends_with(".zip") {
            let file = std::fs::File::open(path).map_err(|e| format!("Gagal membuka arsip: {e}"))?;
            let mut archive = zip::ZipArchive::new(file)
                .map_err(|e| format!("Arsip update tidak valid: {e}"))?;
            archive
                .extract(&staging)
                .map_err(|e| format!("Gagal mengekstrak update: {e}"))?;
        } else {
            return Err("Format pembaruan tidak didukung".to_string());
        }

        let binary = find_binary(&staging)?;
        let current = std::env::current_exe()
            .map_err(|e| format!("Gagal menemukan lokasi aplikasi: {e}"))?;

        #[cfg(windows)]
        {
            let script = staging.join("baca_apply_update.bat");
            let body = format!(
                "@echo off\r\ntimeout /t 2 /nobreak >nul\r\nmove /Y \"{}\" \"{}\"\r\nstart \"\" \"{}\"\r\ndel \"%~f0\"\r\n",
                binary.display(),
                current.display(),
                current.display()
            );
            std::fs::write(&script, body)
                .map_err(|e| format!("Gagal menyiapkan pemasangan: {e}"))?;
            std::process::Command::new("cmd")
                .args(["/C", "start", "", &script.to_string_lossy()])
                .spawn()
                .map_err(|e| format!("Gagal menjalankan pemasangan: {e}"))?;
            std::process::exit(0);
        }

        #[cfg(not(windows))]
        {
        let staged = current.with_extension("update");

        std::fs::copy(&binary, &staged)
            .map_err(|e| format!("Gagal menyalin pembaruan: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| format!("Gagal mengatur izin pembaruan: {e}"))?;
        }
        std::fs::rename(&staged, &current)
            .map_err(|e| format!("Gagal memasang pembaruan: {e}"))?;

        std::process::Command::new(&current)
            .spawn()
            .map_err(|e| format!("Gagal menjalankan versi baru: {e}"))?;
        std::process::exit(0);
        }
    }

    fn find_binary(dir: &Path) -> Result<PathBuf, String> {
        let mut stack = vec![dir.to_path_buf()];
        while let Some(current) = stack.pop() {
            let entries = std::fs::read_dir(&current)
                .map_err(|e| format!("Gagal membaca hasil ekstraksi: {e}"))?;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("._") {
                    continue;
                }
                if name.ends_with(".app") || name.starts_with("baca") {
                    return Ok(path);
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(meta) = path.metadata() {
                        if meta.permissions().mode() & 0o111 != 0 {
                            return Ok(path);
                        }
                    }
                }
            }
        }
        Err("Berkas aplikasi tidak ditemukan di dalam arsip".to_string())
    }
}

#[cfg(target_os = "android")]
mod android_install {
    use super::*;
    use jni::objects::{JObject, JValue};
    use jni::JavaVM;

    pub fn install_apk(path: &Path) -> Result<(), String> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }
            .map_err(|e| format!("Gagal mengakses runtime Android: {e}"))?;
        let mut env = vm
            .attach_current_thread()
            .map_err(|e| format!("Gagal mengakses runtime Android: {e}"))?;
        let context = unsafe { JObject::from_raw(ctx.context().cast()) };

        let path_j = env
            .new_string(path.to_string_lossy())
            .map_err(|e| format!("Gagal menyiapkan path: {e}"))?;
        let file = env
            .new_object(
                "java/io/File",
                "(Ljava/lang/String;)V",
                &[JValue::Object(&path_j)],
            )
            .map_err(|e| format!("Gagal membuat objek berkas: {e}"))?;

        let package = env
            .call_method(&context, "getPackageName", "()Ljava/lang/String;", &[])
            .map_err(|e| format!("Gagal membaca paket aplikasi: {e}"))?
            .l()
            .map_err(|e| format!("Gagal membaca paket aplikasi: {e}"))?;
        let package_jstr = jni::objects::JString::from(package);
        let package_str: String = env
            .get_string(&package_jstr)
            .map_err(|e| format!("Gagal membaca paket aplikasi: {e}"))?
            .into();
        let authority = env
            .new_string(format!("{package_str}.fileprovider"))
            .map_err(|e| format!("Gagal menyiapkan provider: {e}"))?;

        let uri = env
            .call_static_method(
                "androidx/core/content/FileProvider",
                "getUriForFile",
                "(Landroid/content/Context;Ljava/lang/String;Ljava/io/File;)Landroid/net/Uri;",
                &[
                    JValue::Object(&context),
                    JValue::Object(&authority),
                    JValue::Object(&file),
                ],
            )
            .map_err(|e| format!("Gagal menyiapkan URI pemasangan: {e}"))?
            .l()
            .map_err(|e| format!("Gagal menyiapkan URI pemasangan: {e}"))?;

        let intent = env
            .new_object("android/content/Intent", "()V", &[])
            .map_err(|e| format!("Gagal membuat intent: {e}"))?;
        let action = env
            .new_string("android.intent.action.VIEW")
            .map_err(|e| format!("Gagal menyiapkan aksi: {e}"))?;
        env.call_method(
            &intent,
            "setAction",
            "(Ljava/lang/String;)Landroid/content/Intent;",
            &[JValue::Object(&action)],
        )
        .map_err(|e| format!("Gagal menyiapkan aksi: {e}"))?;

        let mime = env
            .new_string("application/vnd.android.package-archive")
            .map_err(|e| format!("Gagal menyiapkan tipe berkas: {e}"))?;
        env.call_method(
            &intent,
            "setDataAndType",
            "(Landroid/net/Uri;Ljava/lang/String;)Landroid/content/Intent;",
            &[JValue::Object(&uri), JValue::Object(&mime)],
        )
        .map_err(|e| format!("Gagal menyiapkan intent: {e}"))?;
        env.call_method(
            &intent,
            "addFlags",
            "(I)Landroid/content/Intent;",
            &[JValue::Int(1 | 0x10000000)],
        )
        .map_err(|e| format!("Gagal menyiapkan izin intent: {e}"))?;

        env.call_method(
            &context,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValue::Object(&intent)],
        )
        .map_err(|e| format!("Gagal membuka pemasang paket: {e}"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{checksum_for_file, is_newer_version, parse_checksum};

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

    #[test]
    fn checksum_sums_file_is_parsed() {
        let hash = "b".repeat(64);
        let body = format!("{hash}  baca-v3.3.4-macos.tar.gz\n{}  other.zip\n", "c".repeat(64));
        assert_eq!(
            checksum_for_file(&body, "baca-v3.3.4-macos.tar.gz"),
            Some(hash)
        );
        assert_eq!(checksum_for_file(&body, "missing.zip"), None);
    }

    #[test]
    fn checksum_format_is_validated() {
        let hash = "a".repeat(64);
        assert_eq!(parse_checksum(&format!("{hash}  file.tar.gz")), Some(hash));
        assert_eq!(parse_checksum("not-a-hash"), None);
    }
}
