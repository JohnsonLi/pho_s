use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self};

use crate::app;

pub fn next_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if current < len - 1 {
        current + 1
    } else {
        0
    }
}

pub fn prev_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if current > 0 {
        current - 1
    } else {
        len - 1
    }
}

pub fn find_destination_for_key<'a>(
    key: char,
    keys: &'a HashMap<PathBuf, String>,
) -> Option<&'a PathBuf> {
    for (path, key_string) in keys {
        if key_string.is_empty() {
            continue;
        }
        if key_string.chars().next() == Some(key) {
            return Some(path);
        }
    }
    None
}

pub fn handle_keystrokes(ctx: &egui::Context, app: &mut app::Phos) {
    ctx.input(|i| {
        if i.key_pressed(egui::Key::ArrowRight) {
            let prev = app.current_image_index;
            app.current_image_index = next_index(app.current_image_index, app.current_folder_images.len());

            if app.current_folder_images.len() > 0 {
                app.current_image_path = Some(app.current_folder_images[app.current_image_index].clone());
                println!("Current image path: {:?}", app.current_image_path);
                if app.current_image_index != prev {
                    app.image_loaded = false;
                }
            } else {
                println!("No images in the folder or no folder selected.");
            }

            println!("Current image index: {}", app.current_image_index);
        }

        if i.key_pressed(egui::Key::ArrowLeft) {
            let prev = app.current_image_index;
            app.current_image_index = prev_index(app.current_image_index, app.current_folder_images.len());

            if app.current_folder_images.len() > 0 {
                app.current_image_path = Some(app.current_folder_images[app.current_image_index].clone());
                println!("Current image path: {:?}", app.current_image_path);
                if app.current_image_index != prev {
                    app.image_loaded = false;
                }
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
                if let Some(key_char) = c.chars().next() {
                    if let Some(path) = find_destination_for_key(key_char, &app.image_destination_keys) {
                        let path = path.clone();
                        println!("Destination key '{:?}' pressed for path: {:?}", key_char, path);

                        if let Some(current_path) = &app.current_image_path {
                            println!("Moving/copying {:?} to {:?}", current_path, path);
                            let result = crate::controllers::file_manager::move_file(current_path, &path);

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
                    }
                }
            }
        }
    });
}
