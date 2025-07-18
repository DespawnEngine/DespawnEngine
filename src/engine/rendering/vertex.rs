// engine/renderer/first_triangle.rs
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MyVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],  // now matches the format (3 * 4 bytes = 12)
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}
