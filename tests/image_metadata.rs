use phos::handlers::image_handler::extract_image_metadata;

const FIXTURE: &str = "test_images/DSC04406.jpg";

#[test]
fn filename_matches_fixture() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    assert_eq!(metadata.filename, "DSC04406.jpg");
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
fn exif_camera_fields_are_present_and_non_empty() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");

    let make = metadata.camera_make.as_deref().unwrap_or("");
    let model = metadata.camera_model.as_deref().unwrap_or("");
    assert!(!make.is_empty(), "camera_make should be non-empty");
    assert!(!model.is_empty(), "camera_model should be non-empty");
}

#[test]
fn exif_exposure_fields_are_present() {
    let metadata = extract_image_metadata(FIXTURE).expect("metadata should parse");
    assert!(metadata.aperture.is_some(), "aperture should be Some");
    assert!(metadata.shutter_speed.is_some(), "shutter_speed should be Some");
    assert!(metadata.iso.is_some(), "iso should be Some");
    assert!(metadata.focal_length.is_some(), "focal_length should be Some");
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
    let result = extract_image_metadata("test_images/__definitely_not_a_real_file__.jpg");
    assert!(result.is_none());
}
