use std::sync::Arc;

use enum_map::enum_map;
use vulkano::{
    buffer::{BufferUsage, CpuBufferPool},
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{Device, Queue},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{MemoryUsage, StandardMemoryAllocator},
    pipeline::graphics::viewport::Viewport,
    swapchain::{Surface, Swapchain},
    sync::{self, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use super::{
    buffers::{IndexBuffers, VertexBuffers},
    device::get_device,
    shaders::{
        flat,
        shader_loader::{load_shaders, LoadedShaders},
    },
    swapchain::{create_swapchain, size_viewport},
};

#[allow(dead_code)]
pub struct VulkanState {
    pub device: Arc<Device>,
    pub surface: Arc<Surface>,
    // pub framebuffers: Vec<Arc<Framebuffer>>,
    pub descriptor_set_allocator: StandardDescriptorSetAllocator,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub queue: Arc<Queue>,
    pub vertex_buffers: VertexBuffers,
    pub index_buffers: IndexBuffers,
    pub shaders: Arc<LoadedShaders>,
    pub swapchain: Arc<Swapchain>,
    pub viewport: Viewport,
}

pub fn vulkano_init() -> (VulkanState, EventLoop<()>) {
    let library = VulkanLibrary::new().unwrap();
    let required_extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            enumerate_portability: true,
            ..Default::default()
        },
    )
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let (device, mut queues) = get_device(instance, surface.clone());

    let queue = queues.next().unwrap();

    let (swapchain, images) = create_swapchain(device.clone(), surface.clone());

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    // let mut square_mesh = create_square();
    // let (vertices, indices) = bm_triangulate(&mut square_mesh);
    // let vertices = bm_edge_list(&mut square_mesh);
    // println!("{:?}", vertices);

    // let _vertex_buffer = CpuAccessibleBuffer::from_iter(
    //     &memory_allocator,
    //     BufferUsage {
    //         vertex_buffer: true,
    //         ..BufferUsage::empty()
    //     },
    //     false,
    //     vertices,
    // )
    // .unwrap();

    // let mut index_buffer = CpuAccessibleBuffer::from_iter(
    //     &memory_allocator,
    //     BufferUsage {
    //         index_buffer: true,
    //         ..BufferUsage::empty()
    //     },
    //     false,
    //     [],#
    // )
    // .unwrap();

    let _uniform_buffer = CpuBufferPool::<flat::vs::ty::Data>::new(
        memory_allocator,
        BufferUsage {
            uniform_buffer: true,
            ..BufferUsage::empty()
        },
        MemoryUsage::Upload,
    );

    let shaders = load_shaders(device.clone());
    // let render_pass = solid_draw_pass(device.clone(), swapchain.image_format()).unwrap();

    // let _pipeline = solid_draw_pipeline(render_pass.clone(), device.clone(), shaders).unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    size_viewport(&images, &mut viewport);

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    (
        VulkanState {
            device,
            surface,
            descriptor_set_allocator,
            command_buffer_allocator,
            recreate_swapchain: false,
            previous_frame_end,
            queue,
            vertex_buffers: enum_map! {_ => None},
            index_buffers: enum_map! {_ => None},
            shaders,
            swapchain,
            viewport,
        },
        event_loop,
    )
}
