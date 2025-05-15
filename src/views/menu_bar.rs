use std::path::PathBuf;

use eframe::egui::{self, Pos2, Vec2};

use crate::util::image_handler::{load_image_at_path, LoadedImage};

pub fn draw_menu_bar(ctx: &egui::Context,
                        loaded_image: &mut Option<LoadedImage>,
                        zoom: &mut f32,
                        pan: &mut Vec2,
                        prev_mouse_pos: &mut Option<Pos2>,
                        current_folder_path: &mut PathBuf,
                        current_folder_images: &mut Vec<PathBuf>) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Open", |ui| {
                if ui.button("Open File").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        if let Some(image) = load_image_at_path(ctx, path.to_str().unwrap()) {
                            *loaded_image = Some(image);
                            *zoom = 1.0;
                            *pan = Vec2::ZERO;
                            *prev_mouse_pos = None;
                            *current_folder_path = PathBuf::new();
                            *current_folder_images = vec![];
                            println!("loaded image");
                        } else {
                            println!("failed to load image");
                        }
                    }
                    ui.close_menu();
                }

                if ui.button("Open Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let valid_formats = vec!["jpg", "jpeg", "png", "webp"];
                        *current_folder_path = path.clone();
                        
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
                        
                        *current_folder_images = image_paths;

                        println!("{:?}", *current_folder_images);
                    }
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });
}