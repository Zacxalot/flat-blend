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
    mesh: Rc<RefCell<Mesh>>,
    translation: glam::Vec2,
    rotation: f32,
    scale: glam::Vec2,
    material: Rc<RefCell<Material>>,
    model_matrix: glam::Mat4,
    aabb: AABB2D,
    pub selected: bool,
}

impl Object {
    pub fn new(
        mesh: Rc<RefCell<Mesh>>,
        translation: glam::Vec2,
        rotation: f32,
        scale: glam::Vec2,
        material: Rc<RefCell<Material>>,
    ) -> Object {
        let mut obj = Object {
            mesh,
            translation,
            rotation,
            scale,
            material,
            selected: false,
            model_matrix: glam::Mat4::IDENTITY,
            aabb: AABB2D::new(glam::Vec2::ZERO, glam::Vec2::ZERO),
        };

        obj.update_model_matrix();
        obj.update_aabb();

        obj
    }

    fn update_aabb(&mut self) {
        // For a square mesh, the extents are 2x2 (from -1 to 1)
        let mesh_extents = glam::Vec2::new(2.0, 2.0);
        self.aabb = AABB2D::from_transform(self.translation, self.rotation, self.scale, mesh_extents);
    }

    pub fn borrow_mesh(&self) -> std::cell::Ref<Mesh> {
        self.mesh.borrow()
    }

    pub fn borrow_material(&self) -> std::cell::Ref<Material> {
        self.material.borrow()
    }

    fn update_model_matrix(&mut self) {
        let translation_mat = glam::Mat4::from_translation(self.translation.extend(0.0));
        let rotation_mat = glam::Mat4::from_rotation_z(self.rotation);
        let scale_mat = glam::Mat4::from_scale(self.scale.extend(1.0));
        self.model_matrix = translation_mat * rotation_mat * scale_mat;
    }

    pub fn get_model_matrix(&self) -> glam::Mat4 {
        self.model_matrix
    }

    /// Get the cached axis-aligned bounding box for this object
    pub fn get_aabb(&self) -> AABB2D {
        self.aabb
    }

    /// Check if a world-space point is inside this object's bounding box
    pub fn contains_point(&self, point: glam::Vec2) -> bool {
        point.x >= self.aabb.min.x
            && point.x <= self.aabb.max.x
            && point.y >= self.aabb.min.y
            && point.y <= self.aabb.max.y
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
