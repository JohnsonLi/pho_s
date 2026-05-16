use phos::handlers::image_handler::format_file_size;

#[test]
fn zero_bytes() {
    assert_eq!(format_file_size(0), "0 bytes");
}

#[test]
fn just_under_one_kb_is_bytes() {
    assert_eq!(format_file_size(1023), "1023 bytes");
}

#[test]
fn exactly_one_kb() {
    assert_eq!(format_file_size(1024), "1.00 KB");
}

#[test]
fn just_under_one_mb_is_kb() {
    let result = format_file_size(1_048_575);
    assert!(result.ends_with("KB"), "expected KB suffix, got {}", result);
}

#[test]
fn exactly_one_mb() {
    assert_eq!(format_file_size(1_048_576), "1.00 MB");
}

#[test]
fn exactly_one_gb() {
    assert_eq!(format_file_size(1_073_741_824), "1.00 GB");
}

#[test]
fn kb_has_two_decimal_precision() {
    assert_eq!(format_file_size(1536), "1.50 KB");
}

#[test]
fn mb_has_two_decimal_precision() {
    assert_eq!(format_file_size(1024 * 1024 + 1024 * 512), "1.50 MB");
}
