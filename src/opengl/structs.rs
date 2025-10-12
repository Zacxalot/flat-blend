use std::{cell::RefCell, rc::Rc, task::Context};

use crate::data::{
    mesh::bmesh::{bm_triangulate, BMesh},
    vertex::{Index, Vertex},
};

use super::frustum::AABB2D;

pub trait FlatBlendPipeline {
    fn draw(&mut self, ctx: &mut Context);
}

pub type Colour = glam::Vec4;

pub struct Material {
    pub colour: Colour,
}

pub struct Object {
    pub mesh: Rc<RefCell<Mesh>>,
    pub translation: glam::Vec2,
    pub rotation: f32,
    pub scale: glam::Vec2,
    pub material: Rc<RefCell<Material>>,
    pub selected: bool,
}

impl Object {
    /// Calculate the axis-aligned bounding box for this object
    /// Assumes the mesh is a square with extents from -1 to 1
    pub fn calculate_aabb(&self) -> AABB2D {
        // For a square mesh, the extents are 2x2 (from -1 to 1)
        let mesh_extents = glam::Vec2::new(2.0, 2.0);
        AABB2D::from_transform(self.translation, self.rotation, self.scale, mesh_extents)
    }

    /// Check if a world-space point is inside this object's bounding box
    pub fn contains_point(&self, point: glam::Vec2) -> bool {
        let aabb = self.calculate_aabb();
        point.x >= aabb.min.x && point.x <= aabb.max.x &&
        point.y >= aabb.min.y && point.y <= aabb.max.y
    }
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
