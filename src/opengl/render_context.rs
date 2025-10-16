use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use glam::Mat4;
use miniquad::Context;

use super::{
    pipelines::{flat::FlatPipeline, grid::GridPipeline, outline::OutlinePipeline},
    scene::SceneData,
    structs::{Mesh, Object},
};

/// Manages scene data and all rendering pipelines
pub struct RenderContext {
    pub scene_data: SceneData,
    pub flat_pipeline: FlatPipeline,
    pub grid_pipeline: GridPipeline,
    pub outline_pipeline: OutlinePipeline,
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,
}

impl RenderContext {
    pub fn new(
        ctx: &mut Context,
        projection_matrix: Arc<Mutex<Mat4>>,
        view_matrix: Arc<Mutex<Mat4>>,
        zoom: Arc<Mutex<f32>>,
        position: Arc<Mutex<glam::Vec2>>,
        objects: Vec<Object>,
        meshes: Vec<Rc<RefCell<Mesh>>>,
        width: u32,
        height: u32,
    ) -> Self {
        let mut flat_pipeline =
            FlatPipeline::new(ctx, projection_matrix.clone(), view_matrix.clone());
        let grid_pipeline = GridPipeline::new(ctx, position, zoom);

        // Update the flat pipeline with mesh data
        flat_pipeline.update(ctx, meshes.clone());

        let mut outline_pipeline = OutlinePipeline::new(
            ctx,
            projection_matrix.clone(),
            view_matrix.clone(),
            width,
            height,
        );
        outline_pipeline.update(ctx, meshes);

        let mut scene_data = SceneData::new();
        scene_data.set_objects(objects);

        let mut render_context = Self {
            scene_data,
            flat_pipeline,
            grid_pipeline,
            outline_pipeline,
            projection_matrix,
            view_matrix,
        };

        // Initial visibility calculation
        render_context.update_visibility();

        render_context
    }

    /// Update scene visibility based on current camera matrices
    /// Call this after the camera has moved or projection has changed
    pub fn update_visibility(&mut self) {
        let projection_matrix = *self.projection_matrix.lock().unwrap();
        let view_matrix = *self.view_matrix.lock().unwrap();
        self.scene_data
            .update_visibility(projection_matrix, view_matrix);
    }

    /// Draw all pipelines
    pub fn draw(&mut self, ctx: &mut Context) {
        let projection_matrix = *self.projection_matrix.lock().unwrap();
        let view_matrix = *self.view_matrix.lock().unwrap();

        self.grid_pipeline.draw(ctx);
        self.flat_pipeline
            .draw(ctx, &self.scene_data, projection_matrix, view_matrix);
        self.outline_pipeline
            .draw(ctx, &self.scene_data, projection_matrix, view_matrix);
    }

    /// Handle window resize
    pub fn resize(&mut self, ctx: &mut Context, width: u32, height: u32) {
        self.outline_pipeline.resize(ctx, width, height);
    }
}
