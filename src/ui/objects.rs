use egui::Context;

pub struct ObjectsUI {}

impl ObjectsUI {
    pub fn new() -> ObjectsUI {
        ObjectsUI {}
    }

    pub fn ui(egui_ctx: &Context) {
        egui::Window::new("Objects").show(egui_ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| ui.label("Howdy"));
        });
    }
}
