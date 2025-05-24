use eframe::egui::{ViewportBuilder, IconData};

fn main() -> eframe::Result {
    let image = image::open("icon.png").expect("Failed to load icon");

    let icon = IconData {
        rgba: image.into_rgba8().into_raw(),
        width: 1024,
        height: 1024,
    };

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_icon(std::sync::Arc::new(icon)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "PhoS",
        native_options,
        Box::new(|cc| Ok(Box::new(phos::Phos::new(cc))))
    )
}