use std::task::Context;

use crate::data::vertex::{Index, Vertex};

pub trait FlatBlendPipeline {
    fn draw(&mut self, ctx: &mut Context);
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderObject {
    pub tris: u32,
    pub index_offset: Index,
}
