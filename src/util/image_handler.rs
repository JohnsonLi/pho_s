use eframe::egui::{self, Vec2};
use image::GenericImageView;

pub struct LoadedImage {
    pub texture: egui::TextureHandle,
    pub size: [usize; 2]
}

impl LoadedImage {
    pub fn new(texture: egui::TextureHandle, size: [usize; 2]) -> Self {
        LoadedImage { texture, size }
    }
}

pub fn load_image_at_path(ctx: &egui::Context, path: &str) -> Option<LoadedImage> {
    if let Ok(image) = image::open(path) {
        let resized_image = image.resize(600, 600, image::imageops::FilterType::Nearest);
        let (width, height) = resized_image.dimensions();
        let rgba_image = resized_image.to_rgba8();

        let texture = ctx.load_texture(
            "loaded_image",
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &rgba_image
            ),
            egui::TextureOptions::default()
        );

        Some(LoadedImage::new(texture, [width as usize, height as usize]))
    } else {
        println!("Failed to load image");
        None
    }
}

pub fn scale_image_to_container(image_size: Vec2, container_size: Vec2) -> Vec2 {
    let scale_x = container_size.x / image_size.x;
    let scale_y = container_size.y / image_size.y;
    let scale = scale_x.min(scale_y);

    Vec2::new(image_size.x * scale, image_size.y * scale)
}