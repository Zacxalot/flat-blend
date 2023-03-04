use enum_map::{Enum, EnumMap};
use vulkano::buffer::CpuAccessibleBuffer;

use crate::data::vertex::Vertex;

#[derive(Enum)]
pub enum VertexBufferKey {
    WireframeEdges,
}

#[derive(Enum)]
pub enum IndexBufferKey {
    Flat,
}

pub type VertexBuffers = EnumMap<VertexBufferKey, Option<CpuAccessibleBuffer<[Vertex]>>>;
pub type IndexBuffers = EnumMap<IndexBufferKey, Option<CpuAccessibleBuffer<[u32]>>>;
