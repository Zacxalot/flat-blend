use glam::{Mat4, Vec2, Vec3, Vec4};

/// Represents a plane in 3D space using the equation: ax + by + cz + d = 0
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }

    /// Create a plane from a Vec4 (normal.x, normal.y, normal.z, distance)
    pub fn from_vec4(v: Vec4) -> Self {
        let normal = Vec3::new(v.x, v.y, v.z);
        let length = normal.length();
        if length > 0.0 {
            Self {
                normal: normal / length,
                distance: v.w / length,
            }
        } else {
            Self {
                normal: Vec3::ZERO,
                distance: v.w,
            }
        }
    }

    /// Calculate the signed distance from a point to this plane
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

/// Represents a viewing frustum with 6 planes
#[derive(Debug, Clone)]
pub struct Frustum {
    pub left: Plane,
    pub right: Plane,
    pub bottom: Plane,
    pub top: Plane,
    pub near: Plane,
    pub far: Plane,
}

impl Frustum {
    /// Extract frustum planes from a view-projection matrix
    /// Uses the Gribb-Hartmann method
    pub fn from_matrix(vp: Mat4) -> Self {
        // Extract rows of the matrix
        let row0 = vp.row(0);
        let row1 = vp.row(1);
        let row2 = vp.row(2);
        let row3 = vp.row(3);

        // Left plane: row3 + row0
        let left = Plane::from_vec4(row3 + row0);

        // Right plane: row3 - row0
        let right = Plane::from_vec4(row3 - row0);

        // Bottom plane: row3 + row1
        let bottom = Plane::from_vec4(row3 + row1);

        // Top plane: row3 - row1
        let top = Plane::from_vec4(row3 - row1);

        // Near plane: row3 + row2
        let near = Plane::from_vec4(row3 + row2);

        // Far plane: row3 - row2
        let far = Plane::from_vec4(row3 - row2);

        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }

    /// Test if an axis-aligned bounding box (AABB) intersects with the frustum
    /// Returns true if the AABB is at least partially inside the frustum
    pub fn intersects_aabb(&self, min: Vec3, max: Vec3) -> bool {
        // Test each plane
        for plane in [
            &self.left,
            &self.right,
            &self.bottom,
            &self.top,
            &self.near,
            &self.far,
        ] {
            // Find the positive vertex (furthest point along plane normal)
            let p = Vec3::new(
                if plane.normal.x >= 0.0 { max.x } else { min.x },
                if plane.normal.y >= 0.0 { max.y } else { min.y },
                if plane.normal.z >= 0.0 { max.z } else { min.z },
            );

            // If the positive vertex is on the negative side, the AABB is completely outside
            if plane.distance_to_point(p) < 0.0 {
                return false;
            }
        }

        true
    }

    /// Test if a 2D AABB (for 2D rendering) intersects with the frustum
    /// This is specialized for 2D rendering where z is typically 0
    pub fn intersects_aabb_2d(&self, min: Vec2, max: Vec2) -> bool {
        // For 2D, we only need to test the left, right, bottom, and top planes
        // Extend to 3D with z = 0
        let min_3d = Vec3::new(min.x, min.y, 0.0);
        let max_3d = Vec3::new(max.x, max.y, 0.0);

        // Test left, right, bottom, top planes
        for plane in [&self.left, &self.right, &self.bottom, &self.top] {
            let p = Vec3::new(
                if plane.normal.x >= 0.0 { max_3d.x } else { min_3d.x },
                if plane.normal.y >= 0.0 { max_3d.y } else { min_3d.y },
                if plane.normal.z >= 0.0 { max_3d.z } else { min_3d.z },
            );

            if plane.distance_to_point(p) < 0.0 {
                return false;
            }
        }

        true
    }
}

/// Axis-aligned bounding box in 2D
#[derive(Debug, Clone, Copy)]
pub struct AABB2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Create an AABB from a center point and extents
    pub fn from_center_extents(center: Vec2, extents: Vec2) -> Self {
        Self {
            min: center - extents,
            max: center + extents,
        }
    }

    /// Create an AABB for a transformed square object
    /// This handles translation, rotation, and scale
    pub fn from_transform(translation: Vec2, rotation: f32, scale: Vec2, mesh_extents: Vec2) -> Self {
        // For a rotated rectangle, we need to find the AABB that contains all corners
        let half_extents = mesh_extents * scale * 0.5;

        // Calculate the four corners of the rotated rectangle
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();

        let corners = [
            Vec2::new(-half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, half_extents.y),
            Vec2::new(-half_extents.x, half_extents.y),
        ];

        // Rotate and translate each corner
        let transformed_corners: Vec<Vec2> = corners
            .iter()
            .map(|&corner| {
                let rotated = Vec2::new(
                    corner.x * cos_r - corner.y * sin_r,
                    corner.x * sin_r + corner.y * cos_r,
                );
                rotated + translation
            })
            .collect();

        // Find the min and max of all transformed corners
        let mut min_x = transformed_corners[0].x;
        let mut max_x = transformed_corners[0].x;
        let mut min_y = transformed_corners[0].y;
        let mut max_y = transformed_corners[0].y;

        for corner in &transformed_corners[1..] {
            min_x = min_x.min(corner.x);
            max_x = max_x.max(corner.x);
            min_y = min_y.min(corner.y);
            max_y = max_y.max(corner.y);
        }

        Self {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_distance_to_point() {
        let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), -5.0);
        assert_eq!(plane.distance_to_point(Vec3::new(0.0, 5.0, 0.0)), 0.0);
        assert!(plane.distance_to_point(Vec3::new(0.0, 6.0, 0.0)) > 0.0);
        assert!(plane.distance_to_point(Vec3::new(0.0, 4.0, 0.0)) < 0.0);
    }

    #[test]
    fn test_aabb_from_center_extents() {
        let aabb = AABB2D::from_center_extents(Vec2::new(10.0, 20.0), Vec2::new(5.0, 10.0));
        assert_eq!(aabb.min, Vec2::new(5.0, 10.0));
        assert_eq!(aabb.max, Vec2::new(15.0, 30.0));
    }
}
