pub mod drawable;

use std::{cmp::min, alloc::GlobalAlloc};
use std::sync::Arc;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::shader::{ShaderModule, ShaderStage, ShaderCreationError};
use vulkano::{
    memory::allocator::{GenericMemoryAllocator, FreeListAllocator},
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderingAttachmentInfo, RenderingInfo,
    },
    device::{
        physical::{PhysicalDeviceType, PhysicalDevice},
        Device, DeviceCreateInfo, DeviceExtensions, Features,
        QueueCreateInfo, QueueFlags, Queue
    },
    image::{
        view::ImageView, view::ImageViewCreateInfo, ImageAccess, ImageUsage, SwapchainImage,
        ImageSubresourceLayers, ImageSubresourceRange, ImageAspect, ImageAspects, SampleCount, ImageLayout
    },
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    instance::debug::{ValidationFeatureEnable, DebugUtilsMessenger, DebugUtilsMessengerCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            render_pass::PipelineRenderingCreateInfo,
            vertex_input::Vertex,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline,
    },
    render_pass::{
        LoadOp, StoreOp, RenderPass, RenderPassCreateInfo, AttachmentDescription,
        SubpassDescription, AttachmentReference, Framebuffer
    },
    swapchain::{
        acquire_next_image, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo, Surface, ColorSpace, SurfaceTransform, CompositeAlpha
    },
    sync::{self, FlushError, GpuFuture, Sharing},
    sampler::ComponentMapping,
    Version, VulkanLibrary, format::Format
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// If true MAILBOX will always be used if available.
// If false MAILBOX will be used only if FIFO is not available
const PREFER_MAILBOX_PRESENT_MODE: bool = false;

const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};

const INSTANCE_EXTENSIONS: InstanceExtensions = InstanceExtensions {
    ext_validation_features: true,
    ext_debug_utils: true,
    ..InstanceExtensions::empty()
};

const ENABLED_VALIDATION_FEATURES: [ValidationFeatureEnable; 1] = [
    ValidationFeatureEnable::BestPractices,
];

const VALIDATION_LAYERS: [&str; 1] = [
    "VK_LAYER_KHRONOS_validation",
];

#[derive(Default)]
struct Queues
{
    graphics_queue: Option<Arc<Queue>>,
    present_queue: Option<Arc<Queue>>,
    transfer_queue: Option<Arc<Queue>>,
}

#[derive(Default)]
struct QueueIndices
{
    graphics_queue: Option<u32>,
    present_queue: Option<u32>,
    transfer_queue: Option<u32>,
}

impl QueueIndices {
    fn is_complete(&self) -> bool
    {
        self.graphics_queue.is_some() &&
        self.present_queue.is_some() &&
        self.transfer_queue.is_some()
    }
}

pub struct Graphics
{
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    debug_messenger: DebugUtilsMessenger,
    event_loop: EventLoop<()>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Queues,
    swapchain: Arc<Swapchain>,
    swapchain_images: Vec<Arc<SwapchainImage>>,
    allocator: StandardMemoryAllocator,
    main_render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl Graphics
{
    pub fn new() -> Graphics
    {
        let library = VulkanLibrary::new().expect("Vulkan library is not installed.");
        
        let instance =
            create_instance(library.clone());

        let debug_messenger =
            create_debug_messenger(instance.clone());

        let (event_loop, surface) =
            create_window(instance.clone());

        let physical_device =
            create_physical_device(instance.clone(), surface.clone());
        
        let (device, queues) =
            create_logical_device(physical_device.clone(), surface.clone());
        
        let (swapchain, swapchain_images) =
            create_swapchain(device.clone(), surface.clone());
        
        let memory_allocator =
            StandardMemoryAllocator::new_default(device.clone());

        let swapchain_image_views =
            create_image_views(&swapchain_images, swapchain.clone());

        let main_render_pass =
            create_main_render_pass(device.clone(), swapchain.clone());

        let framebuffers =
            create_framebuffers(&swapchain_image_views, main_render_pass.clone());
        
        Graphics {
            library: library,
            instance: instance,
            debug_messenger: debug_messenger,
            event_loop: event_loop,
            surface: surface,
            physical_device: physical_device,
            device: device,
            queues: queues,
            swapchain: swapchain,
            swapchain_images: swapchain_images,
            allocator: memory_allocator,
            main_render_pass: main_render_pass,
            framebuffers: framebuffers,
        }
    }

}

fn create_instance(library: Arc<VulkanLibrary>) -> Arc<Instance>
{
    let required_extensions = vulkano_win::required_extensions(&library);

    let create_info = InstanceCreateInfo {
        application_name: Some(String::from("Rosten")),
        enabled_extensions: required_extensions.union(&INSTANCE_EXTENSIONS),
        enabled_layers: VALIDATION_LAYERS.iter().map(|p| String::from(*p)).collect(),
        enumerate_portability: false,
        max_api_version: None,
        enabled_validation_features: Vec::from_iter(ENABLED_VALIDATION_FEATURES.into_iter()),
        ..InstanceCreateInfo::default()
    };


    Instance::new(
        library.clone(),
        create_info
    )
    .expect("Failed to create instance!")
}

fn create_debug_messenger(instance: Arc<Instance>) -> DebugUtilsMessenger
{
    unsafe {
        DebugUtilsMessenger::new(
            instance,
            DebugUtilsMessengerCreateInfo::user_callback(Arc::new(
                |msg| {
                    println!("DEBUG MESSENGER!!!! {}", msg.description);
                }
            ))
        ).unwrap()
    }
}

fn create_window(instance: Arc<Instance>) -> (EventLoop<()>, Arc<Surface>)
{
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(900, 700))
        .with_resizable(false)
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to create window surface!");
    (event_loop, surface)
}

