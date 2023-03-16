use std::sync::Arc;

use enum_map::{enum_map, Enum, EnumMap};
use vulkano::{
    format::Format,
    image::{AttachmentImage, ImageUsage},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator},
    render_pass::Framebuffer,
};

use crate::vulkan::render_passes::{grid, solid};

use super::render_passes::render_pass_loader::RenderPasses;

pub type AttachmentImageMap = EnumMap<AttachmentImageKeys, Arc<AttachmentImage>>;
pub type FrameBufferMap = EnumMap<FrameBufferKeys, Arc<Framebuffer>>;

#[derive(Enum)]
pub enum AttachmentImageKeys {
    FinalOutput,
}

#[derive(Enum)]
pub enum FrameBufferKeys {
    Solid,
    Grid,
}

pub fn create_attachment_images(
    allocator: Arc<GenericMemoryAllocator<Arc<FreeListAllocator>>>,
    dimensions: [u32; 2],
    format: Format,
) -> Arc<AttachmentImageMap> {
    Arc::new(enum_map! {
        AttachmentImageKeys::FinalOutput => AttachmentImage::with_usage(&allocator, dimensions, format, ImageUsage {color_attachment: true, transfer_src: true, ..ImageUsage::empty()}).unwrap()
    })
}

pub fn create_frame_buffers(
    render_passes: Arc<RenderPasses>,
    attachment_images: Arc<AttachmentImageMap>,
) -> Arc<FrameBufferMap> {
    Arc::new(enum_map! {
        FrameBufferKeys::Solid => {solid::create_framebuffer(render_passes.clone(), attachment_images.clone())},
        FrameBufferKeys::Grid => {grid::create_framebuffer(render_passes.clone(), attachment_images.clone())}
    })
}
