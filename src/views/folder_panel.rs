use eframe::egui::{self, TextEdit};

pub fn draw_bottom_panel(ctx: &egui::Context, app: &mut crate::app::Phos) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .show(ctx, |ui| {
                ui.heading("Destination Folders");
                ui.separator();

                let folders: Vec<_> = app.destination_paths.iter().cloned().collect();
                for folder in folders {
                    ui.horizontal(|ui| {
                        ui.label(folder.display().to_string());

                        ui.separator();

                        if let Some(text) = app.image_destination_keys.get_mut(&folder) {
                            // restrixt to 1 character
                            ui.add(TextEdit::singleline(text)
                                .char_limit(1)
                                .desired_width(15.0));
                        }
                        
                        if ui.button("x").clicked() {
                            app.destination_paths.retain(|f| f != &folder);
                        }
                    });
                }
            }
        );
}