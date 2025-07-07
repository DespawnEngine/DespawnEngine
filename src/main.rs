// Importing necessary Vulkano and winit modules for Vulkan API and window creation. A lot easier to read than the last commit, but might need to be adjusted.
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageAccess, SwapchainImage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{self, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError, SwapchainPresentInfo};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::{Version, VulkanLibrary};
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Icon, Window, WindowBuilder};
use std::sync::Arc;
use image::GenericImageView; // "image" crate uses this for loading images
use vulkano::format::Format;
use vulkano::image::ImageUsage;
use vulkano::swapchain::CompositeAlpha;

fn main() {

    // Load the window icon. Mostly a test and this code should be removed soon (or just like, made better).
    let icon = load_icon("assets/icon.png");

    // Create the main Vulkan instance
    let instance = {
        let library = VulkanLibrary::new().unwrap(); // Load Vulkan library
        let extensions = vulkano_win::required_extensions(&library); // Get extensions needed for window display

        // Create Vulkan instance with specific settings
        Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: extensions, // Enable window-related extensions
                enumerate_portability: true, // Think this allows macOS compatibility (MoltenVK)
                max_api_version: Some(Version::V1_1), // Uses Vulkan 1.1 or higher
                ..Default::default() // Default settings for every other option
            },
        )
            .unwrap()
    };

    let event_loop = EventLoop::new(); // Event loop to handle window events (like closing or resizing)
    let surface = WindowBuilder::new()
        .with_title(if cfg!(windows) { "Despawn Engine" } else { "DespawnEngine" })
        .with_window_icon(if cfg!(windows) { Some(load_icon("assets/icon.png")) } else { None })
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true, // Enable swapchain for rendering to window, still need to figure out what exactly this means
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type { // Prefer discrete GPU over integrated, CPU, etc.
            PhysicalDeviceType::DiscreteGpu => 0, // Dedicated GPU
            PhysicalDeviceType::IntegratedGpu => 1, // Integrated graphics
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No suitable physical device found");

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
        .unwrap();

    let queue = queues.next().unwrap();

    // Create a swapchain (manages images for rendering and display)
    let (mut swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        let (image_format, _) = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()
            .into_iter()
            .min_by_key(|(format, _color_space)| match format {
                Format::B8G8R8A8_SRGB => 0,
                _ => 1,
            })
            .unwrap();

        let composite_alpha = if surface_capabilities.supported_composite_alpha.opaque {
            CompositeAlpha::Opaque
        } else {
            CompositeAlpha::PreMultiplied
        };

        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
        let image_extent: [u32; 2] = window.inner_size().into();

        // New swapchain with specifics settings
        Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count,
                image_format: Some(image_format),
                image_extent,
                image_usage: ImageUsage {
                    color_attachment: true,
                    ..ImageUsage::empty()
                },
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap()
    };

    let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
        .unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                previous_frame_end
                    .as_mut()
                    .take()
                    .unwrap()
                    .cleanup_finished();

                // Recreate swapchain if needed
                if recreate_swapchain {
                    let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
                    let image_extent: [u32; 2] = window.inner_size().into();

                    // Recreate swapchain with new window size
                    let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                        image_extent,
                        ..swapchain.create_info()
                    }) {
                        Ok(r) => r,
                        Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                        Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                    };

                    swapchain = new_swapchain; // Update swapchain
                    framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport); // Recreate framebuffers for new images
                    recreate_swapchain = false;
                }

                // Acquire next image from swapchain for rendering
                let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true; // Mark swapchain for recreation
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

                // Recreate swapchain if not using ideal settings
                if suboptimal {
                    recreate_swapchain = true;
                }

                // This is the color to make the window (light blue because it's kind of like the minecraft sky, vaguely)
                let clear_values = vec![Some([0.0, 0.68, 1.0, 1.0].into())];

                let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
                    &command_buffer_allocator,
                    queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                    .unwrap();

                // Start rendering, clear the image, and end rendering
                cmd_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values, // Clear with specified color (showing a color)
                            ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
                        },
                        SubpassContents::Inline, // Render directly
                    )
                    .unwrap()
                    .end_render_pass()
                    .unwrap();

                // Finalize command buffer
                let command_buffer = cmd_buffer_builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future) // Wait for image to be ready
                    .then_execute(queue.clone(), command_buffer) // Run commands
                    .unwrap()
                    .then_swapchain_present(
                        queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                    )
                    .then_signal_fence_and_flush(); // Signal the completion

                // Handle the result of GPU operations
                match future {
                    Ok(future) => {
                        previous_frame_end = Some(Box::new(future) as Box<_>);
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<_>);
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<_>);
                    }
                }
            }
            _ => {}
        }
    });
}

// Helper function to create framebuffers based on window size
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>], // Swapchain images
    render_pass: Arc<RenderPass>, // RenderPass to hook
    viewport: &mut Viewport, // Viewport to update
) -> Vec<Arc<Framebuffer>> {

    // Get image dimensions and update viewport
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    // Create a framebuffer for each image
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
                .unwrap()
        })
        .collect::<Vec<_>>()
}

// Helper function for loading an icon for the window icon. Code will likely be changed, but I wanted to experiment to learn more.
fn load_icon(path: &str) -> Icon {
    // Load the image
    let image = image::open(path).expect("Failed to open icon file");

    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw(); // Convert to raw RGBA bytes

    // Create winit Icon
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
