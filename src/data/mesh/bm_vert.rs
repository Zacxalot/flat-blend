use std::borrow::BorrowMut;

use crate::data::{mesh::bm_disk_link::bmesh_disk_edge_link_from_vert, vertex::Vertex};

use super::{
    bm_edge::{bm_edge_kill, BMEdge},
    bmesh::BMesh,
};

#[derive(Debug)]
pub struct BMVert {
    slab_index: usize,
    pub edge: Option<*mut BMEdge>,
    pub vertex: Vertex,
}

impl From<(f32, f32)> for BMVert {
    fn from(input: (f32, f32)) -> Self {
        BMVert {
            slab_index: 0,
            edge: None,
            vertex: Vertex::from(input),
        }
    }
}

pub fn bm_vert_create(bmesh: &mut BMesh) -> *mut BMVert {
    let v_index = bmesh.vertices.insert(BMVert::from((0.0, 0.0)));
    let v = bmesh.vertices.get_mut(v_index).unwrap();
    v.slab_index = v_index;

    v
}

pub fn bm_vert_kill(bmesh: &mut BMesh, vert: *mut BMVert) {
    unsafe {
        while let Some(edge) = (*vert).edge {
            bm_edge_kill(bmesh, edge);
        }

        bm_kill_only_vert(bmesh, vert);
    }
}

pub fn bm_kill_only_vert(bmesh: &mut BMesh, vert: *mut BMVert) {
    unsafe {
        bmesh.vertices.remove((*vert).slab_index);
    }
}
