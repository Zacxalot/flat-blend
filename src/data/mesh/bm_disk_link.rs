use super::bmesh::{EdgeKey, VertKey};

#[derive(Debug)]
pub struct BMDiskLink {
    next: Option<EdgeKey>,
    prev: Option<EdgeKey>,
}

impl BMDiskLink {
    pub fn new() -> Self {
        BMDiskLink {
            next: None,
            prev: None,
        }
    }
}

pub fn bmesh_disk_edge_append(bmesh: &mut super::bmesh::BMesh, e: EdgeKey, v: VertKey) {
    if let Some(v_edge) = bmesh.vertices[v].edge {
        let dl2_prev = bmesh_disk_edge_link_from_vert(&bmesh.edges[v_edge], v).prev;

        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).next = Some(v_edge);
        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).prev = dl2_prev;

        if let Some(dl2_prev_edge) = dl2_prev {
            bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[dl2_prev_edge], v).next = Some(e);
        }

        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[v_edge], v).prev = Some(e);
    } else {
        bmesh.vertices[v].edge = Some(e);
        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).next = Some(e);
        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).prev = Some(e);
    }
}

#[allow(dead_code)]
pub fn bmesh_disk_edge_remove(bmesh: &mut super::bmesh::BMesh, e: EdgeKey, v: VertKey) {
    let dl1 = bmesh_disk_edge_link_from_vert(&bmesh.edges[e], v);
    let dl1_next = dl1.next;
    let dl1_prev = dl1.prev;

    if let Some(dl1_prev_edge) = dl1_prev {
        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[dl1_prev_edge], v).next = dl1_next;
    }

    if let Some(dl1_next_edge) = dl1_next {
        bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[dl1_next_edge], v).prev = dl1_prev;
    }

    if let Some(v_edge) = bmesh.vertices[v].edge {
        if v_edge == e {
            if dl1_next != Some(e) {
                bmesh.vertices[v].edge = dl1_next;
            } else {
                bmesh.vertices[v].edge = None;
            }
        }
    }

    bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).next = None;
    bmesh_disk_edge_link_from_vert_mut(&mut bmesh.edges[e], v).prev = None;
}

pub fn bmesh_disk_edge_link_from_vert(e: &super::bm_edge::BMEdge, v: VertKey) -> &BMDiskLink {
    if e.v0 == v {
        &e.v0_disk_link
    } else {
        &e.v1_disk_link
    }
}

pub fn bmesh_disk_edge_link_from_vert_mut(
    e: &mut super::bm_edge::BMEdge,
    v: VertKey,
) -> &mut BMDiskLink {
    if e.v0 == v {
        &mut e.v0_disk_link
    } else {
        &mut e.v1_disk_link
    }
}
