use vulkano::buffer::BufferContents;

use crate::engine::rendering::camera::Camera;
use crate::utils::math::{Mat4, perspective_rh};

#[allow(clippy::upper_case_acronyms)]
#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
pub struct MVP {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

impl Default for MVP {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            proj: perspective_rh(80.0, 1.0, 0.01, 20.0),
        }
    }
}

impl MVP {
    pub fn apply_camera_transforms(&self, camera: Camera) -> Self {
        MVP {
            model: self.model,
            view: (Mat4::from(camera.position) * Mat4::from_quat(camera.rotation_quat)).inverse(),
            proj: self.proj,
        }
    }
}
