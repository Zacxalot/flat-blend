use std::task::Context;

use glam::Vec2;

use crate::data::vertex::{Index, Vertex};

pub trait FlatBlendPipeline {
    fn draw(&mut self, ctx: &mut Context);
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
    pub translation: glam::Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderObject {
    pub tris: u32,
    pub index_offset: Index,
    pub translation: Vec2,
}
