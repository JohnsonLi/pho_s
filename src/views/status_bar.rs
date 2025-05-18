use eframe::egui;

use crate::app;

pub fn draw_status_bar(ctx: &egui::Context, app: &mut app::Phos) {
    egui::TopBottomPanel::bottom("status_bar")
        .resizable(false)
        .max_height(30.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if app.current_folder_images.is_empty() {
                    ui.heading(format!("{} / {}", app.current_image_index, app.current_folder_images.len()));
                } else {
                    ui.heading(format!("{} / {}", app.current_image_index + 1, app.current_folder_images.len()));
                }

                ui.add_space(ui.available_width() - 36.0);
                
                ui.separator();

                // Right side: plus button
                let plus_button = ui.add(
                    egui::Button::new("+")
                        .min_size(egui::vec2(20.0, 20.0))
                );
                
                if plus_button.clicked() {
                    println!("Add destination folder button clicked");
                    
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        println!("Added destination folder: {}", path.display());
                        app.destination_paths.push(path);
                    }

                }
                
                // Optional: Add tooltip
                plus_button.on_hover_text("Add destination folder");
            });
        }
    );

    
}