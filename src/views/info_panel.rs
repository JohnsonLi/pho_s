use eframe::egui;

use crate::{app};

pub fn draw_info_panel(ctx: &egui::Context, app: &mut app::Phos) {
    egui::SidePanel::right("right_panel")
        .min_width(280.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Image Information");
            ui.separator();

            let available_width = ui.available_width();
            let label_width = 100.0;
            let value_width = available_width - label_width;

            if let Some(image) = &app.loaded_image {
                egui::Grid::new("metadata_table")
                    .striped(true)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        add_row(ui, "File Name", image.metadata.filename.clone(), value_width);
                        add_row(ui, "File Size", format_file_size(image.metadata.filesize), value_width);
                        add_row(ui, "Dimensions", format!("{} x {}", image.metadata.dimensions.0, image.metadata.dimensions.1), value_width);
                    });
                

                ui.separator();

                egui::Grid::new("metadata_exif_table")
                    .striped(true)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        add_row(ui, "Camera Make", image.metadata.camera_make.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Camera Model", image.metadata.camera_model.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Aperture", image.metadata.aperture.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Shutter Speed", image.metadata.shutter_speed.clone().unwrap_or_default(), value_width);
                        add_row(ui, "ISO", image.metadata.iso.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Focal Length", image.metadata.focal_length.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Lens Make", image.metadata.lens_make.clone().unwrap_or_default(), value_width);
                        add_row(ui, "Lens Model", image.metadata.lens_model.clone().unwrap_or_default(), value_width);
                    });
            } 
        }
    );
}

fn add_row(ui: &mut egui::Ui, label: &str, value: String, value_width: f32) {
    ui.label(label);

    ui.scope(|ui| {
        ui.set_max_width(value_width);
        ui.add(egui::Label::new(value)
            .truncate());
    });
    ui.end_row();
}

fn format_file_size(size_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size_bytes < KB {
        format!("{} bytes", size_bytes)
    } else if size_bytes < MB {
        format!("{:.2} KB", size_bytes as f64 / KB as f64)
    } else if size_bytes < GB {
        format!("{:.2} MB", size_bytes as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size_bytes as f64 / GB as f64)
    }
}