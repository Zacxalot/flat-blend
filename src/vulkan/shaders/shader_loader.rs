use std::{collections::HashMap, sync::Arc};

use vulkano::{device::Device, shader::ShaderModule};

use super::flat;

pub type LoadedShaders = HashMap<ShaderKey, Arc<ShaderModule>>;

#[derive(PartialEq, Eq, Hash)]
pub enum ShaderKey {
    FlatVs,
    FlatFs,
}

pub fn load_shaders(device: Arc<Device>) -> Arc<LoadedShaders> {
    let mut shaders: HashMap<ShaderKey, Arc<ShaderModule>> = HashMap::new();

    shaders.insert(ShaderKey::FlatVs, flat::vs::load(device.clone()).unwrap());
    shaders.insert(ShaderKey::FlatFs, flat::fs::load(device).unwrap());

    Arc::from(shaders)
}
