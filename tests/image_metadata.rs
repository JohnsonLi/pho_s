use phos::handlers::image_handler::extract_image_metadata;

const FIXTURE: &str = "tests/test_images/northern_cardinal.jpg";

#[test]
fn filename_matches_fixture() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    assert_eq!(metadata.filename, "northern_cardinal.jpg");
}

#[test]
fn filesize_is_positive_and_matches_fs_metadata() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    let fs_len = std::fs::metadata(FIXTURE).unwrap().len();
    assert!(metadata.filesize > 0);
    assert_eq!(metadata.filesize, fs_len);
}

#[test]
fn dimensions_are_positive() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    assert!(metadata.dimensions.0 > 0, "width should be > 0");
    assert!(metadata.dimensions.1 > 0, "height should be > 0");
}

#[test]
fn exif_camera_fields_if_present_are_non_empty() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    if let Some(make) = metadata.camera_make.as_deref() {
        assert!(!make.is_empty(), "camera_make should be non-empty if present");
    }
    if let Some(model) = metadata.camera_model.as_deref() {
        assert!(!model.is_empty(), "camera_model should be non-empty if present");
    }
}

#[test]
fn exif_exposure_fields_if_present_are_positive() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    if let Some(a) = metadata.aperture.as_deref() { assert!(!a.is_empty()); }
    if let Some(s) = metadata.shutter_speed.as_deref() { assert!(!s.is_empty()); }
    if let Some(i) = metadata.iso.as_deref() { assert!(!i.is_empty()); }
    if let Some(f) = metadata.focal_length.as_deref() { assert!(!f.is_empty()); }
}

#[test]
fn orientation_if_present_is_in_valid_exif_range() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    if let Some(orientation) = metadata.orientation {
        assert!((1..=8).contains(&orientation), "orientation {} not in 1..=8", orientation);
    }
}

#[test]
fn missing_file_returns_none() {
    let result = extract_image_metadata("tests/test_images/__definitely_not_a_real_file__.jpg");
    assert!(result.is_none());
}
