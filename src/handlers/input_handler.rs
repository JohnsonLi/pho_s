use eframe::egui::{self};

use crate::app;

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

        for event in &i.raw.events {
            if let egui::Event::Text(c) = event {
                for (path, key_string) in &app.image_destination_keys {
                    let key_char = c.chars().next();
                    if !key_string.is_empty() && key_string.chars().next() == key_char {
                        println!("Destination key '{:?}' pressed for path: {:?}", key_char, path);

                        if let Some(current_path) = &app.current_image_path {
                            println!("Moving/copying {:?} to {:?}", current_path, path);
                            let result = crate::controllers::file_manager::move_file(current_path, path);

                            if result.is_ok() {
                                if !app.current_folder_images.is_empty() {
                                    app.current_folder_images.remove(app.current_image_index);
                                }

                                if !app.current_folder_images.is_empty() {
                                    app.current_image_path = Some(app.current_folder_images[app.current_image_index].clone());
                                } else {
                                    app.loaded_image = None;
                                    app.current_image_path = None;
                                    app.current_image_index = 0;
                                }

                                app.image_loaded = false;
                            } else {
                                println!("Failed to move file: {:?}", result.err());
                            }
                        }

                        break;
                    }
                }
            }
        }
    });    
}