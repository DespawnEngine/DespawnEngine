use crate::arguments;
use crate::engine::rendering::vertex::MyVertex;
use image::GenericImageView;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::render_pass::RenderPass;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Icon, Window};

use crate::engine::rendering::vswapchain::IMAGE_FORMAT;
// "image" crate uses this for loading images

// This display script will contain almost all window functionality later, hopefully. Need to make sure I didn't break linux though first.
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
                format: IMAGE_FORMAT,
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
    )
    .unwrap()
}

// Cube module //TODO: This shouldn't be needed like this. Remove later and make chunk mesh create its own.
pub fn create_vertex_buffer(allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[MyVertex]> {
    crate::engine::rendering::cube::create_cube_vertex_buffer(allocator)
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
