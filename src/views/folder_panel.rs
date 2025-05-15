use eframe::egui;

pub fn draw_bottom_panel(ctx: &egui::Context, _app: &mut crate::app::Phos) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .show(ctx, |ui| {
                ui.heading("bottom panel")
            }
        );
}