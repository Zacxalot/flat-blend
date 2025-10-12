use glam::{Mat4, Vec2, Vec4};

pub fn get_ortho_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(
        -width / 2.0,
        width / 2.0,
        -height / 2.0,
        height / 2.0,
        -1.0,
        1.0,
    )
}

pub fn get_view_matrix(position: Vec2, zoom: f32) -> Mat4 {
    let translation = glam::Vec3::new(position.x, position.y, 0.0);
    let scale = glam::Vec3::new(zoom, zoom, 1.0);
    Mat4::from_translation(translation) * Mat4::from_scale(scale)
}

/// Convert screen coordinates to world coordinates
pub fn screen_to_world(
    screen_pos: Vec2,
    screen_size: (f32, f32),
    view_matrix: Mat4,
    projection_matrix: Mat4,
) -> Vec2 {
    // Convert screen coordinates to NDC (-1 to 1)
    // Note: screen Y is top-down, NDC Y is bottom-up
    let ndc_x = (2.0 * screen_pos.x) / screen_size.0 - 1.0;
    let ndc_y = 1.0 - (2.0 * screen_pos.y) / screen_size.1;

    // Inverse transform: NDC -> World
    let vp_matrix = projection_matrix * view_matrix;
    let inv_vp = vp_matrix.inverse();
    let world_pos = inv_vp * Vec4::new(ndc_x, ndc_y, 0.0, 1.0);

    Vec2::new(world_pos.x, world_pos.y)
}
