use std::ops::Not;
use std::sync::Arc;
use std::time::{self, Instant};
use vulkano::image::{ImageType, sampler};

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocatorCreateInfo;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::{
    Validated, VulkanError,
    buffer::Subbuffer,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassContents, SubpassEndInfo, allocator::StandardCommandBufferAllocator,
    },
    image::Image,
    memory::allocator::StandardMemoryAllocator,
    pipeline::{
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    render_pass::Subpass,
    swapchain::{self},
    sync::{self, GpuFuture},
};
use winit::event::{DeviceEvent, DeviceId};
use winit::window::CursorGrabMode;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

use crate::engine::core::input::{InputState, KeyBind};
use crate::engine::core::user_settings::UserSettings;
use crate::engine::rendering::mvp::MVP;
use crate::engine::rendering::vertex::MyVertex;
use crate::engine::rendering::vswapchain::{create_swapchain, window_size_dependent_setup};
use crate::engine::rendering::vulkan::{create_device_and_queue, create_instance};
use crate::engine::rendering::{
    camera::Camera,
    display::{create_main_window, create_render_pass, create_vertex_buffer},
};
use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::ui::egui_integration::EguiStruct;

use image::io::Reader as ImageReader;
use std::io::Cursor;
use vulkano::command_buffer::CopyBufferToImageInfo;
use vulkano::format::Format;
use vulkano::image::ImageCreateInfo;
use vulkano::image::ImageUsage;
use vulkano::image::sampler::{Filter, Sampler, SamplerCreateInfo};
use vulkano::image::view::ImageView;

//
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
    egui: Option<EguiStruct>,
    mvp_buffer: Option<Subbuffer<MVP>>,
    descriptor_set_allocator: Option<Arc<StandardDescriptorSetAllocator>>,
    descriptor_set: Option<Arc<DescriptorSet>>,
    memory_allocator: Option<Arc<StandardMemoryAllocator>>,
    camera: Option<Camera>,
    input_state: Option<InputState>,
    user_settigns: Option<UserSettings>,
    last_frame_time: Option<std::time::Instant>,
    capture_cursor: bool,
    scene_manager: Option<SceneManager>, // MAIN GAME SCENE MANAGER
    texture: Option<Arc<vulkano::image::view::ImageView>>,
    sampler: Option<Arc<vulkano::image::sampler::Sampler>>,
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
            egui: None,
            mvp_buffer: None,
            descriptor_set_allocator: None,
            descriptor_set: None,
            memory_allocator: None,
            camera: None,
            input_state: None,
            user_settigns: None,
            last_frame_time: None,
            capture_cursor: true,
            scene_manager: None, // MAIN GAME SCENE MANAGER
            sampler: None,
            texture: None,
        }
    }
}

