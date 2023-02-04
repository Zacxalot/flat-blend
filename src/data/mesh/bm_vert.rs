use std::mem::ManuallyDrop;

use crate::data::vertex::Vertex;

use super::{
    bm_edge::{BMEdge, PBMEdge},
    bmesh::BMesh,
};

pub type PBMVert = *mut ManuallyDrop<BMVert>;

#[derive(Debug)]
pub struct BMVert {
    pub edge: Option<PBMEdge>,
    pub vertex: Vertex,
}

impl From<(f32, f32)> for BMVert {
    fn from(input: (f32, f32)) -> Self {
        BMVert {
            edge: None,
            vertex: Vertex::from(input),
        }
    }
}

pub fn bm_vert_create(_bmesh: &mut BMesh) -> ManuallyDrop<BMVert> {
    ManuallyDrop::new(BMVert::from((0.0, 0.0)))
}
