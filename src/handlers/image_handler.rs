use std::{fs::File, io::BufReader};

use eframe::egui::{self, Vec2};
use exif::Tag;
use image::GenericImageView;

pub struct LoadedImage {
    pub texture: egui::TextureHandle,
    pub size: [usize; 2],
    pub metadata: ImageMetadata,
}

impl LoadedImage {
    pub fn new(texture: egui::TextureHandle, size: [usize; 2], metadata: ImageMetadata) -> Self {
        LoadedImage { texture, size, metadata }
    }
}

pub fn load_image_at_path(ctx: &egui::Context, path: &str) -> Option<LoadedImage> {
    if let Ok(image) = image::open(path) {
        let metadata = extract_image_metadata(path).unwrap();

        let image = match metadata.orientation {
            Some(2) => image.fliph(),
            Some(3) => image.rotate180(),
            Some(4) => image.flipv(),
            Some(5) => image.rotate90().fliph(),
            Some(6) => image.rotate90(),
            Some(7) => image.rotate270().fliph(),
            Some(8) => image.rotate270(),
            _ => image
        };

        let (width, height) = image.dimensions();
        let rgba_image = image.to_rgba8();
        
        let texture_id = format!("img_{}", path.replace("\\", "_").replace("/", "_").replace(":", "_"));
        let texture = ctx.load_texture(
            &texture_id,
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &rgba_image
            ),
            egui::TextureOptions::default()
        );

        Some(LoadedImage::new(texture, [width as usize, height as usize], metadata))
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
    pub dimensions: (u32, u32),

    // exif
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub aperture: Option<String>,
    pub shutter_speed: Option<String>,
    pub iso: Option<String>,
    pub focal_length: Option<String>,

    pub lens_make: Option<String>,
    pub lens_model: Option<String>,
    pub orientation: Option<u32>
}

impl ImageMetadata {
    pub fn new() -> Self {
        ImageMetadata { 
            filename: String::new(),
            filesize: 0,
            dimensions: (0, 0),
            camera_make: None,
            camera_model: None,
            aperture: None,
            shutter_speed: None,
            iso: None,
            focal_length: None,
            lens_make: None,
            lens_model: None,
            orientation: None
        }
    }
}

pub fn extract_image_metadata(path: &str) -> Option<ImageMetadata> {
    let mut metadata = ImageMetadata::new();

    if let Some(filename) = std::path::Path::new(path).file_name() {
        metadata.filename = filename.to_string_lossy().to_string();
    }

    if let Ok(file_info) = std::fs::metadata(path) {
        metadata.filesize = file_info.len();
    }

    if let Ok(image) = image::open(path) {
        metadata.dimensions = image.dimensions();
    } 

    if let Ok(file) = File::open(path) {
        let mut buf_reader = BufReader::new(file);

        if let Ok(exif) = exif::Reader::new().read_from_container(&mut buf_reader) {
            for field in exif.fields() {
                let tag_value =  Some(field.display_value()
                                                        .with_unit(field)
                                                        .to_string()
                                                        .replace("\"", ""));
                
                match field.tag {
                    Tag::Make => {
                        metadata.camera_make = tag_value
                    },
                    Tag::Model => {
                        metadata.camera_model = tag_value
                    },
                    Tag::FNumber => {
                        metadata.aperture = tag_value
                    },
                    Tag::ExposureTime => {
                        metadata.shutter_speed = tag_value
                    },
                    Tag::PhotographicSensitivity => {
                        metadata.iso = tag_value
                    },
                    Tag::FocalLength => {
                        metadata.focal_length = tag_value
                    },
                    Tag::LensMake => {
                        metadata.lens_make = tag_value
                    },
                    Tag::LensModel => {
                        metadata.lens_model = tag_value
                    },
                    Tag::Orientation => {
                        metadata.orientation = field.value.get_uint(0)
                    },
                    _ => {
                        // ignore
                    }
                }
            }
        }
    } else {
        println!("Failed to open file");
        return None;
    }

    Some(metadata)
}