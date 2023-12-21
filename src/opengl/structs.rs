use std::{cell::RefCell, rc::Rc, task::Context};

use crate::data::{
    mesh::bmesh::{bm_triangulate, BMesh},
    vertex::{Index, Vertex},
};

pub trait FlatBlendPipeline {
    fn draw(&mut self, ctx: &mut Context);
}

pub struct Object {
    pub mesh: Rc<RefCell<Mesh>>,
    pub translation: glam::Vec2,
}

pub struct Mesh {
    pub raw_mesh: BMesh,
    pub tris: u32,
    pub buffer_offset: Index,

    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn new(raw_mesh: BMesh, offset: u32) -> (Rc<RefCell<Mesh>>, Vec<Vertex>, Vec<Index>) {
        let (vertices, indices) = bm_triangulate(&raw_mesh);

        (
            Rc::new(RefCell::new(Mesh {
                raw_mesh,
                vertices: vertices.clone(),
                indices: indices.clone(),
                tris: indices.len() as u32 / 3,
                buffer_offset: offset,
            })),
            vertices,
            indices,
        )
    }

    pub fn update(&mut self, offset: u32) -> (Vec<Vertex>, Vec<Index>) {
        self.buffer_offset = offset;

        (self.vertices.clone(), self.indices.clone())
    }
}
