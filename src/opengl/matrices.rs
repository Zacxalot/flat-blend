use glam::{Mat4, Vec2};

pub fn get_ortho_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(
        -width / 100.0,
        width / 100.0,
        -height / 100.0,
        height / 100.0,
        -1.0,
        1.0,
    )
}

pub fn get_view_matrix(position: Vec2, zoom: f32) -> Mat4 {
    let translation = glam::Vec3::new(position.x, position.y, 0.0);
    let scale = glam::Vec3::new(zoom, zoom, 1.0);
    Mat4::from_translation(translation) * Mat4::from_scale(scale)
}
