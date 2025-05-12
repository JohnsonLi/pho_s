use std::{fs::File, io::BufReader};

use eframe::egui::{self, Image, Vec2};
use image::{metadata, GenericImageView};

pub struct LoadedImage {
    pub texture: egui::TextureHandle,
    pub size: [usize; 2],
    pub path: String
}

impl LoadedImage {
    pub fn new(texture: egui::TextureHandle, size: [usize; 2], path: String) -> Self {
        LoadedImage { texture, size, path}
    }
}

pub fn load_image_at_path(ctx: &egui::Context, path: &str) -> Option<LoadedImage> {
    if let Ok(image) = image::open(path) {
        let (width, height) = image.dimensions();
        let rgba_image = image.to_rgba8();

        let texture = ctx.load_texture(
            "loaded_image",
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &rgba_image
            ),
            egui::TextureOptions::default()
        );

        Some(LoadedImage::new(texture, [width as usize, height as usize], path.to_string()))
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

pub struct ImageMetadata {
    pub filename: String,
    pub filesize: u64,
    pub format: String,
    pub dimensions: (u32, u32),

    // exif
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub aperture: Option<String>,
    pub shutter_speed: Option<String>,
    pub iso: Option<u32>,
    pub focal_length: Option<f32>,

    pub lens_make: Option<String>,
    pub lens_model: Option<String>
}

impl ImageMetadata {
    pub fn new() -> Self {
        ImageMetadata { 
            filename: String::new(),
            filesize: 0,
            format: String::new(),
            dimensions: (0, 0),
            camera_make: None,
            camera_model: None,
            aperture: None,
            shutter_speed: None,
            iso: None,
            focal_length: None,
            lens_make: None,
            lens_model: None 
        }
    }
}

pub fn extract_image_metadata(path: &str) -> Option<ImageMetadata> {
    let mut metadata = ImageMetadata::new();


    // let file = std::fs::File::open(path).ok()?;
    // let mut buf_reader = std::io::BufReader::new(&file);
    // let exif_reader = exif::Reader::new();
    // let exif = exif_reader.read_from_container(&mut buf_reader).ok()?;

    // metadata.camera_make = exif.fields()[0];

    // for f in exif.fields() {
    //     println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    // }

    Some(metadata)
}