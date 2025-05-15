use eframe::egui::{self, Pos2, Rect, Vec2};

use crate::util::image_handler::{scale_image_to_container, LoadedImage};

pub fn draw_image_view(ctx: &egui::Context,
                        loaded_image: &Option<LoadedImage>,
                        zoom: &mut f32,
                        pan: &mut Vec2,
                        prev_mouse_pos: &mut Option<Pos2>) {

    egui::CentralPanel::default().show(ctx, |ui| {
        let response = ui.allocate_rect(ui.max_rect(), egui::Sense::click_and_drag());

        if response.hovered() {
            ctx.input(|i| {
                if let Some(mouse_pos) = i.pointer.hover_pos() {
                    let old_zoom = *zoom;

                    *zoom *= 1.0 + i.smooth_scroll_delta.y * 0.003;
                    *zoom = zoom.clamp(0.4, 10.0);
                    
                    let zoom_factor = *zoom / old_zoom;
                    
                    let image_center = ui.max_rect().center() + *pan;
                    let mouse_to_center = mouse_pos - image_center;
                    
                    let new_mouse_to_center = mouse_to_center * zoom_factor;
                    let pan_delta = new_mouse_to_center - mouse_to_center;
                    *pan -= pan_delta;
                }
            });
        }

        if response.dragged() {
            if let Some(current_pos) = response.hover_pos() {
                if let Some(prev_pos) = *prev_mouse_pos {
                    let delta = current_pos - prev_pos;
                    *pan += delta;
                }
                *prev_mouse_pos = Some(current_pos);
            }
        } else {
            *prev_mouse_pos = None;
        }

        if let Some(image) = loaded_image {
            let painter = ui.painter();

            let available_rect = ui.max_rect();
            let image_size = Vec2::new(image.size[0] as f32, image.size[1] as f32);
            let scaled_size = scale_image_to_container(image_size, available_rect.size());
            let zoomed_size = scaled_size * *zoom;
            let top_left = available_rect.center() - zoomed_size / 2.0 + *pan;

            painter.image(
                image.texture.id(),
                Rect::from_min_size(top_left, zoomed_size),
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE
            );
        }
    });
}