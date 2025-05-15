use std::path::PathBuf;

use eframe::egui::{self};

use crate::{app};

pub fn draw_menu_bar(ctx: &egui::Context, app: &mut app::Phos) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Open", |ui| {
                if ui.button("Open File").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        app.current_image_path = Some(path.clone());
                        app.image_loaded = false;
                    }
                    ui.close_menu();
                }

                if ui.button("Open Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let valid_formats = vec!["jpg", "jpeg", "png", "webp", "arw"];
                        
                        let mut image_paths: Vec<PathBuf> = path.read_dir()
                            .unwrap()
                            .filter_map(|entry| {
                                let path = entry.unwrap().path();
                                if path.is_file() {
                                    if let Some(ext) = path.extension() {
                                        if valid_formats.contains(&ext.to_str().unwrap().to_lowercase().as_str()) {
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
                        
                        app.current_folder_images = image_paths;
                        app.current_image_index = 0;
                        app.current_image_path = Some(app.current_folder_images[0].clone());
                        app.image_loaded = false;

                        println!("{:?}", app.current_folder_images);
                    }
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });
}