use std::sync::Arc;

use egui::Context;

use crate::opengl::structs::Object;

pub struct ObjectsUI {
    objects: Arc<Vec<Object>>,
}

impl ObjectsUI {
    pub fn new(objects: Arc<Vec<Object>>) -> ObjectsUI {
        ObjectsUI { objects }
    }

    pub fn draw(&self, egui_ctx: &Context) {
        egui::Window::new("Objects").show(egui_ctx, |ui| {
            // Scrollable list of object names
            let row_height_sans_spacing = ui.spacing().item_spacing.y;
            let total_rows = self.objects.len();
            egui::ScrollArea::both().auto_shrink([false; 2]).show_rows(
                ui,
                row_height_sans_spacing,
                total_rows,
                |ui, row_range| {
                    for row in row_range {
                        let object = &self.objects[row];
                        ui.label(object.name.clone());
                    }
                },
            );
        });
    }
}
