use lyon::{
    geom::euclid::{Point2D, UnknownUnit},
    lyon_tessellation::{
        geometry_builder::simple_builder, FillOptions, FillTessellator, VertexBuffers,
    },
    math::Point,
    path::{traits::Build, Winding},
};

pub fn create_circle() -> VertexBuffers<Point2D<f32, UnknownUnit>, u16> {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
    let mut geometry_builder = simple_builder(&mut geometry);

    let options = FillOptions::tolerance(0.001);
    let mut tessellator = FillTessellator::new();

    let mut builder = tessellator.builder(&options, &mut geometry_builder);

    builder.add_circle([0.0, 0.0].into(), 1.0, Winding::Positive);

    builder.build().unwrap();

    geometry
}
