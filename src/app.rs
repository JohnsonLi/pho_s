use eframe::egui;

use crate::util::image_handler::{LoadedImage, load_image_path_into_panel};

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
                        // println!("open clicked");
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Some(image) = load_image_path_into_panel(ctx, path.to_str().unwrap()) {
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
        
        // egui::SidePanel::left("left_panel")
        //     .resizable(false)
        //     .default_width(120.0)
        //     .show(ctx, |ui| {
        //         ui.heading("left panel");
        //     });

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(240.0)
            .show(ctx, |ui| {
                ui.heading("right panel");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = &self.loaded_image {
                ui.image(&image.texture);
            } else {
                ui.heading("No image loaded");
            }
        });
    }
}