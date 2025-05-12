use eframe::egui::{self, Pos2, Rect, Vec2};

use crate::util::image_handler::{LoadedImage, load_image_at_path, scale_image_to_container};

#[derive(Default)]
pub struct Phos {
    loaded_image: Option<LoadedImage>
}

impl Phos {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Phos {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save the app state here if needed
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Open", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Some(image) = load_image_at_path(ctx, path.to_str().unwrap()) {
                                self.loaded_image = Some(image);
                                println!("loaded image");
                            } else {
                                println!("failed to load image");
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(140.0)
            .show(ctx, |ui| {
                ui.heading("bottom panel")
            }
        );

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(240.0)
            .show(ctx, |ui| {
                ui.heading("right panel");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = &self.loaded_image {
                let painter = ui.painter();

                let available_rect = ui.max_rect();
                let image_size = Vec2::new(image.size[0] as f32, image.size[1] as f32);
                let scaled_size = scale_image_to_container(image_size, available_rect.size());
                let top_left = available_rect.center() - scaled_size / 2.0;

                painter.image(
                    image.texture.id(),
                    Rect::from_min_size(top_left, scaled_size),
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    egui::Color32::WHITE
                );
            }
        });
    }
}