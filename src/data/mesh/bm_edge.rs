use super::{
    bm_disk_link::{bmesh_disk_edge_append, bmesh_disk_edge_remove, BMDiskLink},
    bm_face::bm_face_kill,
    bmesh::{BMesh, LoopKey, VertKey},
};

pub struct BMEdge {
    pub v0: VertKey,
    pub v1: VertKey,
    pub r#loop: Option<LoopKey>,
    pub v0_disk_link: BMDiskLink,
    pub v1_disk_link: BMDiskLink,
}

pub fn bm_edge_create(bmesh: &mut BMesh, v0: VertKey, v1: VertKey) -> super::bmesh::EdgeKey {
    let e_key = bmesh.edges.insert(BMEdge {
        v0,
        v1,
        r#loop: None,
        v0_disk_link: BMDiskLink::new(),
        v1_disk_link: BMDiskLink::new(),
    });

    bmesh_disk_edge_append(bmesh, e_key, v0);
    bmesh_disk_edge_append(bmesh, e_key, v1);

    e_key
}

#[allow(dead_code)]
pub fn bm_edge_kill(bmesh: &mut BMesh, edge: super::bmesh::EdgeKey) {
    while let Some(loop_key) = bmesh.edges.get(edge).and_then(|e| e.r#loop) {
        let face_key = bmesh.loops[loop_key].face;
        bm_face_kill(bmesh, face_key);
    }

    if let Some(e) = bmesh.edges.get(edge) {
        let v0 = e.v0;
        let v1 = e.v1;
        bmesh_disk_edge_remove(bmesh, edge, v0);
        bmesh_disk_edge_remove(bmesh, edge, v1);
    }

    bm_kill_only_edge(bmesh, edge)
}

#[allow(dead_code)]
pub fn bm_kill_only_edge(bmesh: &mut BMesh, edge: super::bmesh::EdgeKey) {
    bmesh.edges.remove(edge);
}
