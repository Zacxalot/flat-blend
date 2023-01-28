use bytemuck::{Pod, Zeroable};
use lyon::geom::euclid::{Point2D, UnknownUnit};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position);

impl From<Point2D<f32, UnknownUnit>> for Vertex {
    fn from(point: Point2D<f32, UnknownUnit>) -> Self {
        Vertex {
            position: [point.x, point.y],
        }
    }
}
