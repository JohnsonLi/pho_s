use eframe::egui::{self, TextEdit};

pub fn draw_bottom_panel(ctx: &egui::Context, app: &mut crate::app::Phos) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .max_height(400.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.heading("Destination Folders");
                    if ui.button("+").on_hover_text("Add destination folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            if !app.destination_paths.iter().any(|p| p == &path) {
                                app.destination_paths.push(path.clone());
                                app.image_destination_keys.insert(path, String::new());
                            }
                        }
                    }
                });
                ui.separator();

                let folders: Vec<_> = app.destination_paths.iter().cloned().collect();
                for folder in folders {
                    ui.horizontal(|ui| {
                        let display_name = folder
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| folder.display().to_string());
                        ui.label(display_name)
                            .on_hover_text(folder.display().to_string());

                        ui.separator();

                        if let Some(text) = app.image_destination_keys.get_mut(&folder) {
                            let response = ui.add(TextEdit::singleline(text)
                                .char_limit(1)
                                .desired_width(15.0));

                            if response.has_focus() {
                                app.keyboard_focused = true;
                                println!("TextEdit has focus");
                            } else if response.lost_focus() {
                                app.keyboard_focused = false;
                                println!("TextEdit lost focus");
                            }

                        }
                        
                        if ui.button("x").clicked() {
                            app.destination_paths.retain(|f| f != &folder);
                        }
                    });
                }
            }
        );
}