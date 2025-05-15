use eframe::egui;

use crate::app;

pub fn draw_status_bar(ctx: &egui::Context, app: &mut app::Phos) {
    egui::TopBottomPanel::bottom("status_bar")
        .resizable(false)
        .max_height(30.0)
        .show(ctx, |ui| {
            ui.heading(format!("{} / {}", app.current_image_index + 1, app.current_folder_images.len()));
        }
    );
}