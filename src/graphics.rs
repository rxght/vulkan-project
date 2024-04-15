pub mod bindable;
pub mod drawable;
pub mod pipeline;
pub mod shaders;
pub mod utils;

use std::cmp::min;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, Weak};
use vulkano::command_buffer::allocator::StandardCommandBufferAlloc;
use vulkano::command_buffer::{PrimaryAutoCommandBuffer, RenderPassBeginInfo};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::format::{ClearValue, FormatFeatures};
use vulkano::image::{AttachmentImage, ImageTiling};
use vulkano::render_pass::SubpassDependency;

use self::bindable::Bindable;
use self::drawable::{Drawable, DrawableEntry, DrawableSharedPart, GenericDrawable};
use vulkano::sync::{AccessFlags, PipelineStages};
use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{
        view::{ImageView, ImageViewCreateInfo},
        ImageAspects, ImageLayout, ImageSubresourceRange, ImageUsage, SampleCount, SwapchainImage,
    },
    instance::{
        debug::{DebugUtilsMessenger, DebugUtilsMessengerCreateInfo, ValidationFeatureEnable},
        Instance, InstanceCreateInfo, InstanceExtensions,
    },
    memory::allocator::StandardMemoryAllocator,
    pipeline::graphics::viewport::Viewport,
    render_pass::{
        AttachmentDescription, AttachmentReference, Framebuffer, FramebufferCreateInfo, LoadOp,
        RenderPass, RenderPassCreateInfo, StoreOp, SubpassDescription,
    },
    sampler::ComponentMapping,
    swapchain::{
        acquire_next_image, ColorSpace, CompositeAlpha, Surface, Swapchain, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture, Sharing},
    Version, VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const IN_FLIGHT_COUNT: usize = 2;

// If true MAILBOX will always be used if available.
// If false FIFO will be preferred.
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

const ENABLED_FEATURES: Features = Features {
    dynamic_rendering: true,
    ..Features::empty()
};

const ENABLED_VALIDATION_FEATURES: [ValidationFeatureEnable; 1] =
    [ValidationFeatureEnable::BestPractices];

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

#[derive(Default)]
struct Queues {
    graphics_queue: Option<Arc<Queue>>,
    present_queue: Option<Arc<Queue>>,
    transfer_queue: Option<Arc<Queue>>,
}

#[derive(Default)]
struct QueueIndices {
    graphics_queue: Option<u32>,
    present_queue: Option<u32>,
    transfer_queue: Option<u32>,
}

impl QueueIndices {
    fn is_complete(&self) -> bool {
        self.graphics_queue.is_some()
            && self.present_queue.is_some()
            && self.transfer_queue.is_some()
    }
}

pub struct Graphics {
    //library: Arc<VulkanLibrary>,
    //instance: Arc<Instance>,
    //debug_messenger: Option<DebugUtilsMessenger>,
    surface: Arc<Surface>,
    window: Arc<Window>,
    //physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Queues,

    allocator: StandardMemoryAllocator,
    cmd_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,

    swapchain: Arc<Swapchain>,
    //swapchain_images: Vec<Arc<SwapchainImage>>,
    main_render_pass: Arc<RenderPass>,
    //depth_buffer: Vec<Arc<ImageView<AttachmentImage>>>,
    framebuffers: Vec<Arc<Framebuffer>>,

    shared_data_map: HashMap<fn(&Graphics) -> Vec<Arc<dyn Bindable>>, Weak<DrawableSharedPart>>, // THIS SHOULD BE MOVED
    registered_drawables: Vec<Weak<GenericDrawable>>,        // THIS SHOULD BE MOVED

    utils: OnceLock<utils::Utils>,

    main_command_buffer: Option<PrimaryAutoCommandBuffer<StandardCommandBufferAlloc>>,
    futures: Vec<Option<Box<dyn GpuFuture>>>,
    inflight_index: u32,
    framebuffer_index: u32,
}

