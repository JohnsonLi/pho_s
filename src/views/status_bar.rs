use std::path::PathBuf;

use eframe::egui;

pub fn draw_status_bar(ctx: &egui::Context, folders: &Vec<PathBuf>) {
    egui::TopBottomPanel::bottom("status_bar")
        .resizable(false)
        .max_height(30.0)
        .show(ctx, |ui| {
            ui.heading("status bar")
        }
    );
}