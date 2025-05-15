use std::path::PathBuf;

use eframe::egui::{self, Pos2, Vec2};

use crate::util::image_handler::LoadedImage;
use crate::views::folder_panel::draw_bottom_panel;
use crate::views::image_view::draw_image_view;
use crate::views::info_panel::draw_info_panel;
use crate::views::menu_bar::draw_menu_bar;
use crate::views::status_bar::draw_status_bar;

#[derive(Default)]
pub struct Phos {
    loaded_image: Option<LoadedImage>,
    zoom: f32,

    // panning
    pan: Vec2,
    prev_mouse_pos: Option<Pos2>,

    current_folder_path: PathBuf,
    current_folder_images:  Vec<PathBuf>
}

impl Phos {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            ..Default::default()
        }
    }
}

impl eframe::App for Phos {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save the app state here if needed
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_menu_bar(ctx, &mut self.loaded_image, &mut self.zoom, &mut self.pan, 
                        &mut self.prev_mouse_pos, &mut self.current_folder_path, &mut self.current_folder_images);
        draw_status_bar(ctx, &self.current_folder_images);
        draw_bottom_panel(ctx);
        draw_info_panel(ctx, &self.loaded_image);
        draw_image_view(ctx, &self.loaded_image, &mut self.zoom, &mut self.pan, &mut self.prev_mouse_pos);
    }
}