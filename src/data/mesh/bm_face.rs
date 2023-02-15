use std::iter::zip;

use super::{
    bm_edge::BMEdge,
    bm_loop::{self, bm_loop_create, bmesh_radial_loop_append, BMLoop},
    bm_vert::BMVert,
    bmesh::BMesh,
};

pub struct BMFace {
    loop_start: Option<*mut BMLoop>,
    loop_len: usize,
    slab_index: usize,
}

pub fn bm_face_create(
    bmesh: &mut BMesh,
    verts: Vec<*mut BMVert>,
    edges: Vec<*mut BMEdge>,
) -> *mut BMFace {
    let f = bm_face_create__internal(bmesh);

    unsafe {
        (*f).loop_len = verts.len();
    }

    let start_l = bm_face_boundary_add(bmesh, f, verts[0], edges[0]);
    let mut last_l = start_l;

    for (vert, edge) in zip(verts, edges).skip(1) {
        let l = bm_loop_create(bmesh, vert, edge, f);

        bmesh_radial_loop_append(edge, l);

        unsafe {
            (*l).prev = Some(last_l);
            (*last_l).next = Some(l);
            last_l = l;
        }
    }

    unsafe {
        (*start_l).prev = Some(last_l);
        (*last_l).next = Some(start_l);
    }

    f
}

pub fn bm_face_create__internal(bmesh: &mut BMesh) -> *mut BMFace {
    let f_index = bmesh.faces.insert(BMFace {
        loop_len: 0,
        loop_start: None,
        slab_index: 0,
    });
    let f = bmesh.faces.get_mut(f_index).unwrap();
    f.slab_index = f_index;

    f
}

pub fn bm_face_boundary_add(
    bmesh: &mut BMesh,
    f: *mut BMFace,
    start_v: *mut BMVert,
    start_e: *mut BMEdge,
) -> *mut BMLoop {
    let l = bm_loop_create(bmesh, start_v, start_e, f);

    bmesh_radial_loop_append(start_e, l);

    l
}

pub fn bm_face_kill(_bmesh: &mut BMesh, _face: *mut BMFace) {
    todo!("Implement");
}
