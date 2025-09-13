
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

use crate::utils::math::{Vec2, Vec3, Vec4};

#[derive(BufferContents, Vertex, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GuiVertex{

    #[format(R32G32_SFLOAT)]
    pub position: Vec2,

    #[format(R32G32B32A32_SFLOAT)]
    pub color: Vec4,
}



impl GuiVertex {
    pub fn new<P: Into<Vec2>, C: Into<Vec4>>(pos: P, col: C) -> Self {
        Self {
            position: pos.into(),
            color: col.into(),
        }
    }
}
