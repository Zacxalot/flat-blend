use std::sync::Arc;

use enum_map::{enum_map, Enum, EnumMap};
use vulkano::{device::Device, shader::ShaderModule};

use super::flat;

pub type LoadedShaders = EnumMap<ShaderKey, Arc<ShaderModule>>;

#[derive(Enum)]
pub enum ShaderKey {
    FlatVs,
    FlatFs,
}

pub fn load_shaders(device: Arc<Device>) -> Arc<LoadedShaders> {
    let map = enum_map! {
        ShaderKey::FlatVs => flat::vs::load(device.clone()).unwrap(),
        ShaderKey::FlatFs => flat::fs::load(device.clone()).unwrap()
    };

    Arc::from(map)
}
