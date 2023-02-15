use crate::data::vertex::Vertex;

use super::{
    bm_edge::{bm_edge_kill, BMEdge},
    bmesh::BMesh,
};

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

pub fn bm_vert_kill(bmesh: &mut BMesh, vert: *mut BMVert) {
    unsafe {
        while let Some(edge) = (*vert).edge {
            bm_edge_kill(bmesh, edge);
        }

        bm_kill_only_vert(bmesh, vert);
    }
}

pub fn bm_kill_only_vert(bmesh: &mut BMesh, vert: *mut BMVert) {}
