use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents};
use vulkano::render_pass::Framebuffer;
use vulkano::swapchain::{self, AcquireError, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::pipeline::graphics::viewport::Viewport;
use std::sync::Arc;
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::engine::vulkan::{create_instance, create_device_and_queue};
use crate::engine::vswapchain::{create_swapchain, window_size_dependent_setup};
use crate::engine::display::load_icon;

// Importing DespawnEngine engine modules. Work in progress moving things to each other.
mod engine {
    pub mod vulkan;
    pub mod vswapchain;
    pub mod display;
}

fn main() {
    // Load the window icon. Mostly a test and this code should be removed soon (or just like, made better).
    let icon = load_icon("assets/icon.png");

    let event_loop = EventLoop::new(); // Event loop to handle window events (like closing or resizing)
    let instance = create_instance();
    let surface = winit::window::WindowBuilder::new()
        .with_title(if cfg!(windows) { "Despawn Engine" } else { "DespawnEngine" })
        .with_window_icon(if cfg!(windows) { Some(load_icon("assets/icon.png")) } else { None })
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to create Vulkan surface");

    let (device, queue) = create_device_and_queue(instance.clone(), surface.clone());
    let (mut swapchain, images) = create_swapchain(device.clone(), surface.clone());

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
        .expect("Failed to create render pass");

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Ensure continuous polling for events
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
            Event::MainEventsCleared => { // Changed from RedrawEventsCleared to MainEventsCleared for winit 0.27.3 compatibility
                previous_frame_end
                    .as_mut()
                    .take()
                    .expect("No previous frame end")
                    .cleanup_finished();

                // Recreate swapchain if needed
                if recreate_swapchain {
                    let window = surface.object().unwrap().downcast_ref::<winit::window::Window>().unwrap();
                    let image_extent: [u32; 2] = window.inner_size().into();

                    // Recreate swapchain with new window size
                    let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                        image_extent,
                        ..swapchain.create_info()
                    }) {
                        Ok(r) => r,
                        Err(vulkano::swapchain::SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
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
                    .expect("Failed to create command buffer builder");

                // Start rendering, clear the image, and end rendering
                cmd_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values, // Clear with specified color (showing a color)
                            ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
                        },
                        SubpassContents::Inline, // Render directly
                    )
                    .expect("Failed to begin render pass")
                    .end_render_pass()
                    .expect("Failed to end render pass");

                // Finalize command buffer
                let command_buffer = cmd_buffer_builder.build().expect("Failed to build command buffer");

                let future = previous_frame_end
                    .take()
                    .expect("No previous frame end")
                    .join(acquire_future) // Wait for image to be ready
                    .then_execute(queue.clone(), command_buffer) // Run commands
                    .expect("Failed to execute command buffer")
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