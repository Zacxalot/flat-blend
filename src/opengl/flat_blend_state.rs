use miniquad::*;

use super::{pipelines::flat::FlatPipeline, structs::Object};

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
}

impl FlatBlendState {
    pub fn new(ctx: &mut Context, objects: Vec<Object>) -> FlatBlendState {
        ctx.set_cull_face(CullFace::Nothing);

        let mut flat_pipeline = FlatPipeline::new(ctx);

        println!("Wow Pre Update");
        flat_pipeline.update(ctx, &objects);
        println!("Wow Post Update");

        FlatBlendState { flat_pipeline }
    }
}

impl EventHandler for FlatBlendState {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.flat_pipeline.draw(ctx);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}
