use std::sync::Arc;
use winit::window::{Icon, Window};
use image::GenericImageView;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::render_pass::RenderPass;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event::WindowEvent::KeyboardInput;
use winit::event_loop::ActiveEventLoop;
use crate::arguments;
use crate::engine::rendering::vertex::MyVertex;
use winit::keyboard::{KeyCode, PhysicalKey};
// "image" crate uses this for loading images

// This display script will contain almost all window functionality later, hopefully. Need to make sure I didn't break linux though first.

pub struct InputState {
    pub w_pressed: bool,
    pub s_pressed: bool,
    pub a_pressed: bool,
    pub d_pressed: bool,
    pub space_pressed: bool,
    pub shift_pressed: bool,
    pub mouse_delta_x: f32,
    pub mouse_delta_y: f32,
    pub last_mouse_x: f32,
    pub last_mouse_y: f32,
}

// Creating the main window with definitions
pub fn create_main_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    let window_attributes = Window::default_attributes()
        .with_title("Despawn Engine")
        .with_decorations(arguments::use_decorations())
        .with_window_icon(Some(load_icon("assets/icon.png")));
    Arc::new(event_loop.create_window(window_attributes).unwrap())
}

// Define a less simple render pass with one color attachment and whatever else makes this
// maco happy :)
pub fn create_render_pass(device: Arc<Device>) -> Arc<RenderPass> {
    vulkano::ordered_passes_renderpass!(
        device,
        attachments: {
            color: {
                format: Format::R8G8B8A8_SRGB,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
            depth: {
                format: Format::D32_SFLOAT,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
            }
        },
        passes: [
            {
                color: [color],
                depth_stencil: {depth},
                input: []
            },
            {
                color: [color],
                depth_stencil: {},
                input: []
            }
        ]
    ).unwrap()
}

// Create vertex buffer and simple cube rendering
pub fn create_vertex_buffer(
    allocator: Arc<StandardMemoryAllocator>,
) -> Subbuffer<[MyVertex]> {
    let vertex_data = [
        // Front face
        MyVertex { position: [-0.5, -0.5,  0.5].into(), color: [1.0, 0.0, 0.0].into() },
        MyVertex { position: [ 0.5, -0.5,  0.5].into(), color: [0.0, 1.0, 0.0].into() },
        MyVertex { position: [ 0.5,  0.5,  0.5].into(), color: [0.0, 0.0, 1.0].into() },
        MyVertex { position: [-0.5,  0.5,  0.5].into(), color: [1.0, 1.0, 0.0].into() },

        // Back face
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into() },
        MyVertex { position: [ 0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into() },
        MyVertex { position: [ 0.5,  0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into() },
        MyVertex { position: [-0.5,  0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into() },
    ];

    // Define triangles using these vertices
    let index_order = [
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        5, 4, 7, 7, 6, 5, // back
        4, 0, 3, 3, 7, 4, // left
        3, 2, 6, 6, 7, 3, // top
        4, 5, 1, 1, 0, 4, // bottom
    ];

    let full_vertex_data: Vec<MyVertex> = index_order
        .iter()
        .map(|&i| vertex_data[i].clone())
        .collect();

    Buffer::from_iter(
        allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        full_vertex_data,
    ).unwrap()
}



pub fn handle_events(event: Event<()>, input_state: &mut InputState) {
    if let Event::WindowEvent { event, .. } = event {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    match keycode {
                        KeyCode::KeyW => input_state.w_pressed = pressed,
                        KeyCode::KeyS => input_state.s_pressed = pressed,
                        KeyCode::KeyA => input_state.a_pressed = pressed,
                        KeyCode::KeyD => input_state.d_pressed = pressed,
                        KeyCode::Space => input_state.space_pressed = pressed,
                        KeyCode::ShiftLeft | KeyCode::ShiftRight => input_state.shift_pressed = pressed,
                        _ => {}
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let new_x = position.x as f32;
                let new_y = position.y as f32;

                input_state.mouse_delta_x = new_x - input_state.last_mouse_x;
                input_state.mouse_delta_y = new_y - input_state.last_mouse_y;

                input_state.last_mouse_x = new_x;
                input_state.last_mouse_y = new_y;
            }
            _ => {}
        }
    }
}



// Helper function for loading an icon for the window icon. Code will likely be changed, but I wanted to experiment to learn more.
pub fn load_icon(path: &str) -> Icon {
    // Load the image
    let image = image::open(path).expect("Failed to open icon file");

    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw(); // Convert to raw RGBA bytes

    // Create winit Icon
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}