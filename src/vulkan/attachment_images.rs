use std::sync::Arc;

use enum_map::{enum_map, Enum, EnumMap};
use vulkano::{
    format::Format,
    image::{AttachmentImage, ImageUsage},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator, MemoryAllocator},
};

pub type AttachmentImageMap = EnumMap<AttachmentImages, Arc<AttachmentImage>>;

#[derive(Enum)]
pub enum AttachmentImages {
    FinalOutput,
}

pub fn create_attachment_images(
    allocator: Arc<GenericMemoryAllocator<Arc<FreeListAllocator>>>,
    dimensions: [u32; 2],
    format: Format,
) -> AttachmentImageMap {
    enum_map! {
        AttachmentImages::FinalOutput => AttachmentImage::with_usage(&allocator, dimensions, format, ImageUsage {color_attachment: true, transfer_src: true, ..ImageUsage::empty()}).unwrap()
    }
}
