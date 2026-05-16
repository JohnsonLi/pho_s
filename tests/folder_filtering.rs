use std::fs;

use phos::views::menu_bar::{collect_image_paths_sorted, VALID_IMAGE_EXTENSIONS};
use tempfile::tempdir;

fn touch(dir: &std::path::Path, name: &str) {
    fs::write(dir.join(name), b"").unwrap();
}

#[test]
fn returns_only_valid_image_extensions() {
    let dir = tempdir().unwrap();
    touch(dir.path(), "a.jpg");
    touch(dir.path(), "c.png");
    touch(dir.path(), "d.webp");
    touch(dir.path(), "e.txt");
    touch(dir.path(), "f.ARW");

    let names: Vec<String> = collect_image_paths_sorted(dir.path())
        .unwrap()
        .into_iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert_eq!(names, vec!["a.jpg", "c.png", "d.webp"]);
}

#[test]
fn extension_match_is_case_insensitive() {
    let dir = tempdir().unwrap();
    touch(dir.path(), "B.JPEG");
    touch(dir.path(), "C.PnG");

    let names: Vec<String> = collect_image_paths_sorted(dir.path())
        .unwrap()
        .into_iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert_eq!(names, vec!["B.JPEG", "C.PnG"]);
}

#[test]
fn excludes_subdirectories() {
    let dir = tempdir().unwrap();
    touch(dir.path(), "a.jpg");
    fs::create_dir(dir.path().join("nested")).unwrap();
    touch(&dir.path().join("nested"), "inner.jpg");

    let result = collect_image_paths_sorted(dir.path()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].file_name().unwrap(), "a.jpg");
}

#[test]
fn result_is_sorted_alphabetically() {
    let dir = tempdir().unwrap();
    for name in ["zebra.jpg", "apple.png", "mango.webp", "banana.jpeg"] {
        touch(dir.path(), name);
    }

    let names: Vec<String> = collect_image_paths_sorted(dir.path())
        .unwrap()
        .into_iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert_eq!(names, vec!["apple.png", "banana.jpeg", "mango.webp", "zebra.jpg"]);
}

#[test]
fn empty_directory_returns_empty_vec() {
    let dir = tempdir().unwrap();
    assert!(collect_image_paths_sorted(dir.path()).unwrap().is_empty());
}

#[test]
fn nonexistent_path_returns_err() {
    let result = collect_image_paths_sorted(std::path::Path::new("c:/this/path/does/not/exist/__phos_test__"));
    assert!(result.is_err());
}

#[test]
fn valid_extensions_constant_is_lowercase() {
    for ext in VALID_IMAGE_EXTENSIONS {
        assert_eq!(*ext, ext.to_lowercase(), "extension '{}' should be lowercase", ext);
    }
}
