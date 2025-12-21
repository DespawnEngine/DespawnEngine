use crate::utils::math::Vec3;
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct BlockVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3, // now matches the format (3 * 4 bytes = 12)
    #[format(R32G32_SFLOAT)]
    pub tex_coords: [f32; 2],
}

impl BlockVertex {
    pub fn new<T: Into<Vec3>>(pos: T, uv: [f32; 2]) -> Self {
        Self {
            position: pos.into(),
            tex_coords: uv,
        }
    }
    pub const ZERO: BlockVertex = BlockVertex {
        position: Vec3([0.0; 3]),
        tex_coords: [0.0, 1.0],
    };
}
