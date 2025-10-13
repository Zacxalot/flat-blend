use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use egui_miniquad as egui_mq;
use glam::{Mat4, Vec2};
use miniquad::*;

use crate::opengl::matrices::{get_view_matrix, screen_to_world};
use crate::ui::objects::ObjectsUI;
use crate::ui::viewport::ViewportUI;

use super::{
    matrices::get_ortho_matrix,
    render_context::RenderContext,
    structs::{Mesh, Object},
};

pub struct FlatBlendState {
    render_context: RenderContext,
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,
    zoom: Arc<Mutex<f32>>,
    position: Arc<Mutex<Vec2>>,
    mouse_state: HashMap<MouseButton, bool>,
    last_mouse_position: Vec2,
    egui_mq: egui_mq::EguiMq,
}

impl FlatBlendState {
    pub fn new(
        ctx: &mut Context,
        objects: Vec<Object>,
        meshes: Vec<Rc<RefCell<Mesh>>>,
    ) -> FlatBlendState {
        ctx.set_cull_face(CullFace::Nothing);
        let zoom = Arc::new(Mutex::new(1.0));

        let (width, height) = ctx.screen_size();
        let position = Arc::new(Mutex::new(Vec2::new(0.0, 0.0)));

        let projection_matrix = Arc::new(Mutex::new(get_ortho_matrix(width, height)));
        let view_matrix = Arc::new(Mutex::new(get_view_matrix(
            *(position.lock().unwrap()),
            *(zoom.lock().unwrap()),
        )));

        let render_context = RenderContext::new(
            ctx,
            projection_matrix.clone(),
            view_matrix.clone(),
            zoom.clone(),
            position.clone(),
            objects,
            meshes,
        );

        FlatBlendState {
            render_context,
            projection_matrix,
            view_matrix,
            position,
            mouse_state: HashMap::new(),
            last_mouse_position: Vec2::new(0.0, 0.0),
            egui_mq: egui_mq::EguiMq::new(ctx),
            zoom,
        }
    }

    pub fn update_view_matrix(&mut self) {
        let mut view_matrix = self.view_matrix.lock().unwrap();
        *view_matrix = get_view_matrix(
            *(self.position.lock().unwrap()),
            *(self.zoom.lock().unwrap()),
        );
        drop(view_matrix);

        // Update scene visibility when camera changes
        self.render_context.update_visibility();
    }

    /// Select object at the given world position
    fn select_object_at(&mut self, world_pos: Vec2, add_to_selection: bool) {
        let objects = self.render_context.scene_data.objects_mut();

        // If not adding to selection, clear previous selection
        if !add_to_selection {
            for obj in objects.iter_mut() {
                obj.selected = false;
            }
        }

        // Find and select the topmost object at this position
        // Iterate in reverse to get the top-most object first
        for obj in objects.iter_mut().rev() {
            if obj.contains_point(world_pos) {
                obj.selected = !obj.selected; // Toggle if adding to selection
                if !add_to_selection {
                    break; // Only select one if not multi-selecting
                }
            }
        }
    }
}

impl EventHandler for FlatBlendState {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);

        let mouse_position = Vec2::new(x, y);

        if let Some(middle_click) = self.mouse_state.get(&MouseButton::Middle) {
            if *middle_click {
                let diff = mouse_position - self.last_mouse_position;

                {
                    let mut position = self.position.lock().unwrap();
                    position.x += diff.x;
                    position.y -= diff.y;
                }

                self.update_view_matrix();
            }
        }

        self.last_mouse_position = mouse_position;
    }

    fn mouse_wheel_event(&mut self, _: &mut Context, dx: f32, dy: f32) {
        if dy != 0.0 {
            let mut zoom = self.zoom.lock().unwrap();
            *zoom = (*zoom + dy / 1000.0).max(0.1).min(20.0);
            drop(zoom);
            self.update_view_matrix();
        }

        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(ctx, button, x, y);

        // Handle left-click selection (only if egui doesn't want the input)
        if button == MouseButton::Left && !self.egui_mq.egui_ctx().wants_pointer_input() {
            let screen_size = ctx.screen_size();
            let world_pos = screen_to_world(
                Vec2::new(x, y),
                screen_size,
                *self.view_matrix.lock().unwrap(),
                *self.projection_matrix.lock().unwrap(),
            );

            // Check if Shift is held for multi-selection
            // Note: miniquad doesn't provide modifier state in mouse events,
            // so we'll default to single selection for now
            self.select_object_at(world_pos, false);
        }

        self.mouse_state.insert(button, true);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(ctx, button, x, y);
        self.mouse_state.insert(button, false);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        let mut projection_matrix = self.projection_matrix.lock().unwrap();
        *projection_matrix = get_ortho_matrix(width, height);
        drop(projection_matrix);

        // Update scene visibility when projection changes
        self.render_context.update_visibility();
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.render_context.draw(ctx);

        ctx.end_render_pass();

        let position = *(self.position.lock().unwrap());
        let zoom = *(self.zoom.lock().unwrap());

        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            ObjectsUI::ui(egui_ctx);
            ViewportUI::ui(egui_ctx, position, zoom);
        });

        self.egui_mq.draw(ctx);

        ctx.commit_frame();
    }
}
