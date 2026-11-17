use baca::engine::updater::{is_newer_version, get_update_storage_dir};

#[test]
fn test_semver_comparisons() {
    // Normal patch bump
    assert!(is_newer_version("v3.3.2", "3.3.1"));
    assert!(is_newer_version("3.3.2", "3.3.1"));

    // Minor and major bumps
    assert!(is_newer_version("v3.4.0", "3.3.1"));
    assert!(is_newer_version("v4.0.0", "3.3.1"));
    assert!(is_newer_version("10.0.0", "3.3.1"));

    // Quad semver
    assert!(is_newer_version("v3.3.1.1", "3.3.1"));

    // Equal versions
    assert!(!is_newer_version("v3.3.1", "3.3.1"));
    assert!(!is_newer_version("3.3.1", "3.3.1"));
    assert!(!is_newer_version("3.3.1", "v3.3.1"));

    // Older versions
    assert!(!is_newer_version("v3.3.0", "3.3.1"));
    assert!(!is_newer_version("v3.2.0", "3.3.1"));
    assert!(!is_newer_version("v2.9.9", "3.3.1"));
    assert!(!is_newer_version("v1.0.0", "3.3.1"));
}

#[test]
fn test_update_storage_dir() {
    let dir = get_update_storage_dir().expect("update dir should resolve");
    assert!(dir.to_string_lossy().contains("updates"));
    assert!(dir.exists());
}
