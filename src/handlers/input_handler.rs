use eframe::egui::{self};

use crate::{app};

pub fn handle_keystrokes(ctx: &egui::Context, app: &mut app::Phos) {
    ctx.input(|i| {
        if i.key_pressed(egui::Key::ArrowRight) {
            if app.current_image_index < app.current_folder_images.len() - 1 && app.current_folder_images.len() > 0 {
                app.current_image_index += 1;
            } else {
                app.current_image_index = 0;
            }

            if app.current_folder_images.len() > 0 {
                app.current_image_path = Some(app.current_folder_images[app.current_image_index].clone());
                println!("Current image path: {:?}", app.current_image_path);
                app.image_loaded = false;
            } else {
                println!("No images in the folder or no folder selected.");
            }

            println!("Current image index: {}", app.current_image_index);
        }

        if i.key_pressed(egui::Key::ArrowLeft) {
            if app.current_image_index > 0 {
                app.current_image_index -= 1;
            } else {
                app.current_image_index = app.current_folder_images.len() - 1;
            }

            if app.current_folder_images.len() > 0 {
                app.current_image_path = Some(app.current_folder_images[app.current_image_index].clone());
                println!("Current image path: {:?}", app.current_image_path);
                app.image_loaded = false;
            } else {
                println!("No images in the folder or no folder selected.");
            }

            println!("Current image index: {}", app.current_image_index);
        }

        if i.key_pressed(egui::Key::Space) {
            app.reset_view();
        }
    });    
}