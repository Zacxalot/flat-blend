use std::sync::Arc;

use cgmath::{ortho, Matrix4};
use vulkano::swapchain::Swapchain;

pub fn get_ortho(swapchain: Arc<Swapchain>) -> Matrix4<f32> {
    let x: f32 = (swapchain.image_extent()[0] as f32) / 100.0;
    let y: f32 = (swapchain.image_extent()[1] as f32) / 100.0;
    ortho(-x, x, y, -y, -10.0, 1.0)
}
