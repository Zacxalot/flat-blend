use super::vertex::Vertex;

pub struct EEdge(pub usize, pub usize);

pub struct ELoop {
    pub vertex: usize,
    pub edge: *const EEdge,
    pub face: *const EFace,
    pub next: Option<*mut ELoop>,
    pub prev: Option<*mut ELoop>,
}

pub struct EFace {
    loop_start: *const ELoop,
    loop_len: usize,
}

pub struct EMesh {
    vertices: Vec<Vertex>,
    edges: Vec<EEdge>,
    loops: Vec<ELoop>,
    faces: Vec<EFace>,
}

pub fn gen_square() -> EMesh {
    let vertices = vec![
        Vertex::from((-1.0, -1.0)),
        Vertex::from((1.0, -1.0)),
        Vertex::from((1.0, 1.0)),
        Vertex::from((-1.0, 1.0)),
    ];

    let edges = vec![EEdge(0, 1), EEdge(1, 2), EEdge(2, 3), EEdge(3, 0)];

    let mut face = EFace {
        loop_start: std::ptr::null(),
        loop_len: 4,
    };

    let mut faces = vec![face];

    let mut loop_1 = ELoop {
        vertex: 0,
        edge: &edges[0],
        face: &faces[0],
        next: None,
        prev: None,
    };

    let mut loop_2 = ELoop {
        vertex: 1,
        edge: &edges[1],
        face: &faces[0],
        next: None,
        prev: None,
    };

    let mut loop_3 = ELoop {
        vertex: 2,
        edge: &edges[2],
        face: &faces[0],
        next: None,
        prev: None,
    };

    let mut loop_4 = ELoop {
        vertex: 3,
        edge: &edges[3],
        face: &faces[0],
        next: None,
        prev: None,
    };

    let mut loops = vec![loop_1, loop_2, loop_3, loop_4];

    loops[0].prev = Some(core::ptr::addr_of_mut!(loops[3]));
    loops[0].next = Some(core::ptr::addr_of_mut!(loops[1]));
    loops[1].prev = Some(core::ptr::addr_of_mut!(loops[0]));
    loops[1].next = Some(core::ptr::addr_of_mut!(loops[2]));
    loops[2].prev = Some(core::ptr::addr_of_mut!(loops[1]));
    loops[2].next = Some(core::ptr::addr_of_mut!(loops[3]));
    loops[3].prev = Some(core::ptr::addr_of_mut!(loops[2]));
    loops[3].next = Some(core::ptr::addr_of_mut!(loops[0]));

    faces[0].loop_start = &loops[0];

    EMesh {
        vertices,
        edges,
        loops,
        faces,
    }
}

pub fn edges_of_face(emesh: EMesh, face_pos: usize) -> Vec<*const EEdge> {
    let face = &emesh.faces[0];
    let mut edges: Vec<*const EEdge> = vec![];

    let mut to_get = face.loop_start;

    for i in 0..face.loop_len {
        unsafe {
            edges.push((*to_get).edge);
            to_get = (*to_get).next.unwrap();
        }
    }

    edges
}