// This is called once when the application starts.
impl App {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window = create_main_window(event_loop);
        self.window = Some(window.clone());
    }
    fn create_vulkan(&mut self, event_loop: &ActiveEventLoop) {
        let instance = create_instance(event_loop);
        let surface = swapchain::Surface::from_window(
            instance.clone(),
            self.window.as_ref().unwrap().clone(),
        )
        .unwrap();
        self.surface = Some(surface.clone());

        let (device, queue) = create_device_and_queue(instance, surface.clone());
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());

        let (swapchain, images) = create_swapchain(
            device.clone(),
            surface.clone(),
            self.window.as_ref().unwrap().inner_size().into(),
        );
        self.swapchain = Some(swapchain.clone());
        self.images = Some(images.clone());

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        self.command_buffer_allocator = Some(command_buffer_allocator);

        // Define the render pass from display.rs
        let render_pass = create_render_pass(device.clone());
        self.render_pass = Some(render_pass.clone());

        // Creating vertices for the triangle, MVP buffer, and memory allocator for it.
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        self.memory_allocator = Some(memory_allocator.clone());

        let tex_bytes = include_bytes!("../../../assets/texture.png");
        let img = ImageReader::new(Cursor::new(tex_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();

        let (width, height) = img.dimensions();
        let img_data = img.into_raw();

        // Create the GPU image (empty)
        let texture_image = Image::new(
            memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent: [width, height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )
        .unwrap();

        // Upload pixels via a staging buffer
        let staging_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            img_data,
        )
        .unwrap();

        let mut builder = AutoCommandBufferBuilder::primary(
            self.command_buffer_allocator.as_ref().unwrap().clone(),
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                staging_buffer,
                texture_image.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        // Create the view & sampler
        let texture = ImageView::new_default(texture_image.clone()).unwrap();
        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Nearest, // Basically the same as nearest neighbor. Keeps sharp pixels.
                min_filter: Filter::Nearest,
                ..Default::default()
            },
        )
        .unwrap();

        self.texture = Some(texture);
        self.sampler = Some(sampler);

        let vertex_buffer = create_vertex_buffer(memory_allocator.clone());
        self.vertex_buffer = Some(vertex_buffer);

        self.camera = Some(Camera::from_pos(2.5, -2.5, 2.5));

        let mvp_buffer = Buffer::from_data(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            MVP::default(),
        )
        .unwrap();
        self.mvp_buffer = Some(mvp_buffer); // Temp store if needed later

        let framebuffers = window_size_dependent_setup(
            &images,
            render_pass.clone(),
            &mut self.viewport,
            &memory_allocator,
        );
        self.framebuffers = Some(framebuffers);

        self.egui = Some(EguiStruct::new(
            event_loop,
            surface,
            queue,
            Subpass::from(render_pass.clone(), 1).unwrap(),
        ));

        self.recreate_swapchain = false;
        self.previous_frame_end = Some(sync::now(device.clone()).boxed());
        self.input_state = Some(InputState::default());

        // Create the SceneManager and call Awake/Start
        let scene_manager = SceneManager::instance();
        scene_manager.awake();
        scene_manager.start();
        self.scene_manager = Some(scene_manager);

        // UserSettings is a singleton in order for easy access anywhere and hot reloading
        self.user_settigns = Some(UserSettings::instance());
    }
    fn create_pipeline(&mut self) {
        let depth_stencil_state = DepthStencilState {
            depth: Some(vulkano::pipeline::graphics::depth_stencil::DepthState {
                write_enable: true,
                compare_op: vulkano::pipeline::graphics::depth_stencil::CompareOp::Less,
            }),
            ..Default::default()
        };

        // Loading the vertex and fragment shaders
        mod vs {
            vulkano_shaders::shader! { ty: "vertex", path: "assets/shaders/first_triangle/vertex.glsl" }
        }
        mod fs {
            vulkano_shaders::shader! { ty: "fragment", path: "assets/shaders/first_triangle/fragment.glsl" }
        }

        let vs = vs::load(self.device.as_ref().unwrap().clone())
            .expect("failed to create shader module");
        let fs = fs::load(self.device.as_ref().unwrap().clone())
            .expect("failed to create shader module");

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };

        // Creating the graphics pipeline
        let pipeline = {
            let vs_entry = vs.entry_point("main").unwrap();
            let fs_entry = fs.entry_point("main").unwrap();

            let vertex_input_state = MyVertex::per_vertex().definition(&vs_entry).unwrap();

            let stages = [
                PipelineShaderStageCreateInfo::new(vs_entry),
                PipelineShaderStageCreateInfo::new(fs_entry),
            ];

            let layout = PipelineLayout::new(
                self.device.as_ref().unwrap().clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(self.device.as_ref().unwrap().clone())
                    .unwrap(),
            )
            .unwrap();

            let subpass = Subpass::from(self.render_pass.as_ref().unwrap().clone(), 0).unwrap();

            GraphicsPipeline::new(
                self.device.as_ref().unwrap().clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    depth_stencil_state: Some(depth_stencil_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState {
                        viewports: [viewport].into_iter().collect(),
                        ..Default::default()
                    }),
                    rasterization_state: Some(RasterizationState {
                        cull_mode: vulkano::pipeline::graphics::rasterization::CullMode::Back,
                        ..Default::default()
                    }),
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
        self.pipeline = Some(pipeline.clone()); // store

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            self.device.as_ref().unwrap().clone(),
            StandardDescriptorSetAllocatorCreateInfo::default(),
        ));
        self.descriptor_set_allocator = Some(descriptor_set_allocator.clone());
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            self.device.as_ref().unwrap().clone(),
            StandardDescriptorSetAllocatorCreateInfo::default(),
        ));
        let layout = pipeline.layout().set_layouts()[0].clone();
        let mut writes = vec![WriteDescriptorSet::buffer(
            0,
            self.mvp_buffer.as_ref().unwrap().clone(),
        )];
        writes.push(WriteDescriptorSet::image_view_sampler(
            1,
            self.texture.as_ref().unwrap().clone(),
            self.sampler.as_ref().unwrap().clone(),
        ));
        let set = DescriptorSet::new(descriptor_set_allocator.clone(), layout.clone(), writes, [])
            .unwrap();
        self.descriptor_set = Some(set);
        let set = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            layout.clone(),
            [WriteDescriptorSet::buffer(
                0,
                self.mvp_buffer.as_ref().unwrap().clone(),
            )],
            [],
        )
        .unwrap();

        self.descriptor_set_allocator = Some(descriptor_set_allocator);
        self.descriptor_set = Some(set);
    }
}

