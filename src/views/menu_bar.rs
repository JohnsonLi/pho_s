use std::path::{Path, PathBuf};

use eframe::egui::{self};

use crate::{app, views::theme::toggle_theme};

pub const VALID_IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn collect_image_paths_sorted(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut image_paths: Vec<PathBuf> = dir
        .read_dir()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if !path.is_file() {
                return None;
            }
            let ext = path.extension()?.to_str()?.to_lowercase();
            if VALID_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    image_paths.sort_by(|a, b| {
        a.file_name().unwrap_or_default()
            .cmp(&b.file_name().unwrap_or_default())
    });

    Ok(image_paths)
}

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
                        let image_paths = collect_image_paths_sorted(&path).unwrap_or_default();

                        app.current_folder_images = image_paths;
                        app.current_image_index = 0;

                        if let Some(first_image) = app.current_folder_images.get(0) {
                            app.current_image_path = Some(first_image.clone());
                        } else {
                            app.current_image_path = None;
                        }

                        app.image_loaded = false;

                        ui.close_menu();
                    }
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let icon = if ctx.theme() == egui::Theme::Dark { "\u{2600}" } else { "🌙" };
                if ui.button(icon).on_hover_text("Toggle light / dark").clicked() {
                    toggle_theme(ctx);
                }
            });
        });
    });
}
