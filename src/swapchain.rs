use std::sync::Arc;

use vulkano::{
    device::Device,
    image::{view::ImageView, ImageAccess, ImageUsage, SwapchainImage},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};
use winit::window::Window;

/// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();

    let image_format = Some(
        device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
    );
    let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count,

            image_format,

            image_extent: window.inner_size().into(),

            image_usage: ImageUsage {
                color_attachment: true,
                ..ImageUsage::empty()
            },

            composite_alpha: surface_capabilities
                .supported_composite_alpha
                .iter()
                .next()
                .unwrap(),

            ..Default::default()
        },
    )
    .unwrap()
}
