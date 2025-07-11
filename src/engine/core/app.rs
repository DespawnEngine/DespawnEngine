use crate::engine::{
    display::load_icon,
    vswapchain::{create_swapchain, window_size_dependent_setup},
    vulkan::{create_device_and_queue, create_instance},
};
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, Subbuffer, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo,
    },
    image::Image,
    pipeline::graphics::viewport::{Viewport, ViewportState},
    swapchain::{self},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};
use vulkano::memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator};
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::{
    input_assembly::InputAssemblyState,
    color_blend::ColorBlendState,
    rasterization::RasterizationState,
    multisample::MultisampleState,
};
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::render_pass::Subpass;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::Window,
};
use crate::engine::vertex::MyVertex;

// `App` holds the state of the application, including all Vulkan objects that need to persist between frames.
pub struct App {
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
    vertex_buffer: Option<Subbuffer<[MyVertex]>>,
    pipeline: Option<Arc<GraphicsPipeline>>,
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
            vertex_buffer: None,
            pipeline: None,
        }
    }
}

impl ApplicationHandler for App {
    // This is called once when the application starts.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the window
        let window = {
            let window_attributes = Window::default_attributes()
                .with_title("Despawn Engine")
                .with_window_icon(Some(load_icon("assets/icon.png")));
            Arc::new(event_loop.create_window(window_attributes).unwrap())
        };
        self.window = Some(window.clone());

        // Initialize Vulkan
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


        // Creating vertices for the triangle.
        let vertex_data = [
            MyVertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
            MyVertex { position: [0.0, 0.5], color: [0.0, 1.0, 0.0] },
            MyVertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },
        ];

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(
            self.device.as_ref().unwrap().clone(),
        ));

        let vertex_buffer: Subbuffer<[MyVertex]> = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertex_data,
        ).unwrap();

        self.vertex_buffer = Some(vertex_buffer);
        // End of creating vertices for the triangle.


        // Define a simple render pass with one color attachment
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

        // Loading the vertex and fragment shaders
        mod vs {
            vulkano_shaders::shader! {
        ty: "vertex",
        path: "assets/shaders/first_triangle/vertex.glsl"
        }
        }
        mod fs {
            vulkano_shaders::shader! {
        ty: "fragment",
        path: "assets/shaders/first_triangle/fragment.glsl"
        }
        }

        let vs = vs::load(device.clone()).expect("failed to create shader module");
        let fs = fs::load(device.clone()).expect("failed to create shader module");

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };

        // Creating the graphics pipeline
        let pipeline = {
            let vs = vs.entry_point("main").unwrap();
            let fs = fs.entry_point("main").unwrap();

            let vertex_input_state = MyVertex::per_vertex().definition(&vs).unwrap();

            let stages = [
                PipelineShaderStageCreateInfo::new(vs),
                PipelineShaderStageCreateInfo::new(fs),
            ];

            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            )
                .unwrap();

            let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

            GraphicsPipeline::new(
                device.clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState {
                        viewports: [viewport].into_iter().collect(),
                        ..Default::default()
                    }),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState::default(),
                    )),
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )
                .unwrap()
        };
        self.pipeline = Some(pipeline);


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
            // On resize, simply flag that the swapchain needs to be recreated.
            // The actual recreation happens at the beginning of the next `RedrawRequested` event.
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

                // Ensure the GPU resources from N-2 frames ago are freed
                previous_frame_end.cleanup_finished();

                // Acquire the next available image from the swapchain.
                let (image_i, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        // This error indicates the surface properties have changed, and we need to recreate.
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

                // Build the command buffer for this frame's drawing commands.
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
                            ..RenderPassBeginInfo::framebuffer(framebuffers[image_i as usize].clone())
                        },
                        SubpassBeginInfo {
                            contents: SubpassContents::Inline,
                            ..Default::default()
                        },
                    )
                    .unwrap()
                    .bind_pipeline_graphics(self.pipeline.as_ref().unwrap().clone())
                    .unwrap()
                    .bind_vertex_buffers(0, self.vertex_buffer.as_ref().unwrap().clone())
                    .unwrap();

                unsafe {
                    cmd_buffer_builder
                        .draw(self.vertex_buffer.as_ref().unwrap().len() as u32, 1, 0, 0)
                        .unwrap();
                }

                cmd_buffer_builder
                    .end_render_pass(SubpassEndInfo::default())
                    .unwrap();


                let command_buffer = cmd_buffer_builder.build().unwrap();

                // Chain all GPU operations together:
                // 1. Wait for the previous frame to finish.
                // 2. Wait for the swapchain image to be acquired.
                // 3. Execute our new command buffer.
                // 4. Present the rendered image to the screen.
                // 5. Signal a fence when done.
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

                // Handle the result of the submission.
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
