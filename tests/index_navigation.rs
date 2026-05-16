use phos::handlers::input_handler::{next_index, prev_index};

#[test]
fn next_index_advances_within_bounds() {
    assert_eq!(next_index(0, 3), 1);
    assert_eq!(next_index(1, 3), 2);
}

#[test]
fn next_index_wraps_to_zero_at_end() {
    assert_eq!(next_index(2, 3), 0);
}

#[test]
fn prev_index_decrements_within_bounds() {
    assert_eq!(prev_index(2, 3), 1);
    assert_eq!(prev_index(1, 3), 0);
}

#[test]
fn prev_index_wraps_to_last_at_start() {
    assert_eq!(prev_index(0, 3), 2);
}

#[test]
fn both_handle_empty_list_without_panic() {
    assert_eq!(next_index(0, 0), 0);
    assert_eq!(prev_index(0, 0), 0);
}

#[test]
fn both_handle_single_element_list() {
    assert_eq!(next_index(0, 1), 0);
    assert_eq!(prev_index(0, 1), 0);
}
