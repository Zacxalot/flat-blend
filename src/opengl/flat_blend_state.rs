use miniquad::*;

use super::pipelines::flat::FlatPipeline;

pub struct FlatBlendState {
    flat_pipeline: FlatPipeline,
}

impl FlatBlendState {
    pub fn new(ctx: &mut Context) -> FlatBlendState {
        FlatBlendState {
            flat_pipeline: FlatPipeline::new(ctx),
        }
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
