use eframe::egui::Vec2;
use phos::handlers::image_handler::scale_image_to_container;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.001
}

#[test]
fn image_larger_than_container_scales_down_preserving_aspect() {
    let image = Vec2::new(2000.0, 1000.0);
    let container = Vec2::new(1000.0, 1000.0);
    let result = scale_image_to_container(image, container);

    assert!(approx_eq(result.x, 1000.0), "x = {}", result.x);
    assert!(approx_eq(result.y, 500.0), "y = {}", result.y);
}

#[test]
fn image_smaller_than_container_scales_up() {
    let image = Vec2::new(100.0, 50.0);
    let container = Vec2::new(1000.0, 1000.0);
    let result = scale_image_to_container(image, container);

    assert!(approx_eq(result.x, 1000.0), "x = {}", result.x);
    assert!(approx_eq(result.y, 500.0), "y = {}", result.y);
}

#[test]
fn square_image_in_landscape_container_is_height_bound() {
    let image = Vec2::new(500.0, 500.0);
    let container = Vec2::new(1000.0, 500.0);
    let result = scale_image_to_container(image, container);

    assert!(approx_eq(result.x, 500.0));
    assert!(approx_eq(result.y, 500.0));
}

#[test]
fn tall_image_in_landscape_container_is_height_bound() {
    let image = Vec2::new(500.0, 2000.0);
    let container = Vec2::new(1000.0, 500.0);
    let result = scale_image_to_container(image, container);

    assert!(approx_eq(result.y, 500.0), "y should equal container height, got {}", result.y);
    assert!(approx_eq(result.x, 125.0), "x = {}", result.x);
}

#[test]
fn equal_dimensions_return_same_size() {
    let image = Vec2::new(800.0, 600.0);
    let result = scale_image_to_container(image, image);
    assert!(approx_eq(result.x, 800.0));
    assert!(approx_eq(result.y, 600.0));
}

#[test]
fn aspect_ratio_preserved_after_scaling() {
    let image = Vec2::new(1600.0, 900.0);
    let container = Vec2::new(800.0, 800.0);
    let result = scale_image_to_container(image, container);

    let original_ratio = image.x / image.y;
    let scaled_ratio = result.x / result.y;
    assert!(approx_eq(original_ratio, scaled_ratio), "ratios differ: {} vs {}", original_ratio, scaled_ratio);
}