impl ApplicationHandler for App {
    // This is called once when the application starts.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
        self.create_vulkan(event_loop);
        self.create_pipeline();
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if self.capture_cursor {
                    self.input_state
                        .as_mut()
                        .expect("failed to get input state")
                        .update_mouse(delta)
                }
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let egui = self.egui.as_mut().unwrap();
        egui.update(
            &event,
            self.last_frame_time.unwrap_or(Instant::now()).elapsed(),
        );

        self.input_state
            .as_mut()
            .unwrap()
            .update_just_pressed_into_held();
        self.input_state
            .as_mut()
            .unwrap()
            .handle_events(event.clone());

        if self
            .input_state
            .as_ref()
            .expect("failed to get input state")
            .get_keybind_is_just_pressed(KeyBind::new("FreeMouse"))
        {
            self.capture_cursor = self.capture_cursor.not();
        }

        if self.capture_cursor {
            self.window
                .as_mut()
                .expect("failed to get window")
                .set_cursor_grab(CursorGrabMode::Confined)
                .expect("failed to set cursor grab mode to locked");
            self.window
                .as_mut()
                .expect("failed to get window")
                .set_cursor_visible(false);
        } else {
            self.window
                .as_mut()
                .expect("failed to get window")
                .set_cursor_grab(CursorGrabMode::None)
                .expect("failed to set cursor grab mode to None");

            self.window
                .as_mut()
                .expect("failed to get window")
                .set_cursor_visible(true);
        }

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
                let now = std::time::Instant::now();
                let delta_time = if let Some(last_frame_time) = self.last_frame_time {
                    let dt = now.duration_since(last_frame_time).as_secs_f32();
                    dt.min(0.1) // clamp max delta to avoid big jumps
                } else {
                    1.0 / 60.0 // default first frame delta
                };
                self.last_frame_time = Some(now);

                // Scene update order
                if let Some(scene_manager) = &self.scene_manager {
                    if let (Some(input_state), Some(camera)) =
                        (self.input_state.as_mut(), self.camera.as_mut())
                    {
                        scene_manager.fixed_update(delta_time, input_state, camera);
                        scene_manager.update(delta_time, input_state, camera);
                        scene_manager.late_update(delta_time, input_state, camera);
                    }
                }

                egui.redraw();

                // Camera update is now handled in the scene

