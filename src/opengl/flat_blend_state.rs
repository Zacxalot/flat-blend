use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use glam::{Mat4, Vec2};
use miniquad::*;

use crate::opengl::matrices::get_view_matrix;

use super::{
    matrices::get_ortho_matrix,
    pipelines::{
        flat::FlatPipeline,
        grid::{self, GridPipeline},
    },
    structs::Object,
};

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
    grid_pipeline: GridPipeline,
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,
    position: Vec2,
    mouse_state: HashMap<MouseButton, bool>,
    last_mouse_position: Vec2,
}

impl FlatBlendState {
    pub fn new(ctx: &mut Context, objects: Vec<Object>) -> FlatBlendState {
        ctx.set_cull_face(CullFace::Nothing);

        let (width, height) = ctx.screen_size();
        let position = Vec2::new(0.0, 0.0);

        let projection_matrix = Arc::new(Mutex::new(get_ortho_matrix(width, height)));
        let view_matrix = Arc::new(Mutex::new(get_view_matrix(position)));

        let mut flat_pipeline =
            FlatPipeline::new(ctx, projection_matrix.clone(), view_matrix.clone());
        let grid_pipeline = GridPipeline::new(ctx);

        flat_pipeline.update(ctx, &objects);

        FlatBlendState {
            flat_pipeline,
            grid_pipeline,
            projection_matrix,
            view_matrix,
            position,
            mouse_state: HashMap::new(),
            last_mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update_view_matrix(&mut self) {
        let mut view_matrix = self.view_matrix.lock().unwrap();
        *view_matrix = get_view_matrix(self.position);
    }
}

impl EventHandler for FlatBlendState {
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let size = ctx.screen_size();
        let mouse_position = Vec2::new(x, y);

        if let Some(middle_click) = self.mouse_state.get(&MouseButton::Middle) {
            if *middle_click {
                let diff = mouse_position - self.last_mouse_position;

                self.position.x += diff.x / (size.0 / 2.0);
                self.position.y -= diff.y / (size.1 / 2.0);

                self.update_view_matrix();
            }
        }

        self.last_mouse_position = mouse_position;
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse_state.insert(button, true);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.mouse_state.insert(button, false);
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

        ctx.commit_frame();
    }
}
