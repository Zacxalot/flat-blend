use std::sync::Arc;

use enum_map::{Enum, EnumMap};
use vulkano::buffer::CpuAccessibleBuffer;

use crate::data::vertex::Vertex;

#[derive(Enum)]
pub enum VertexBufferKey {
    WireframeEdges,
    Flat,
    Grid,
}

#[derive(Enum)]
pub enum IndexBufferKey {
    Flat,
}

pub type VertexBuffers = EnumMap<VertexBufferKey, Option<Arc<CpuAccessibleBuffer<[Vertex]>>>>;
pub type IndexBuffers = EnumMap<IndexBufferKey, Option<Arc<CpuAccessibleBuffer<[u32]>>>>;
