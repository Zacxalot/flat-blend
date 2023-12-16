use bytemuck::{Pod, Zeroable};
use lyon::geom::euclid::{Point2D, UnknownUnit};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod, Vertex, BufferContents)]
pub struct FBVertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

impl From<Point2D<f32, UnknownUnit>> for FBVertex {
    fn from(point: Point2D<f32, UnknownUnit>) -> Self {
        FBVertex {
            position: [point.x, point.y],
        }
    }
}

impl From<(f32, f32)> for FBVertex {
    fn from((x, y): (f32, f32)) -> Self {
        FBVertex { position: [x, y] }
    }
}
