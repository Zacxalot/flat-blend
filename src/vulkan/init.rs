use std::sync::Arc;

use enum_map::enum_map;
use vulkano::{
    buffer::{BufferUsage, CpuBufferPool},
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{Device, Queue},
    image::{ImageAccess, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        FreeListAllocator, GenericMemoryAllocator, MemoryUsage, StandardMemoryAllocator,
    },
    pipeline::graphics::viewport::Viewport,
    swapchain::{Surface, Swapchain},
    sync::{self, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use super::{
    attachment_images::{
        create_attachment_images, create_frame_buffers, AttachmentImageMap, FrameBufferMap,
    },
    buffers::{IndexBuffers, VertexBuffers},
    device::get_device,
    pipeline::{load_pipelines, Pipelines},
    render_passes::render_pass_loader::{load_render_passes, RenderPasses},
    shaders::{
        flat::{self, vs::ty::Data},
        shader_loader::{load_shaders, LoadedShaders},
    },
    swapchain::create_swapchain,
};

pub struct VulkanState {
    pub device: Arc<Device>,
    pub surface: Arc<Surface>,
    pub descriptor_set_allocator: StandardDescriptorSetAllocator,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub queue: Arc<Queue>,
    pub vertex_buffers: VertexBuffers,
    pub index_buffers: IndexBuffers,
    pub shaders: Arc<LoadedShaders>,
    pub swapchain: Arc<Swapchain>,
    pub swapchain_images: Vec<Arc<SwapchainImage>>,
    pub viewport: Viewport,
    pub attachment_images: Arc<AttachmentImageMap>,
    pub memory_allocator: Arc<GenericMemoryAllocator<Arc<FreeListAllocator>>>,
    pub render_passes: Arc<RenderPasses>,
    pub pipelines: Arc<Pipelines>,
    pub frame_buffers: Arc<FrameBufferMap>,
    pub uniform_buffer: Arc<CpuBufferPool<Data>>,
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

    let (swapchain, swapchain_images) = create_swapchain(device.clone(), surface.clone());

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

    let uniform_buffer = Arc::new(CpuBufferPool::<flat::vs::ty::Data>::new(
        memory_allocator.clone(),
        BufferUsage {
            uniform_buffer: true,
            ..BufferUsage::empty()
        },
        MemoryUsage::Upload,
    ));

    let shaders = load_shaders(device.clone());
    // let render_pass = solid_draw_pass(device.clone(), swapchain.image_format()).unwrap();

    // let _pipeline = solid_draw_pipeline(render_pass.clone(), device.clone(), shaders).unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let render_passes = load_render_passes(device.clone(), swapchain.image_format());
    let pipelines = load_pipelines(render_passes.clone(), device.clone(), shaders.clone());

    let dimensions = swapchain_images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    let attachment_images = create_attachment_images(
        memory_allocator.clone(),
        dimensions,
        swapchain.image_format(),
    );

    let frame_buffers = create_frame_buffers(render_passes.clone(), attachment_images.clone());

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    // let vertices: Vec<Vertex> = vec![];

    // let mut vertex_buffer = CpuAccessibleBuffer::from_iter(
    //     &memory_allocator,
    //     BufferUsage {
    //         vertex_buffer: true,
    //         ..BufferUsage::empty()
    //     },
    //     false,
    //     vertices,
    // )
    // .unwrap();

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
            swapchain_images,
            attachment_images,
            frame_buffers,
            memory_allocator,
            render_passes,
            pipelines,
            uniform_buffer,
        },
        event_loop,
    )
}
