
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

use crate::utils::math::{Vec2, Vec4};

#[derive(BufferContents, Vertex, Clone, Copy, Debug)]
#[repr(C)]
pub struct GuiVertex{

    #[format(R32G32_SFLOAT)]
    pub position: Vec2,

    #[format(R32_SFLOAT)]
    pub depth: f32,

    #[format(R32G32B32A32_SFLOAT)]
    pub color: Vec4,

    #[format(R32G32_SFLOAT)]
    pub uv: Vec2,
}


impl GuiVertex {
    pub fn new<P: Into<Vec2>, C: Into<Vec4>>(pos: P, col: C, depth: f32, uv: P) -> Self {
        Self {
            position: pos.into(),
            color: col.into(),
            depth,
            uv: uv.into(),
        }
    }
}

impl Default for GuiVertex {
    fn default() -> Self {
        Self{
            position: [-1.0, -1.0].into(),
            color: [1.0, 0.0, 0.0, 1.0].into(),
            depth: 0.0,
            uv: [0.0, 0.0].into(),
        }
    }
}
