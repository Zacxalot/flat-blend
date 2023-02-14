use std::mem::ManuallyDrop;

use crate::data::vertex::Vertex;

use super::{bm_edge::BMEdge, bmesh::BMesh};

pub type PBMVert = *mut ManuallyDrop<BMVert>;

#[derive(Debug)]
pub struct BMVert {
    pub edge: Option<*mut BMEdge>,
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

pub fn bm_vert_create(bmesh: &mut BMesh) -> *mut BMVert {
    bmesh.vertices.alloc(BMVert::from((0.0, 0.0)))
}
