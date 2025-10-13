use glam::Mat4;

use super::{frustum::Frustum, structs::Object};

/// Holds all scene objects and cached visibility/culling data
pub struct SceneData {
    /// All objects in the scene
    objects: Vec<Object>,
    /// Cached frustum from the current camera view
    frustum: Frustum,
    /// Indices of objects that are visible (passed frustum culling)
    visible_objects: Vec<usize>,
    /// Cached view-projection matrix used to generate the frustum
    cached_vp_matrix: Mat4,
}

impl SceneData {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            frustum: Frustum::from_matrix(Mat4::IDENTITY),
            visible_objects: Vec::new(),
            cached_vp_matrix: Mat4::IDENTITY,
        }
    }

    /// Set the objects in the scene
    pub fn set_objects(&mut self, objects: Vec<Object>) {
        self.objects = objects;
    }

    /// Get a reference to all objects
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    /// Get a mutable reference to all objects
    pub fn objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }

    /// Get the indices of visible objects
    pub fn visible_objects(&self) -> &[usize] {
        &self.visible_objects
    }

    /// Get the cached frustum
    pub fn frustum(&self) -> &Frustum {
        &self.frustum
    }

    /// Update the frustum and recalculate visible objects
    /// Should be called when the camera moves or the projection changes
    pub fn update_visibility(&mut self, projection_matrix: Mat4, view_matrix: Mat4) {
        let vp_matrix = projection_matrix * view_matrix;

        self.cached_vp_matrix = vp_matrix;
        self.frustum = Frustum::from_matrix(vp_matrix);

        // Recalculate visibility
        self.visible_objects.clear();
        for (i, object) in self.objects.iter().enumerate() {
            let aabb = object.get_aabb();
            if self.frustum.intersects_aabb_2d(aabb.min, aabb.max) {
                self.visible_objects.push(i);
            }
        }
    }

    /// Get selected objects from the visible set
    pub fn visible_selected_objects(&self) -> Vec<usize> {
        self.visible_objects
            .iter()
            .copied()
            .filter(|&i| self.objects[i].selected)
            .collect()
    }
}
