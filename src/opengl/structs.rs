use std::task::Context;

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
}

pub trait FlatBlendPipeline {
    fn draw(&mut self, ctx: &mut Context);
}
