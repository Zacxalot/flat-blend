use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use egui_miniquad as egui_mq;
use glam::{Mat4, Vec2};
use miniquad::*;

use crate::opengl::matrices::get_view_matrix;
use crate::ui::objects::ObjectsUI;
use crate::ui::viewport::ViewportUI;

use super::{
    matrices::get_ortho_matrix,
    pipelines::{flat::FlatPipeline, grid::GridPipeline},
    structs::{Mesh, Object},
};

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
    grid_pipeline: GridPipeline,
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

        let mut flat_pipeline =
            FlatPipeline::new(ctx, projection_matrix.clone(), view_matrix.clone());
        let grid_pipeline = GridPipeline::new(ctx, position.clone(), zoom.clone());

        flat_pipeline.update(ctx, objects, meshes);

        FlatBlendState {
            flat_pipeline,
            grid_pipeline,
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
    }
}

impl EventHandler for FlatBlendState {
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);

        let size = ctx.screen_size();
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
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.grid_pipeline.draw(ctx);
        self.flat_pipeline.draw(ctx);

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
