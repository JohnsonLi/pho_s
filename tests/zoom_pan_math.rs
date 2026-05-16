use eframe::egui::Vec2;
use phos::views::image_view::{apply_zoom_delta, pan_toward_cursor, ZOOM_MAX, ZOOM_MIN};

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.001
}

#[test]
fn positive_scroll_grows_zoom() {
    let new_zoom = apply_zoom_delta(1.0, 100.0);
    assert!(new_zoom > 1.0, "expected > 1.0, got {}", new_zoom);
}

#[test]
fn negative_scroll_shrinks_zoom() {
    let new_zoom = apply_zoom_delta(1.0, -100.0);
    assert!(new_zoom < 1.0, "expected < 1.0, got {}", new_zoom);
}

#[test]
fn zero_scroll_returns_input_zoom() {
    assert!(approx_eq(apply_zoom_delta(1.5, 0.0), 1.5));
    assert!(approx_eq(apply_zoom_delta(0.7, 0.0), 0.7));
}

#[test]
fn zoom_is_clamped_at_upper_bound() {
    let new_zoom = apply_zoom_delta(9.0, 1_000_000.0);
    assert!(approx_eq(new_zoom, ZOOM_MAX), "expected clamp to {}, got {}", ZOOM_MAX, new_zoom);
}

#[test]
fn zoom_is_clamped_at_lower_bound() {
    let new_zoom = apply_zoom_delta(0.5, -1_000_000.0);
    assert!(approx_eq(new_zoom, ZOOM_MIN), "expected clamp to {}, got {}", ZOOM_MIN, new_zoom);
}

#[test]
fn pan_unchanged_when_zoom_does_not_change() {
    let pan = Vec2::new(10.0, 20.0);
    let mouse_to_center = Vec2::new(50.0, 30.0);
    let result = pan_toward_cursor(2.0, 2.0, mouse_to_center, pan);
    assert!(approx_eq(result.x, pan.x), "x: {} vs {}", result.x, pan.x);
    assert!(approx_eq(result.y, pan.y), "y: {} vs {}", result.y, pan.y);
}

#[test]
fn pan_shifts_away_from_cursor_when_zooming_in() {
    let pan = Vec2::new(0.0, 0.0);
    let mouse_to_center = Vec2::new(100.0, 0.0);
    let result = pan_toward_cursor(1.0, 2.0, mouse_to_center, pan);
    assert!(approx_eq(result.x, -100.0), "x = {}", result.x);
    assert!(approx_eq(result.y, 0.0));
}

#[test]
fn pan_shifts_toward_cursor_when_zooming_out() {
    let pan = Vec2::new(0.0, 0.0);
    let mouse_to_center = Vec2::new(100.0, 0.0);
    let result = pan_toward_cursor(2.0, 1.0, mouse_to_center, pan);
    assert!(approx_eq(result.x, 50.0), "x = {}", result.x);
}
