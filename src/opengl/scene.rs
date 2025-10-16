use glam::Mat4;
use slotmap::{new_key_type, SlotMap};

use super::{frustum::Frustum, structs::Object};

// Define a strongly-typed key for objects in the scene
new_key_type! {
    pub struct ObjectKey;
}

/// Holds all scene objects and cached visibility/culling data
pub struct SceneData {
    /// All objects in the scene
    objects: SlotMap<ObjectKey, Object>,
    /// Ordered list of object keys by depth (bottom to top)
    /// First element renders first (bottom-most), last element renders last (top-most)
    object_order: Vec<ObjectKey>,
    /// Cached frustum from the current camera view
    frustum: Frustum,
    /// Keys of objects that are visible (passed frustum culling)
    visible_objects: Vec<ObjectKey>,
    /// Cached view-projection matrix used to generate the frustum
    cached_vp_matrix: Mat4,
}

impl SceneData {
    pub fn new(initial_objects: Vec<Object>) -> Self {
        let mut objects: SlotMap<ObjectKey, Object> = SlotMap::with_key();
        let mut object_order = Vec::new();

        for object in initial_objects {
            let key = objects.insert(object);
            object_order.push(key);
        }

        Self {
            objects,
            object_order,
            frustum: Frustum::from_matrix(Mat4::IDENTITY),
            visible_objects: Vec::new(),
            cached_vp_matrix: Mat4::IDENTITY,
        }
    }

    /// Get a reference to the object storage
    pub fn objects(&self) -> &SlotMap<ObjectKey, Object> {
        &self.objects
    }

    /// Get a mutable reference to the object storage
    pub fn objects_mut(&mut self) -> &mut SlotMap<ObjectKey, Object> {
        &mut self.objects
    }

    /// Get the ordered list of object keys (bottom to top)
    pub fn object_order(&self) -> &[ObjectKey] {
        &self.object_order
    }

    /// Get a mutable reference to the object order for reordering
    pub fn object_order_mut(&mut self) -> &mut Vec<ObjectKey> {
        &mut self.object_order
    }

    /// Get the keys of visible objects (in depth order)
    pub fn visible_objects(&self) -> &[ObjectKey] {
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

        // Recalculate visibility, maintaining depth order
        self.visible_objects.clear();
        for &key in &self.object_order {
            if let Some(object) = self.objects.get(key) {
                let aabb = object.get_aabb();
                if self.frustum.intersects_aabb_2d(aabb.min, aabb.max) {
                    self.visible_objects.push(key);
                }
            }
        }
    }

    /// Get selected objects from the visible set
    pub fn visible_selected_objects(&self) -> Vec<ObjectKey> {
        self.visible_objects
            .iter()
            .copied()
            .filter(|&key| self.objects[key].selected)
            .collect()
    }
}
