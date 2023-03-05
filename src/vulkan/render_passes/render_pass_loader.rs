use std::sync::Arc;

use enum_map::{enum_map, Enum, EnumMap};
use vulkano::{device::Device, format::Format, render_pass::RenderPass};

use crate::vulkan::render_passes::solid::solid_draw_pass;

#[derive(Enum)]
pub enum RenderPassKeys {
    Solid,
}

pub type RenderPasses = EnumMap<RenderPassKeys, Arc<RenderPass>>;

pub fn load_render_passes(device: Arc<Device>, format: Format) -> Arc<RenderPasses> {
    Arc::new(enum_map! {
        RenderPassKeys::Solid => solid_draw_pass(device.clone(), format).unwrap()
    })
}
