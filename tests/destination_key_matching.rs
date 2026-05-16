use std::collections::HashMap;
use std::path::PathBuf;

use phos::handlers::input_handler::find_destination_for_key;

fn map_of(entries: &[(&str, &str)]) -> HashMap<PathBuf, String> {
    entries
        .iter()
        .map(|(p, k)| (PathBuf::from(p), k.to_string()))
        .collect()
}

#[test]
fn matches_first_character() {
    let keys = map_of(&[("/dst/keep", "k")]);
    let result = find_destination_for_key('k', &keys);
    assert_eq!(result, Some(&PathBuf::from("/dst/keep")));
}

#[test]
fn returns_none_when_no_key_matches() {
    let keys = map_of(&[("/dst/keep", "k"), ("/dst/reject", "r")]);
    assert!(find_destination_for_key('z', &keys).is_none());
}

#[test]
fn empty_value_string_is_skipped() {
    let keys = map_of(&[("/dst/unassigned", "")]);
    assert!(find_destination_for_key('\0', &keys).is_none());
}

#[test]
fn multi_character_value_matches_on_first_char_only() {
    let keys = map_of(&[("/dst/keep", "keep")]);
    assert_eq!(find_destination_for_key('k', &keys), Some(&PathBuf::from("/dst/keep")));
    assert!(find_destination_for_key('e', &keys).is_none());
}

#[test]
fn empty_map_returns_none() {
    let keys = HashMap::new();
    assert!(find_destination_for_key('k', &keys).is_none());
}
