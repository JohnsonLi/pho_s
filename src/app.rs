use std::path::PathBuf;

use eframe::egui::{self, Pos2, Rect, Vec2};

use crate::util::image_handler::{load_image_at_path, scale_image_to_container, LoadedImage};

#[derive(Default)]
pub struct Phos {
    loaded_image: Option<LoadedImage>,
    zoom: f32,

    // panning
    pan: Vec2,
    prev_mouse_pos: Option<Pos2>,

    current_folder_path: PathBuf,
    current_folder_images:  Vec<PathBuf>
}

impl Phos {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            ..Default::default()
        }
    }
}

impl eframe::App for Phos {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save the app state here if needed
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Open", |ui| {
                    if ui.button("Open File").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Some(image) = load_image_at_path(ctx, path.to_str().unwrap()) {
                                self.loaded_image = Some(image);
                                self.zoom = 1.0;
                                self.pan = Vec2::ZERO;
                                self.prev_mouse_pos = None;
                                println!("loaded image");
                            } else {
                                println!("failed to load image");
                            }
                        }
                    }

                    if ui.button("Open Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let valid_formats = vec!["jpg", "jpeg", "png", "webp"];
                            self.current_folder_path = path.clone();
                            
                            let mut image_paths: Vec<PathBuf> = path.read_dir()
                                .unwrap()
                                .filter_map(|entry| {
                                    let path = entry.unwrap().path();
                                    if path.is_file() {
                                        if let Some(ext) = path.extension() {
                                            if valid_formats.contains(&ext.to_str().unwrap()) {
                                                return Some(path);
                                            }
                                        }
                                    }
                                    None
                                })
                                .collect();

                            image_paths.sort_by(|a, b| {
                                a.file_name().unwrap_or_default()
                                 .cmp(&b.file_name().unwrap_or_default())
                            });
                            
                            self.current_folder_images = image_paths;

                            println!("{:?}", self.current_folder_images);
                        }
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .show(ctx, |ui| {
                ui.heading("bottom panel")
            }
        );

        egui::SidePanel::right("right_panel")
            .min_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Image Information");
                ui.separator();

                let available_width = ui.available_width();
                let label_width = 100.0;
                let value_width = available_width - label_width;

                if let Some(image) = &self.loaded_image {
                    egui::Grid::new("metadata_table")
                        .striped(true)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            add_row(ui, "File Name", image.metadata.filename.clone(), value_width);
                            add_row(ui, "File Size", format_file_size(image.metadata.filesize), value_width);
                            add_row(ui, "Dimensions", format!("{} x {}", image.metadata.dimensions.0, image.metadata.dimensions.1), value_width);
                        });
                    

                    ui.separator();

                    egui::Grid::new("metadata_exif_table")
                        .striped(true)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            add_row(ui, "Camera Make", image.metadata.camera_make.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Camera Model", image.metadata.camera_model.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Aperture", image.metadata.aperture.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Shutter Speed", image.metadata.shutter_speed.clone().unwrap_or_default(), value_width);
                            add_row(ui, "ISO", image.metadata.iso.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Focal Length", image.metadata.focal_length.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Lens Make", image.metadata.lens_make.clone().unwrap_or_default(), value_width);
                            add_row(ui, "Lens Model", image.metadata.lens_model.clone().unwrap_or_default(), value_width);
                        });
                } 
            }
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_rect(ui.max_rect(), egui::Sense::click_and_drag());

            if response.hovered() {
                ctx.input(|i| {
                    if let Some(mouse_pos) = i.pointer.hover_pos() {
                        let old_zoom = self.zoom;

                        self.zoom *= 1.0 + i.smooth_scroll_delta.y * 0.003;
                        self.zoom = self.zoom.clamp(0.4, 10.0);
                        
                        let zoom_factor = self.zoom / old_zoom;
                        
                        let image_center = ui.max_rect().center() + self.pan;
                        let mouse_to_center = mouse_pos - image_center;
                        
                        let new_mouse_to_center = mouse_to_center * zoom_factor;
                        let pan_delta = new_mouse_to_center - mouse_to_center;
                        self.pan -= pan_delta;
                    }
                });
            }

            if response.dragged() {
                if let Some(current_pos) = response.hover_pos() {
                    if let Some(prev_pos) = self.prev_mouse_pos {
                        let delta = current_pos - prev_pos;
                        self.pan += delta;
                    }
                    self.prev_mouse_pos = Some(current_pos);
                }
            } else {
                self.prev_mouse_pos = None;
            }

            if let Some(image) = &self.loaded_image {
                let painter = ui.painter();

                let available_rect = ui.max_rect();
                let image_size = Vec2::new(image.size[0] as f32, image.size[1] as f32);
                let scaled_size = scale_image_to_container(image_size, available_rect.size());
                let zoomed_size = scaled_size * self.zoom;
                let top_left = available_rect.center() - zoomed_size / 2.0 + self.pan;

                painter.image(
                    image.texture.id(),
                    Rect::from_min_size(top_left, zoomed_size),
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    egui::Color32::WHITE
                );
            }
        });
    }

    
}

fn add_row(ui: &mut egui::Ui, label: &str, value: String, value_width: f32) {
    ui.label(label);

    // let rich_text = egui::RichText::new(value);
    ui.scope(|ui| {
        ui.set_max_width(value_width);
        ui.add(egui::Label::new(value)
            .truncate());
    });
    ui.end_row();
}

fn format_file_size(size_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size_bytes < KB {
        format!("{} bytes", size_bytes)
    } else if size_bytes < MB {
        format!("{:.2} KB", size_bytes as f64 / KB as f64)
    } else if size_bytes < GB {
        format!("{:.2} MB", size_bytes as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size_bytes as f64 / GB as f64)
    }
}