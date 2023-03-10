use super::{bm_edge::BMEdge, bm_face::BMFace, bm_vert::BMVert, bmesh::BMesh};

pub struct BMLoop {
    pub slab_index: usize,
    pub vertex: *mut BMVert,
    pub edge: Option<*mut BMEdge>,
    pub face: *mut BMFace,
    pub next: Option<*mut BMLoop>,
    pub prev: Option<*mut BMLoop>,
    pub radial_next: Option<*mut BMLoop>,
    pub radial_prev: Option<*mut BMLoop>,
}

pub fn bm_loop_create(
    bmesh: &mut BMesh,
    v: *mut BMVert,
    e: *mut BMEdge,
    f: *mut BMFace,
) -> *mut BMLoop {
    let l_index = bmesh.loops.insert(BMLoop {
        slab_index: 0,
        vertex: v,
        edge: Some(e),
        face: f,
        next: None,
        prev: None,
        radial_next: None,
        radial_prev: None,
    });
    let l = bmesh.loops.get_mut(l_index).unwrap();
    l.slab_index = l_index;

    l
}

pub fn bmesh_radial_loop_append(e: *mut BMEdge, l: *mut BMLoop) {
    unsafe {
        if let Some(e_loop) = (*e).r#loop {
            (*l).radial_prev = (*e).r#loop;
            (*l).radial_next = (*e_loop).radial_next;

            (*(*e_loop).radial_next.unwrap()).radial_prev = Some(l);
            (*e_loop).radial_next = Some(l);

            (*e).r#loop = Some(l);
        } else {
            (*e).r#loop = Some(l);
            (*l).radial_next = Some(l);
            (*l).radial_prev = Some(l);
        }

        (*l).edge = Some(e);
    }
}

pub fn bmesh_radial_loop_remove(e: *mut BMEdge, l: *mut BMLoop) {
    unsafe {
        if (*l).radial_next != Some(l) {
            if (*e).r#loop == Some(l) {
                (*e).r#loop = (*l).radial_next;
            }

            (*(*l).radial_next.unwrap()).radial_prev = (*l).radial_prev;
            (*(*l).radial_prev.unwrap()).radial_prev = (*l).radial_next;
        } else if (*e).r#loop == Some(l) {
            (*e).r#loop = None
        }

        (*l).radial_next = None;
        (*l).radial_prev = None;
        (*l).edge = None;
    }
}

pub unsafe fn bm_kill_only_loop(bmesh: &mut BMesh, l: *mut BMLoop) {
    bmesh.loops.remove((*l).slab_index);
}

pub struct BMLoopIterator {
    current: Option<*mut BMLoop>,
    start: *mut BMLoop,
    finished: bool,
}

impl BMLoopIterator {
    #[allow(dead_code)]
    pub fn new(start: *mut BMLoop) -> Self {
        Self {
            current: Some(start),
            start,
            finished: false,
        }
    }
}

impl Iterator for BMLoopIterator {
    type Item = *mut BMLoop;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current = self.current.take()?;

        unsafe {
            if (*current).next == Some(self.start) {
                self.finished = true;
            } else {
                self.current = (*current).next;
            }
        }

        Some(current)
    }
}
