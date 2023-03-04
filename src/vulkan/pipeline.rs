use std::sync::Arc;

use vulkano::{
    buffer::{CpuAccessibleBuffer, CpuBufferPool, TypedBufferAccess},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        CommandBufferExecFuture, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::Queue,
    pipeline::{graphics::viewport::Viewport, GraphicsPipeline, Pipeline, PipelineBindPoint},
    render_pass::{Framebuffer, RenderPass},
    swapchain::{
        acquire_next_image, AcquireError, PresentFuture, Surface, Swapchain,
        SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainCreationError, SwapchainPresentInfo,
    },
    sync::{FenceSignalFuture, GpuFuture, JoinFuture},
};
use winit::window::Window;

use crate::{data::vertex::Vertex, vulkan::swapchain::size_viewport};

use super::{init::VulkanState, shaders::flat::vs::ty::Data, view::get_ortho};

type RenderFrameFutureFence = Option<
    Result<
        FenceSignalFuture<
            PresentFuture<
                CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>,
            >,
        >,
        vulkano::sync::FlushError,
    >,
>;

pub fn render_frame(state: &mut VulkanState) -> RenderFrameFutureFence {
    let window = state
        .surface
        .object()
        .unwrap()
        .downcast_ref::<Window>()
        .unwrap();
    let dimensions = window.inner_size();
    if dimensions.width == 0 || dimensions.height == 0 {
        return None;
    }

    state
        .previous_frame_end
        .as_mut()
        .unwrap()
        .cleanup_finished();

    if state.recreate_swapchain {
        println!("Recreate swapchain");
        let (new_swapchain, new_images) = match state.swapchain.recreate(SwapchainCreateInfo {
            image_extent: dimensions.into(),
            ..state.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return None,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        state.swapchain = new_swapchain;
        size_viewport(&new_images, &mut state.viewport);
        state.recreate_swapchain = false;
    }

    let uniform_buffer_subbuffer = {
        let uniform_data = crate::vulkan::shaders::flat::vs::ty::Data {
            view: get_ortho(state.swapchain.clone()).into(),
        };

        // uniform_buffer.from_data(uniform_data).unwrap()
    };

    // let layout = pipeline.layout().set_layouts().get(0).unwrap();
    // let set = PersistentDescriptorSet::new(
    //     descriptor_set_allocator,
    //     layout.clone(),
    //     [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
    // )
    // .unwrap();

    let (image_index, suboptimal, acquire_future) =
        match acquire_next_image(state.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                state.recreate_swapchain = true;
                return None;
            }
            Err(e) => panic!("Failed to acquire next image: {:?}", e),
        };

    if suboptimal {
        state.recreate_swapchain = true;
    }

    let mut builder = AutoCommandBufferBuilder::primary(
        &state.command_buffer_allocator,
        state.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    // builder
    // .begin_render_pass(
    //     RenderPassBeginInfo {
    //         clear_values: vec![Some([0.02, 0.02, 0.02, 1.0].into())],
    //         ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
    //     },
    //     SubpassContents::Inline,
    // )
    // .unwrap()
    // .set_viewport(0, [viewport.clone()])
    // .bind_pipeline_graphics(pipeline.clone())
    // .bind_descriptor_sets(
    //     PipelineBindPoint::Graphics,
    //     pipeline.layout().clone(),
    //     0,
    //     set,
    // )
    // .bind_vertex_buffers(0, vertex_buffer.clone())
    // .draw(vertex_buffer.len() as u32, 1, 0, 0)
    // .unwrap()
    // .end_render_pass()
    // .unwrap();

    let command_buffer = builder.build().unwrap();

    Some(
        state
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(state.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                state.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(state.swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush(),
    )
}
