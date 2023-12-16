use std::sync::{Arc, Mutex};

use glam::{Mat4, Vec2};
use miniquad::*;

use crate::opengl::matrices::get_view_matrix;

use super::{matrices::get_ortho_matrix, pipelines::flat::FlatPipeline, structs::Object};

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,
    position: Vec2,
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

        flat_pipeline.update(ctx, &objects);

        FlatBlendState {
            flat_pipeline,
            projection_matrix,
            view_matrix,
            position,
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

        self.position.x = -(x / size.0 * 2.0 - 1.0);
        self.position.y = y / size.1 * 2.0 - 1.0;

        self.update_view_matrix();
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        let mut projection_matrix = self.projection_matrix.lock().unwrap();
        *projection_matrix = get_ortho_matrix(width, height);
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.flat_pipeline.draw(ctx);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}
