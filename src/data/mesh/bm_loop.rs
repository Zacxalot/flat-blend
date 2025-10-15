use super::bmesh::{BMesh, EdgeKey, FaceKey, LoopKey, VertKey};

pub struct BMLoop {
    pub vertex: VertKey,
    pub edge: Option<EdgeKey>,
    pub face: FaceKey,
    pub next: Option<LoopKey>,
    pub prev: Option<LoopKey>,
    pub radial_next: Option<LoopKey>,
    pub radial_prev: Option<LoopKey>,
}

pub fn bm_loop_create(
    bmesh: &mut BMesh,
    v: VertKey,
    e: EdgeKey,
    f: FaceKey,
) -> LoopKey {
    bmesh.loops.insert(BMLoop {
        vertex: v,
        edge: Some(e),
        face: f,
        next: None,
        prev: None,
        radial_next: None,
        radial_prev: None,
    })
}

pub fn bmesh_radial_loop_append(bmesh: &mut BMesh, e: EdgeKey, l: LoopKey) {
    let edge = &bmesh.edges[e];
    if let Some(e_loop_key) = edge.r#loop {
        let e_loop = &bmesh.loops[e_loop_key];
        let radial_next = e_loop.radial_next;

        bmesh.loops[l].radial_prev = Some(e_loop_key);
        bmesh.loops[l].radial_next = radial_next;

        if let Some(rn) = radial_next {
            bmesh.loops[rn].radial_prev = Some(l);
        }
        bmesh.loops[e_loop_key].radial_next = Some(l);

        bmesh.edges[e].r#loop = Some(l);
    } else {
        bmesh.edges[e].r#loop = Some(l);
        bmesh.loops[l].radial_next = Some(l);
        bmesh.loops[l].radial_prev = Some(l);
    }

    bmesh.loops[l].edge = Some(e);
}

pub fn bmesh_radial_loop_remove(bmesh: &mut BMesh, e: EdgeKey, l: LoopKey) {
    let loop_data = &bmesh.loops[l];
    let radial_next = loop_data.radial_next;
    let radial_prev = loop_data.radial_prev;

    if radial_next != Some(l) {
        if bmesh.edges[e].r#loop == Some(l) {
            bmesh.edges[e].r#loop = radial_next;
        }

        if let Some(rn) = radial_next {
            bmesh.loops[rn].radial_prev = radial_prev;
        }
        if let Some(rp) = radial_prev {
            bmesh.loops[rp].radial_next = radial_next;
        }
    } else if bmesh.edges[e].r#loop == Some(l) {
        bmesh.edges[e].r#loop = None;
    }

    bmesh.loops[l].radial_next = None;
    bmesh.loops[l].radial_prev = None;
    bmesh.loops[l].edge = None;
}

pub fn bm_kill_only_loop(bmesh: &mut BMesh, l: LoopKey) {
    bmesh.loops.remove(l);
}

pub struct BMLoopIterator<'a> {
    bmesh: &'a BMesh,
    current: Option<LoopKey>,
    start: LoopKey,
    finished: bool,
}

impl<'a> BMLoopIterator<'a> {
    #[allow(dead_code)]
    pub fn new(bmesh: &'a BMesh, start: LoopKey) -> Self {
        Self {
            bmesh,
            current: Some(start),
            start,
            finished: false,
        }
    }
}

impl<'a> Iterator for BMLoopIterator<'a> {
    type Item = LoopKey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current = self.current.take()?;

        if let Some(loop_data) = self.bmesh.loops.get(current) {
            if loop_data.next == Some(self.start) {
                self.finished = true;
            } else {
                self.current = loop_data.next;
            }
        } else {
            self.finished = true;
        }

        Some(current)
    }
}
