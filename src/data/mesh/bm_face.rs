use std::iter::zip;

use super::{
    bm_loop::{
        bm_kill_only_loop, bm_loop_create, bmesh_radial_loop_append,
        bmesh_radial_loop_remove,
    },
    bmesh::{BMesh, EdgeKey, FaceKey, LoopKey, VertKey},
};

pub struct BMFace {
    pub loop_start: Option<LoopKey>,
    pub loop_len: usize,
}

pub fn bm_face_create(
    bmesh: &mut BMesh,
    verts: &[VertKey],
    edges: &[EdgeKey],
) -> FaceKey {
    let f = bm_face_create_internal(bmesh);

    bmesh.faces[f].loop_len = verts.len();

    let start_l = bm_face_boundary_add(bmesh, f, verts[0], edges[0]);
    let mut last_l = start_l;

    for (vert, edge) in zip(verts, edges).skip(1) {
        let l = bm_loop_create(bmesh, *vert, *edge, f);

        bmesh_radial_loop_append(bmesh, *edge, l);

        bmesh.loops[l].prev = Some(last_l);
        bmesh.loops[last_l].next = Some(l);
        last_l = l;
    }

    bmesh.loops[start_l].prev = Some(last_l);
    bmesh.loops[last_l].next = Some(start_l);

    f
}

pub fn bm_face_create_internal(bmesh: &mut BMesh) -> FaceKey {
    bmesh.faces.insert(BMFace {
        loop_len: 0,
        loop_start: None,
    })
}

pub fn bm_face_boundary_add(
    bmesh: &mut BMesh,
    f: FaceKey,
    start_v: VertKey,
    start_e: EdgeKey,
) -> LoopKey {
    let l = bm_loop_create(bmesh, start_v, start_e, f);

    bmesh_radial_loop_append(bmesh, start_e, l);

    bmesh.faces[f].loop_start = Some(l);

    l
}

pub fn bm_face_kill(bmesh: &mut BMesh, face: FaceKey) {
    if let Some(l_first) = bmesh.faces.get(face).and_then(|f| f.loop_start) {
        let mut l_iter = l_first;

        loop {
            let loop_data = &bmesh.loops[l_iter];
            let l_next = loop_data.next.unwrap();
            let edge = loop_data.edge.unwrap();

            bmesh_radial_loop_remove(bmesh, edge, l_iter);
            bm_kill_only_loop(bmesh, l_iter);

            if l_next == l_first {
                break;
            }

            l_iter = l_next;
        }
    }

    bm_kill_only_face(bmesh, face);
}

pub fn bm_kill_only_face(bmesh: &mut BMesh, face: FaceKey) {
    bmesh.faces.remove(face);
}
