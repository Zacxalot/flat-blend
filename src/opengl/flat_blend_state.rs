use std::sync::{Arc, Mutex};

use glam::Mat4;
use miniquad::*;

use super::{pipelines::flat::FlatPipeline, structs::Object};

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
    projection_matrix: Arc<Mutex<Mat4>>,
}

impl FlatBlendState {
    pub fn new(ctx: &mut Context, objects: Vec<Object>) -> FlatBlendState {
        ctx.set_cull_face(CullFace::Nothing);

        let left = -2.0;
        let right = 2.0;
        let bottom = 2.0;
        let top = -2.0;
        let near = -1.0;
        let far = 1.0;
        let projection_matrix = Arc::new(Mutex::new(glam::Mat4::orthographic_rh_gl(
            left, right, bottom, top, near, far,
        )));

        let mut flat_pipeline = FlatPipeline::new(ctx, projection_matrix.clone());

        println!("Wow Pre Update");
        flat_pipeline.update(ctx, &objects);
        println!("Wow Post Update");

        FlatBlendState {
            flat_pipeline,
            projection_matrix,
        }
    }
}

impl EventHandler for FlatBlendState {
    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        let mut projection_matrix = self.projection_matrix.lock().unwrap();
        *projection_matrix = Mat4::orthographic_rh_gl(
            -width / 100.0,
            width / 100.0,
            -height / 100.0,
            height / 100.0,
            -1.0,
            1.0,
        );
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.flat_pipeline.draw(ctx);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}
