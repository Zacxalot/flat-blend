use egui::Context;
use glam::Vec2;

pub struct ViewportUI {}

impl ViewportUI {
    pub fn new() -> ViewportUI {
        ViewportUI {}
    }

    pub fn ui(egui_ctx: &Context, position: Vec2, zoom: f32) {
        egui::Window::new("Viewport Info").show(egui_ctx, |ui| {
            ui.label(format!("Position: ({:.2}, {:.2})", position.x, position.y));
            ui.label(format!("Zoom: {:.2}", zoom));
        });
    }
}