impl Graphics {
    pub fn new() -> (Graphics, EventLoop<()>) {
        let library = VulkanLibrary::new().expect("Vulkan library is not installed.");

        let instance = create_instance(library.clone());

        //let debug_messenger = create_debug_messenger(instance.clone());

        let (event_loop, surface) = create_window(instance.clone());

        let physical_device = create_physical_device(instance.clone(), surface.clone());

        let (device, queues) = create_logical_device(physical_device.clone(), surface.clone());

        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

        let cmd_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

        let (swapchain, swapchain_images) = create_swapchain(device.clone(), surface.clone());

        println!("Swapchain is using {:?} images.", swapchain.image_count());

        let swapchain_image_views = create_image_views(&swapchain_images, swapchain.clone());

        let (depth_buffers, depth_format) =
            create_depth_buffer(device.clone(), swapchain.clone(), &memory_allocator);

        let main_render_pass =
            create_main_render_pass(device.clone(), swapchain.image_format(), depth_format);

        let framebuffers = create_framebuffers(
            &swapchain_image_views,
            main_render_pass.clone(),
            &depth_buffers,
        );

        let mut futures = Vec::with_capacity(IN_FLIGHT_COUNT);
        futures.resize_with(IN_FLIGHT_COUNT, || Some(sync::now(device.clone()).boxed()));

        let window = surface.object().unwrap().clone().downcast().unwrap();

        #[allow(unused_mut)]
        let mut gfx = Graphics {
            //library: library,
            //instance: instance,
            //debug_messenger: None,
            surface: surface,
            window: window,
            //physical_device: physical_device,
            device: device,
            queues: queues,

            allocator: memory_allocator,
            cmd_allocator: cmd_allocator,
            descriptor_set_allocator: descriptor_set_allocator,

            swapchain: swapchain,
            //swapchain_images: swapchain_images,
            main_render_pass: main_render_pass,
            framebuffers: framebuffers,

            shared_data_map: HashMap::new(),
            registered_drawables: Vec::new(),

            utils: OnceLock::new(),

            main_command_buffer: None,
            futures: futures,
            inflight_index: 0,
            framebuffer_index: 0,
        };

        _ = gfx.utils.set(utils::Utils::new(&gfx));

        (gfx, event_loop)
    }

    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
    }
    pub fn get_main_render_pass(&self) -> Arc<RenderPass> {
        self.main_render_pass.clone()
    }
    pub fn get_allocator(&self) -> &StandardMemoryAllocator {
        &self.allocator
    }
    pub fn get_shared_data_map(&self) -> &HashMap<fn(&Graphics) -> Vec<Arc<dyn Bindable>>, Weak<DrawableSharedPart>> {
        &self.shared_data_map
    }
    pub fn get_swapchain_format(&self) -> Format {
        self.swapchain.image_format()
    }
    pub fn get_descriptor_set_allocator(&self) -> &StandardDescriptorSetAllocator {
        &self.descriptor_set_allocator
    }
    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone()
    }
    pub fn graphics_queue(&self) -> Arc<Queue> {
        self.queues.graphics_queue.clone().unwrap()
    }
    pub fn get_cmd_allocator(&self) -> &StandardCommandBufferAllocator {
        &self.cmd_allocator
    }
    pub const fn get_in_flight_count(&self) -> usize {
        IN_FLIGHT_COUNT
    }
    pub fn get_in_flight_index(&self) -> usize {
        self.inflight_index as usize
    }
    pub fn get_utils(&self) -> &utils::Utils {
        self.utils.get().unwrap()
    }

    pub fn recreate_command_buffer(&mut self) {
        let mut builder = AutoCommandBufferBuilder::primary(
            &self.cmd_allocator,
            self.queues
                .graphics_queue
                .as_ref()
                .unwrap()
                .queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: self.swapchain.image_extent().map(|int| int as f32),
            depth_range: 0.0..1.0,
        };

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    render_pass: self.main_render_pass.clone(),
                    clear_values: vec![
                        Some(ClearValue::Float([0.0, 0.0, 0.0, 1.0])),
                        Some(ClearValue::Depth(1.0)),
                    ],
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[self.framebuffer_index as usize].clone(),
                    )
                },
                vulkano::command_buffer::SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [viewport.clone()]);

        for drawable in self.registered_drawables.iter().filter_map(|p| p.upgrade()) {
            for bindable in drawable.get_bindables() {
                bindable.bind(&self, &mut builder, drawable.get_pipeline_layout());
            }

            for bindable in drawable.get_shared_bindables() {
                bindable.bind(&self, &mut builder, drawable.get_pipeline_layout());
            }

            builder.bind_pipeline_graphics(drawable.get_pipeline());
            builder
                .draw_indexed(drawable.get_index_count(), 1, 0, 0, 0)
                .unwrap();
        }

        builder.end_render_pass().unwrap();
        self.main_command_buffer = Some(builder.build().unwrap());
    }

    pub fn draw_frame(&mut self) {
        self.futures[self.inflight_index as usize]
            .as_mut()
            .unwrap()
            .cleanup_finished();

        let (image_index, suboptimal, acquire_future) =
            acquire_next_image(self.swapchain.clone(), None).unwrap();

        self.framebuffer_index = image_index;

        match self.main_command_buffer.as_ref() {
            None => self.recreate_command_buffer(),
            _ => {}
        };

        let new_future = self.futures[self.inflight_index as usize]
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(
                self.queues.graphics_queue.clone().unwrap(),
                self.main_command_buffer.take().unwrap(),
            )
            .unwrap()
            .then_swapchain_present(
                self.queues.graphics_queue.clone().unwrap(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush();

        match new_future {
            Ok(future) => {
                self.futures[self.inflight_index as usize] = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain();
                self.futures[self.inflight_index as usize] =
                    Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                self.futures[self.inflight_index as usize] =
                    Some(sync::now(self.device.clone()).boxed());
            }
        };

        if suboptimal {
            self.recreate_swapchain();
        }
        self.inflight_index = (self.inflight_index + 1) % IN_FLIGHT_COUNT as u32;
    }

    pub fn register_drawable(&mut self, drawable_entry: &mut DrawableEntry) {
        if drawable_entry.registered_uid.is_some() {
            return;
        }

        drawable_entry.registered_uid = Some(self.registered_drawables.len() as u32);
        self.registered_drawables.push(drawable_entry.get_weak());
    }

    pub fn unregister_drawable(&mut self, drawable_entry: &mut DrawableEntry) {
        match drawable_entry.registered_uid {
            Some(idx) => match self.registered_drawables.get_mut(idx as usize) {
                Some(weak) => *weak = Weak::new(),
                None => _ = dbg!("[WARN] Tried to unregister an entry that was out of bounds."),
            },
            None => _ = dbg!("[WARN] Tried to unregister an entry that wasn't registered."),
        }
    }

    pub fn recreate_swapchain(&mut self) {
        let capabilities = self
            .device
            .physical_device()
            .surface_capabilities(self.surface.as_ref(), Default::default())
            .unwrap();

        let extent: [u32; 2] = match capabilities.current_extent {
            Some(current) => current,
            None => {
                let window: &Window = self.surface.object().unwrap().downcast_ref().unwrap();
                let framebuffer_extent = window.inner_size();
                let width = framebuffer_extent.width;
                let height = framebuffer_extent.height;
                [
                    width.clamp(
                        capabilities.min_image_extent[0],
                        capabilities.max_image_extent[0],
                    ),
                    height.clamp(
                        capabilities.min_image_extent[1],
                        capabilities.max_image_extent[1],
                    ),
                ]
            }
        };

        let create_info = SwapchainCreateInfo {
            image_extent: extent,
            ..self.swapchain.create_info()
        };

        let (swapchain, swapchain_images) = self.swapchain.recreate(create_info).unwrap();

        let image_views = create_image_views(&swapchain_images, swapchain.clone());

        let (depth_buffers, _) =
            create_depth_buffer(self.device.clone(), self.swapchain.clone(), &self.allocator);

        let framebuffers =
            create_framebuffers(&image_views, self.main_render_pass.clone(), &depth_buffers);

        self.swapchain = swapchain;
        self.framebuffers = framebuffers;

        self.utils.get().unwrap().recreate(&self);
    }
}

fn create_instance(library: Arc<VulkanLibrary>) -> Arc<Instance> {
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

    Instance::new(library.clone(), create_info).expect("Failed to create instance!")
}

fn create_debug_messenger(instance: Arc<Instance>) -> DebugUtilsMessenger {
    unsafe {
        DebugUtilsMessenger::new(
            instance,
            DebugUtilsMessengerCreateInfo::user_callback(Arc::new(
                |msg: &vulkano::instance::debug::Message<'_>| {
                    println!("DEBUG MESSENGER!!!! {}", msg.description);
                },
            )),
        )
        .unwrap()
    }
}