fn create_physical_device(instance: Arc<Instance>, surface: Arc<Surface>) -> Arc<PhysicalDevice>
{

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("No appropriate physical device found!")
        .filter(|p|
            is_device_suitable(p.clone(), surface.clone()))
        .min_by_key(|p| {
            // We assign a lower score to device types that are likely to be faster/better.
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            }
        })
        .expect("no suitable physical device found");

    // Some little debug infos.
    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    physical_device
}

fn is_device_suitable(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>) -> bool
{
    (
        physical_device.api_version() >= Version::V1_3 ||
        physical_device.supported_extensions().khr_dynamic_rendering
    ) &&
    physical_device.supported_extensions().contains(&DEVICE_EXTENSIONS) &&
    {
        let indices = find_queue_indices(physical_device.clone(), surface.clone());
        indices.graphics_queue.is_some() && indices.present_queue.is_some()
    }
}

fn find_queue_indices(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>) -> QueueIndices
{
    let mut indices = QueueIndices::default();

    for (i, properties) in
        physical_device.queue_family_properties().iter().enumerate()
    {
        let flags = properties.queue_flags;
        if indices.graphics_queue.is_none() && flags.contains(QueueFlags::GRAPHICS)
        {
            indices.graphics_queue = Some(i as u32);
        }
        if indices.present_queue.is_none() && 
            physical_device.surface_support(i as u32, &surface).unwrap_or(false)
        {
            indices.present_queue = Some(i as u32);
        }
        if indices.transfer_queue.is_none() && 
            flags.contains(QueueFlags::TRANSFER) && !(flags.contains(QueueFlags::GRAPHICS))
        {
            indices.transfer_queue = Some(i as u32);
        }
        if indices.is_complete() { break; }
    }
    indices
}

fn create_logical_device(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>) -> (Arc<Device>, Queues)
{
    let mut extensions = DEVICE_EXTENSIONS.clone();

    if physical_device.api_version() < Version::V1_3 {
        extensions.khr_dynamic_rendering = true;
    }

    let indices = find_queue_indices(physical_device.clone(), surface.clone());
    let mut index_set = vec![indices.graphics_queue.unwrap()];

    if !index_set.contains(&indices.present_queue.unwrap()) {
        index_set.push(indices.present_queue.unwrap());
    }

    if indices.transfer_queue.is_some() && !index_set.contains(&indices.transfer_queue.unwrap()) {
        index_set.push(indices.transfer_queue.unwrap());
    }

    let create_info = DeviceCreateInfo {
        enabled_extensions: extensions,
        enabled_features: Features::empty(),
        queue_create_infos: index_set.iter().map( |p| QueueCreateInfo{
            queue_family_index: *p,
            ..Default::default()
        }).collect(),
        ..Default::default()
    };

    let (device, mut queue_iter) =
        Device::new(physical_device.clone(), create_info).expect("Failed to create logical device!");
    
    let mut queues = Queues::default();

    queues.graphics_queue = queue_iter.next();

    if !index_set.contains(&indices.present_queue.unwrap()) {
        dbg!("Forced to use a dedicated present queue,");
        queues.present_queue = queue_iter.next();
    }
    else {
        queues.present_queue = queues.graphics_queue.clone();
    }

    if indices.transfer_queue.is_some() && !index_set.contains(&indices.transfer_queue.unwrap()) {
        dbg!("Found support for dedicated transfer queue.");
        queues.transfer_queue = queue_iter.next();
    }
    else {
        queues.transfer_queue = queues.graphics_queue.clone();
    }

    (device, queues)
}

