use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_position([640.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PhoS",
        native_options,
        Box::new(|cc| Ok(Box::new(phos::Phos::new(cc))))
    )
}