fn create_window(instance: Arc<Instance>) -> (EventLoop<()>, Arc<Surface>) {
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 400))
        .with_resizable(true)
        .with_title("Batako")
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to create window surface!");
    (event_loop, surface)
}

fn create_physical_device(instance: Arc<Instance>, surface: Arc<Surface>) -> Arc<PhysicalDevice> {
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("No appropriate physical device found!")
        .filter(|p| is_device_suitable(p.clone(), surface.clone()))
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

fn is_device_suitable(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>) -> bool {
    (physical_device.api_version() >= Version::V1_3
        || physical_device.supported_extensions().khr_dynamic_rendering)
        && physical_device
            .supported_extensions()
            .contains(&DEVICE_EXTENSIONS)
        && {
            let indices = find_queue_indices(physical_device.clone(), surface.clone());
            indices.graphics_queue.is_some() && indices.present_queue.is_some()
        }
}

fn find_queue_indices(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>) -> QueueIndices {
    let mut indices = QueueIndices::default();

    for (i, properties) in physical_device.queue_family_properties().iter().enumerate() {
        let flags = properties.queue_flags;
        if indices.graphics_queue.is_none() && flags.contains(QueueFlags::GRAPHICS) {
            indices.graphics_queue = Some(i as u32);
        }
        if indices.present_queue.is_none()
            && physical_device
                .surface_support(i as u32, &surface)
                .unwrap_or(false)
        {
            indices.present_queue = Some(i as u32);
        }
        if indices.transfer_queue.is_none()
            && flags.contains(QueueFlags::TRANSFER)
            && !(flags.contains(QueueFlags::GRAPHICS))
        {
            indices.transfer_queue = Some(i as u32);
        }
        if indices.is_complete() {
            break;
        }
    }
    indices
}

fn create_logical_device(
    physical_device: Arc<PhysicalDevice>,
    surface: Arc<Surface>,
) -> (Arc<Device>, Queues) {
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
        enabled_features: ENABLED_FEATURES,
        queue_create_infos: index_set
            .iter()
            .map(|p| QueueCreateInfo {
                queue_family_index: *p,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };

    let (device, mut queue_iter) = Device::new(physical_device.clone(), create_info)
        .expect("Failed to create logical device!");

    let mut queues = Queues::default();

    queues.graphics_queue = queue_iter.next();

    if !index_set.contains(&indices.present_queue.unwrap()) {
        dbg!("Forced to use a dedicated present queue,");
        queues.present_queue = queue_iter.next();
    } else {
        queues.present_queue = queues.graphics_queue.clone();
    }

    if indices.transfer_queue.is_some() && !index_set.contains(&indices.transfer_queue.unwrap()) {
        dbg!("Found support for dedicated transfer queue.");
        queues.transfer_queue = queue_iter.next();
    } else {
        queues.transfer_queue = queues.graphics_queue.clone();
    }

    (device, queues)
}

fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    let (capabilities, formats, present_modes) = (
        device
            .physical_device()
            .surface_capabilities(surface.as_ref(), Default::default())
            .unwrap(),
        device
            .physical_device()
            .surface_formats(surface.as_ref(), Default::default())
            .unwrap(),
        device
            .physical_device()
            .surface_present_modes(surface.as_ref())
            .unwrap(),
    );

    let surface_format = formats
        .iter()
        .find(|(format, color_space)| {
            *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
        })
        .unwrap_or(formats.first().unwrap());

    let extent: [u32; 2] = match capabilities.current_extent {
        Some(current) => current,
        None => {
            let window: &Window = surface.object().unwrap().downcast_ref().unwrap();
            let framebuffer_extent = window.inner_size();
            let width = framebuffer_extent.width;
            let height = framebuffer_extent.height;
            [
                width.clamp(
                    capabilities.min_image_extent[0],
                    capabilities.max_image_extent[0],
                ),
                height.clamp(
                    capabilities.min_image_extent[1],
                    capabilities.max_image_extent[1],
                ),
            ]
        }
    };

    use vulkano::swapchain::PresentMode;
    let present_mode = present_modes
        .min_by_key(|p| match *p {
            PresentMode::Mailbox => {
                if PREFER_MAILBOX_PRESENT_MODE {
                    0
                } else {
                    2
                }
            }
            PresentMode::Fifo => 1,

            PresentMode::FifoRelaxed => 3,
            PresentMode::Immediate => 4,
            _ => 5,
        })
        .unwrap();

    let indices = find_queue_indices(device.physical_device().clone(), surface.clone());
    let image_sharing = if indices.graphics_queue == indices.present_queue {
        Sharing::Exclusive
    } else {
        Sharing::Concurrent(smallvec::smallvec![
            indices.graphics_queue.unwrap(),
            indices.present_queue.unwrap()
        ])
    };

    let create_info = SwapchainCreateInfo {
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
        composite_alpha: capabilities
            .supported_composite_alpha
            .into_iter()
            .min_by_key(|p| match *p {
                CompositeAlpha::Opaque => 0,
                _ => 1,
            })
            .unwrap(),
        present_mode: present_mode,
        clipped: true,
        ..Default::default()
    };

    Swapchain::new(device.clone(), surface.clone(), create_info)
        .expect("Failed to create Swapchain!")
}

fn create_image_views(
    images: &Vec<Arc<SwapchainImage>>,
    swapchain: Arc<Swapchain>,
) -> Vec<Arc<ImageView<SwapchainImage>>> {
    images
        .iter()
        .map(|image| {
            ImageView::new(
                image.clone(),
                ImageViewCreateInfo {
                    view_type: vulkano::image::ImageViewType::Dim2d,
                    format: Some(swapchain.image_format()),
                    component_mapping: ComponentMapping::identity(),
                    subresource_range: ImageSubresourceRange {
                        aspects: ImageAspects::COLOR,
                        mip_levels: 0..1,
                        array_layers: 0..1,
                    },
                    usage: ImageUsage::COLOR_ATTACHMENT,
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect()
}

fn create_main_render_pass(
    device: Arc<Device>,
    swapchain_format: Format,
    depth_format: Format,
) -> Arc<RenderPass> {
    let attachments = vec![
        AttachmentDescription {
            format: Some(swapchain_format),
            samples: SampleCount::Sample1,
            load_op: LoadOp::Clear,
            store_op: StoreOp::Store,
            stencil_load_op: LoadOp::DontCare,
            stencil_store_op: StoreOp::DontCare,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::PresentSrc,
            ..Default::default()
        },
        AttachmentDescription {
            format: Some(depth_format),
            samples: SampleCount::Sample1,
            load_op: LoadOp::Clear,
            store_op: StoreOp::Store,
            stencil_load_op: LoadOp::DontCare,
            stencil_store_op: StoreOp::DontCare,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::DepthStencilAttachmentOptimal,
            ..AttachmentDescription::default()
        },
    ];

    let color_attachment_refs = vec![Some(AttachmentReference {
        attachment: 0,
        layout: ImageLayout::ColorAttachmentOptimal,
        ..Default::default()
    })];

    let depth_attachment_ref = AttachmentReference {
        attachment: 1,
        layout: ImageLayout::DepthStencilAttachmentOptimal,
        //aspects: ImageAspects::DEPTH,
        ..Default::default()
    };

    let mut subpasses = Vec::new();
    subpasses.push(SubpassDescription {
        color_attachments: color_attachment_refs,
        depth_stencil_attachment: Some(depth_attachment_ref),
        ..Default::default()
    });

    let create_info = RenderPassCreateInfo {
        attachments: attachments,
        subpasses: subpasses,
        dependencies: vec![SubpassDependency {
            src_subpass: None,
            dst_subpass: Some(0),
            src_stages: PipelineStages::COLOR_ATTACHMENT_OUTPUT
                | PipelineStages::EARLY_FRAGMENT_TESTS,
            dst_stages: PipelineStages::COLOR_ATTACHMENT_OUTPUT
                | PipelineStages::EARLY_FRAGMENT_TESTS,
            src_access: AccessFlags::empty(),
            dst_access: AccessFlags::COLOR_ATTACHMENT_WRITE
                | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ..Default::default()
        }],
        ..Default::default()
    };
    RenderPass::new(device.clone(), create_info).expect("Failed to create render pass!")
}

fn create_framebuffers(
    image_views: &Vec<Arc<ImageView<SwapchainImage>>>,
    render_pass: Arc<RenderPass>,
    depth_buffers: &Vec<Arc<ImageView<AttachmentImage>>>,
) -> Vec<Arc<Framebuffer>> {
    image_views
        .iter()
        .zip(depth_buffers)
        .map(|(image, depth_buffer)| {
            let create_info = FramebufferCreateInfo {
                attachments: vec![image.clone(), depth_buffer.clone()],
                extent: [0, 0],
                layers: 1,
                ..Default::default()
            };
            Framebuffer::new(render_pass.clone(), create_info).unwrap()
        })
        .collect()
}

fn select_image_format(
    device: Arc<Device>,
    tiling: ImageTiling,
    features: FormatFeatures,
    candidates: &[Format],
) -> Option<Format> {
    for format in candidates {
        let props = device.physical_device().format_properties(*format).unwrap();
        if tiling == ImageTiling::Optimal && props.optimal_tiling_features.contains(features) {
            return Some(*format);
        }
        if tiling == ImageTiling::Linear && props.linear_tiling_features.contains(features) {
            return Some(*format);
        }
    }
    None
}

fn create_depth_buffer(
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    allocator: &StandardMemoryAllocator,
) -> (Vec<Arc<ImageView<AttachmentImage>>>, Format) {
    let format_candidates = [
        Format::D16_UNORM,
        Format::D32_SFLOAT,
        Format::D16_UNORM_S8_UINT,
        Format::D24_UNORM_S8_UINT,
        Format::D32_SFLOAT_S8_UINT,
    ];

    let format = select_image_format(
        device.clone(),
        ImageTiling::Optimal,
        FormatFeatures::DEPTH_STENCIL_ATTACHMENT,
        &format_candidates,
    )
    .unwrap();

    let mut views = Vec::new();
    views.resize_with(swapchain.image_count() as usize, || {
        let image = AttachmentImage::with_usage(
            allocator,
            swapchain.image_extent(),
            format,
            ImageUsage::DEPTH_STENCIL_ATTACHMENT,
        )
        .unwrap();

        ImageView::new(
            image.clone(),
            ImageViewCreateInfo {
                format: Some(format),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..ImageViewCreateInfo::from_image(&image)
            },
        )
        .unwrap()
    });

    (views, format)
}
