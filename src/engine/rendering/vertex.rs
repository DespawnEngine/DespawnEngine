use crate::utils::math::Vec3;
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MyVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3, // now matches the format (3 * 4 bytes = 12)
    #[format(R32G32B32_SFLOAT)]
    pub color: Vec3,
    #[format(R32G32_SFLOAT)]
    pub tex_coords: [f32; 2],
}

impl MyVertex {
    pub fn new<T: Into<Vec3>>(pos: T, col: T) -> Self {
        MyVertex {
            position: pos.into(),
            color: col.into(),
            tex_coords: [0.0, 0.0]
        }
    }
}
