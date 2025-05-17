use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self, Pos2, Vec2};

use crate::handlers::input_handler::handle_keystrokes;
use crate::util::image_handler::LoadedImage;
use crate::views::folder_panel::draw_bottom_panel;
use crate::views::image_view::draw_image_view;
use crate::views::info_panel::draw_info_panel;
use crate::views::menu_bar::draw_menu_bar;
use crate::views::status_bar::draw_status_bar;

#[derive(Default)]
pub struct Phos {
    pub loaded_image: Option<LoadedImage>,

    pub zoom: f32,
    pub pan: Vec2,
    pub prev_mouse_pos: Option<Pos2>,

    pub current_folder_images:  Vec<PathBuf>,
    pub current_image_index: usize,
    pub current_image_path: Option<PathBuf>,

    pub image_loaded: bool,

    pub image_destination_keys: HashMap<String, PathBuf>
}

impl Phos {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            current_image_index: 0,
            loaded_image: None,
            ..Default::default()
        }
    }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan = Vec2::ZERO;
        self.prev_mouse_pos = None;
    }
}

impl eframe::App for Phos {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save the app state here if needed
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_menu_bar(ctx, self);
        draw_status_bar(ctx, self);
        draw_bottom_panel(ctx, self);
        draw_info_panel(ctx, self);
        draw_image_view(ctx, self);

        handle_keystrokes(ctx, self);
    }
}