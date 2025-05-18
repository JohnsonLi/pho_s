use eframe::egui::{self, TextEdit};

pub fn draw_bottom_panel(ctx: &egui::Context, app: &mut crate::app::Phos) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .show(ctx, |ui| {
                ui.heading("bottom panel");

                let folders: Vec<_> = app.destination_paths.iter().cloned().collect();
                for folder in folders {
                    ui.horizontal(|ui| {
                        ui.label(folder.display().to_string());

                        ui.separator();

                        if ui.button("+").clicked() {
                            app.show_hotkey_input = true;
                        }

                        if app.show_hotkey_input {
                            ui.add(TextEdit::singleline(&mut "asdasda"));
                            // ui.label(format!("You typed: {}", self.user_input));

                            // Optional: button to "submit" or hide input box
                            if ui.button("Done").clicked() {
                                app.show_hotkey_input = false;
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