fn create_swapchain(device: Arc<Device>, surface: Arc<Surface>) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>)
{
    let (capabilities, formats,
        present_modes) =
    (
        device.physical_device().surface_capabilities(surface.as_ref(), Default::default()).unwrap(),
        device.physical_device().surface_formats(surface.as_ref(), Default::default()).unwrap(),
        device.physical_device().surface_present_modes(surface.as_ref()).unwrap()
    );

    let surface_format = formats.iter().find(|(format, color_space)| {
        *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
    }).unwrap_or(formats.first().unwrap());

    let extent: [u32; 2] = match capabilities.current_extent
    {
        Some(current) => current,
        None => {
            let window: &Window = surface.object().unwrap().downcast_ref().unwrap();
            let framebuffer_extent = window.inner_size();
            let width = framebuffer_extent.width;
            let height = framebuffer_extent.height;
            [
                width.clamp(capabilities.min_image_extent[0], capabilities.max_image_extent[0]),
                height.clamp(capabilities.min_image_extent[1], capabilities.max_image_extent[1]),
            ]
        }
    };


    use vulkano::swapchain::PresentMode;
    let present_mode = present_modes.min_by_key(|p| {
        match *p {
            PresentMode::Mailbox => if PREFER_MAILBOX_PRESENT_MODE { 0 } else { 2 },
            PresentMode::Fifo => 1,

            PresentMode::FifoRelaxed => 3,
            PresentMode::Immediate => 4,
            _ => 5,
        }
    }).unwrap();

    let indices = find_queue_indices(device.physical_device().clone(), surface.clone());
    let image_sharing =
        if indices.graphics_queue == indices.present_queue { Sharing::Exclusive }
        else { Sharing::Concurrent(smallvec::smallvec!
            [indices.graphics_queue.unwrap(), indices.present_queue.unwrap()]
        )};

    let create_info = SwapchainCreateInfo
    {
        min_image_count: match capabilities.max_image_count {
            Some(max) => min(capabilities.min_image_count + 1, max),
            None => capabilities.min_image_count + 1,
        },
        image_format: Some(surface_format.0),
        image_color_space: surface_format.1,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: ImageUsage::COLOR_ATTACHMENT,
        image_sharing: image_sharing,
        pre_transform: capabilities.current_transform,
        composite_alpha: capabilities.supported_composite_alpha
            .into_iter().min_by_key(|p| match *p {
                CompositeAlpha::Opaque => 0,
                _ => 1,
            }).unwrap(),
        present_mode: present_mode,
        clipped: true,
        ..Default::default()
    };

    Swapchain::new(device.clone(), surface.clone(), create_info)
        .expect("Failed to create Swapchain!")
}

fn create_image_views(images: &Vec<Arc<SwapchainImage>>, swapchain: Arc<Swapchain>) -> Vec<Arc<ImageView<SwapchainImage>>>
{
    images.iter().map(|image| ImageView::new(image.clone(), ImageViewCreateInfo {
        view_type: vulkano::image::ImageViewType::Dim2d,
        format: Some(swapchain.image_format()),
        component_mapping: ComponentMapping::identity(),
        subresource_range: ImageSubresourceRange{
            aspects: ImageAspects::COLOR,
            mip_levels: 0..1,
            array_layers: 0..1,
        },
        usage: ImageUsage::COLOR_ATTACHMENT,
        ..Default::default()
    }).unwrap()).collect()
}

fn create_main_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass>
{
    let mut attachments = Vec::new();
    attachments.push(AttachmentDescription{
        format: Some(swapchain.image_format()),
        samples: SampleCount::Sample1,
        load_op: LoadOp::Clear,
        store_op: StoreOp::Store,
        stencil_load_op: LoadOp::DontCare,
        stencil_store_op: StoreOp::DontCare,
        initial_layout: ImageLayout::Undefined,
        final_layout: ImageLayout::PresentSrc,
        ..Default::default()
    });

    let color_attachment_ref = AttachmentReference {
        attachment: 0,
        layout: ImageLayout::ColorAttachmentOptimal,
        ..Default::default()
    };

    let mut subpasses = Vec::new();
    subpasses.push(SubpassDescription{
        color_attachments: vec![Some(color_attachment_ref)],
        ..Default::default()
    });

    let create_info = RenderPassCreateInfo {
        attachments: attachments,
        subpasses: subpasses,
        ..Default::default()
    };
    RenderPass::new(device.clone(), create_info).expect("Failed to create render pass!")
}

fn create_framebuffers(image_views: &Vec<Arc<ImageView<SwapchainImage>>>, render_pass: Arc<RenderPass>) -> Vec<Arc<Framebuffer>>
{
    image_views.iter().map(|image| {
        let create_info = FramebufferCreateInfo { 
            attachments: vec![image.clone()],
            extent: [0, 0],
            layers: 1,
            ..Default::default()
        };
        Framebuffer::new(render_pass.clone(), create_info).unwrap()
    }).collect()
}

pub fn create_shader_module(device: Arc<Device>) -> Arc<ShaderModule>
{
    let shader_path = "shaders/first.vert";
    let bytes =
        std::fs::read(shader_path).expect("Couldn't find the file specified");
    
    unsafe
    {
        ShaderModule::from_bytes(device.clone(), bytes.as_ref())
            .expect("Failed to create shader module!")
    }
}

pub fn test_draw()
{
    
}
