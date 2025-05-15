use eframe::egui;


pub fn handle_keystrokes(ctx: &egui::Context) {
    ctx.input(|i| {
        if i.key_pressed(egui::Key::ArrowRight) {
            println!("Right arrow key pressed");
        }

        if i.key_pressed(egui::Key::ArrowLeft) {
            println!("Left arrow key pressed");
        }

        if i.key_pressed(egui::Key::Space) {
            println!("Up arrow key pressed");
        }
    });    
}