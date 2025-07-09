use crate::engine::{
    display::load_icon,
    vswapchain::{create_swapchain, window_size_dependent_setup},
    vulkan::{create_device_and_queue, create_instance},
};
use std::sync::Arc;
use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo,
    },
    image::Image,
    pipeline::graphics::viewport::Viewport,
    swapchain::{self},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

mod engine {
    pub mod display;
    pub mod vswapchain;
    pub mod vulkan;
}

// Application state
struct App {
    window: Option<Arc<Window>>,
    surface: Option<Arc<swapchain::Surface>>,
    device: Option<Arc<vulkano::device::Device>>,
    queue: Option<Arc<vulkano::device::Queue>>,
    swapchain: Option<Arc<swapchain::Swapchain>>,
    images: Option<Vec<Arc<Image>>>,
    render_pass: Option<Arc<vulkano::render_pass::RenderPass>>,
    viewport: Viewport,
    framebuffers: Option<Vec<Arc<vulkano::render_pass::Framebuffer>>>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    command_buffer_allocator: Option<Arc<StandardCommandBufferAllocator>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            surface: None,
            device: None,
            queue: None,
            swapchain: None,
            images: None,
            render_pass: None,
            viewport: Viewport {
                offset: [0.0, 0.0],
                extent: [0.0, 0.0],
                depth_range: 0.0..=1.0,
            },
            framebuffers: None,
            recreate_swapchain: false,
            previous_frame_end: None,
            command_buffer_allocator: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window_attributes = Window::default_attributes()
                .with_title("DespawnEngine")
                .with_window_icon(Some(load_icon("assets/icon.png")));
            Arc::new(event_loop.create_window(window_attributes).unwrap())
        };
        self.window = Some(window.clone());

        let instance = create_instance(event_loop);
        let surface = swapchain::Surface::from_window(instance.clone(), window.clone()).unwrap();
        self.surface = Some(surface.clone());

        let (device, queue) = create_device_and_queue(instance, surface.clone());
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());

        let (swapchain, images) =
            create_swapchain(device.clone(), surface.clone(), window.inner_size().into());
        self.swapchain = Some(swapchain.clone());
        self.images = Some(images.clone());

        let command_buffer_allocator =
            Arc::new(StandardCommandBufferAllocator::new(device.clone(), Default::default()));
        self.command_buffer_allocator = Some(command_buffer_allocator);

        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();
        self.render_pass = Some(render_pass.clone());

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut self.viewport);
        self.framebuffers = Some(framebuffers);
        self.recreate_swapchain = false;
        self.previous_frame_end = Some(sync::now(device.clone()).boxed());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_) => {
                self.recreate_swapchain = true;
            }
            WindowEvent::RedrawRequested => {
                if self.previous_frame_end.is_none() {
                    return;
                }

                let window = self.window.as_ref().unwrap();
                let device = self.device.as_ref().unwrap();
                let queue = self.queue.as_ref().unwrap();
                let mut swapchain = self.swapchain.as_ref().unwrap().clone();
                let render_pass = self.render_pass.as_ref().unwrap();
                let command_buffer_allocator = self.command_buffer_allocator.as_ref().unwrap();

                if self.recreate_swapchain {
                    let image_extent: [u32; 2] = window.inner_size().into();
                    if image_extent.contains(&0) {
                        return;
                    }
                    let (new_swapchain, new_images) =
                        match swapchain.recreate(swapchain::SwapchainCreateInfo {
                            image_extent,
                            ..swapchain.create_info()
                        }) {
                            Ok(r) => r,
                            Err(e) => panic!("Failed to recreate swapchain: {e}"),
                        };
                    self.swapchain = Some(new_swapchain.clone());
                    swapchain = new_swapchain;
                    self.images = Some(new_images.clone());
                    self.framebuffers = Some(window_size_dependent_setup(
                        &new_images,
                        render_pass.clone(),
                        &mut self.viewport,
                    ));
                    self.recreate_swapchain = false;
                }

                let framebuffers = self.framebuffers.as_ref().unwrap();
                let mut previous_frame_end = self.previous_frame_end.take().unwrap();
                previous_frame_end.cleanup_finished();

                let (image_i, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            self.recreate_swapchain = true;
                            self.previous_frame_end = Some(previous_frame_end);
                            return;
                        }
                        Err(e) => panic!("failed to acquire next image: {e}"),
                    };


                if suboptimal {
                    self.recreate_swapchain = true;
                }

                let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
                    command_buffer_allocator.clone(),
                    queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                cmd_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.68, 1.0, 1.0].into())],
                            ..RenderPassBeginInfo::framebuffer(
                                framebuffers[image_i as usize].clone(),
                            )
                        },
                        SubpassBeginInfo {
                            contents: SubpassContents::Inline,
                            ..Default::default()
                        },
                    )
                    .unwrap()
                    .end_render_pass(SubpassEndInfo::default())
                    .unwrap();

                let command_buffer = cmd_buffer_builder.build().unwrap();
                let future = previous_frame_end
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        queue.clone(),
                        swapchain::SwapchainPresentInfo::swapchain_image_index(
                            swapchain.clone(),
                            image_i,
                        ),
                    )
                    .then_signal_fence_and_flush();

                match future.map_err(Validated::unwrap) {
                    Ok(future) => {
                        self.previous_frame_end = Some(future.boxed());
                    }
                    Err(VulkanError::OutOfDate) => {
                        self.recreate_swapchain = true;
                        self.previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("failed to flush future: {e}");
                        self.previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