                // Create MVP descriptor set through the scene manager
                let layout = self.pipeline.clone().unwrap().layout().set_layouts()[0].clone();
                let set = if let Some(scene_manager) = &self.scene_manager {
                    scene_manager
                        .create_mvp_descriptor_set(
                            self.memory_allocator.as_ref().unwrap(),
                            self.descriptor_set_allocator.as_ref().unwrap(),
                            &layout,
                            self.camera.as_ref().unwrap(),
                            self.texture.as_ref().unwrap(),
                            self.sampler.as_ref().unwrap(),
                        )
                        .unwrap_or_else(|| {
                            // Fallback to default MVP buffer if scene doesn't provide one
                            let mvp = MVP::default()
                                .apply_camera_transforms(*self.camera.as_ref().unwrap());
                            let mvp_buffer = Buffer::from_data(
                                self.memory_allocator.as_ref().unwrap().clone(),
                                BufferCreateInfo {
                                    usage: BufferUsage::UNIFORM_BUFFER,
                                    ..Default::default()
                                },
                                AllocationCreateInfo {
                                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                                    ..Default::default()
                                },
                                mvp,
                            )
                            .unwrap();

                            let mut writes = vec![WriteDescriptorSet::buffer(0, mvp_buffer)];
                            writes.push(WriteDescriptorSet::image_view_sampler(
                                1,
                                self.texture.as_ref().unwrap().clone(),
                                self.sampler.as_ref().unwrap().clone(),
                            ));
                            DescriptorSet::new(
                                self.descriptor_set_allocator.as_ref().unwrap().clone(),
                                layout.clone(),
                                writes,
                                [],
                            )
                            .unwrap()
                        })
                } else {
                    // Fallback if no scene manager
                    let mvp =
                        MVP::default().apply_camera_transforms(*self.camera.as_ref().unwrap());
                    let mvp_buffer = Buffer::from_data(
                        self.memory_allocator.as_ref().unwrap().clone(),
                        BufferCreateInfo {
                            usage: BufferUsage::UNIFORM_BUFFER,
                            ..Default::default()
                        },
                        AllocationCreateInfo {
                            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                            ..Default::default()
                        },
                        mvp,
                    )
                    .unwrap();

                    let mut writes = vec![WriteDescriptorSet::buffer(0, mvp_buffer)];
                    writes.push(WriteDescriptorSet::image_view_sampler(
                        1,
                        self.texture.as_ref().unwrap().clone(),
                        self.sampler.as_ref().unwrap().clone(),
                    ));
                    DescriptorSet::new(
                        self.descriptor_set_allocator.as_ref().unwrap().clone(),
                        layout.clone(),
                        writes,
                        [],
                    )
                    .unwrap()
                };
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
                        self.memory_allocator.as_ref().unwrap(),
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
                let image_extent: [u32; 2] = window.inner_size().into(); // Image extent

                // Do scene manager lifecycle draw
                if let Some(scene_manager) = &self.scene_manager {
                    scene_manager.draw();
                }

                // START BUILDING BUFFERS
                let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
                    command_buffer_allocator.clone(),
                    queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                cmd_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![
                                Some([0.0, 0.68, 1.0, 1.0].into()), // for color
                                Some(1.0_f32.into()),               // depth
                            ],
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
                    .bind_pipeline_graphics(self.pipeline.as_ref().unwrap().clone())
                    .unwrap()
                    .bind_vertex_buffers(0, self.vertex_buffer.as_ref().unwrap().clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        vulkano::pipeline::PipelineBindPoint::Graphics,
                        self.pipeline.as_ref().unwrap().layout().clone(),
                        0,
                        set.clone(),
                    )
                    .unwrap();

                // Wrap the draw call in an unsafe block so it works :|
                unsafe {
                    cmd_buffer_builder
                        .draw(self.vertex_buffer.as_ref().unwrap().len() as u32, 1, 0, 0)
                        .unwrap();
                }

                cmd_buffer_builder
                    .next_subpass(
                        SubpassEndInfo::default(),
                        SubpassBeginInfo {
                            contents: SubpassContents::SecondaryCommandBuffers,
                            ..Default::default()
                        },
                    )
                    .unwrap()
                    .execute_commands(egui.draw_on_subpass_image(image_extent))
                    .unwrap()
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
                self.input_state
                    .as_mut()
                    .expect("failed to unwrap input state")
                    .reset_deltas();
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
