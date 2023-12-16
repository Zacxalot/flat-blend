use glam::Vec2;
use lyon::geom::euclid::{Point2D, UnknownUnit};

pub type Index = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub pos: Vec2,
}

impl From<Point2D<f32, UnknownUnit>> for Vertex {
    fn from(point: Point2D<f32, UnknownUnit>) -> Self {
        Vertex {
            pos: Vec2 {
                x: point.x,
                y: point.y,
            },
        }
    }
}

impl From<(f32, f32)> for Vertex {
    fn from((x, y): (f32, f32)) -> Self {
        Vertex { pos: Vec2 { x, y } }
    }
}
