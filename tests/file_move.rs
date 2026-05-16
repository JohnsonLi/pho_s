use std::fs;

use phos::controllers::file_manager::move_file;
use tempfile::tempdir;

#[test]
fn moves_jpg_into_destination_directory() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("photo.jpg");
    let dst_dir = dir.path().join("keep");
    fs::write(&src, b"jpg-bytes").unwrap();

    move_file(&src, &dst_dir).unwrap();

    assert!(!src.exists(), "source should be gone");
    let dst_file = dst_dir.join("photo.jpg");
    assert!(dst_file.exists(), "destination file should exist");
    assert_eq!(fs::read(&dst_file).unwrap(), b"jpg-bytes");
}

#[test]
fn moves_sibling_arw_when_present() {
    let dir = tempdir().unwrap();
    let src_jpg = dir.path().join("photo.jpg");
    let src_arw = dir.path().join("photo.ARW");
    let dst_dir = dir.path().join("keep");
    fs::write(&src_jpg, b"jpg").unwrap();
    fs::write(&src_arw, b"arw").unwrap();

    move_file(&src_jpg, &dst_dir).unwrap();

    assert!(!src_jpg.exists());
    assert!(!src_arw.exists());
    assert!(dst_dir.join("photo.jpg").exists());
    assert!(dst_dir.join("photo.ARW").exists());
}

#[test]
fn succeeds_when_arw_sibling_does_not_exist() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("photo.jpg");
    let dst_dir = dir.path().join("keep");
    fs::write(&src, b"jpg").unwrap();

    let result = move_file(&src, &dst_dir);

    assert!(result.is_ok(), "expected Ok even without ARW sibling, got {:?}", result);
    assert!(dst_dir.join("photo.jpg").exists());
    assert!(!dst_dir.join("photo.ARW").exists());
}

#[test]
fn auto_creates_destination_directory() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("photo.jpg");
    let dst_dir = dir.path().join("brand").join("new").join("nested");
    fs::write(&src, b"x").unwrap();

    move_file(&src, &dst_dir).unwrap();

    assert!(dst_dir.is_dir());
    assert!(dst_dir.join("photo.jpg").exists());
}

#[test]
fn returns_err_when_source_missing() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("nonexistent.jpg");
    let dst_dir = dir.path().join("keep");

    let result = move_file(&src, &dst_dir);
    assert!(result.is_err());
}
