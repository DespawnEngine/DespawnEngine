use vulkano::buffer::BufferContents;

use crate::engine::rendering::camera::Camera;
use crate::utils::math::perspective_rh;
use bytemuck::{Pod, Zeroable};

use glam::{Mat4, Quat, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Mat4Pod(pub [[f32; 4]; 4]);

impl From<Mat4> for Mat4Pod {
    fn from(mat: Mat4) -> Self {
        Mat4Pod(mat.to_cols_array_2d())
    }
}

impl From<Mat4Pod> for Mat4 {
    fn from(pod: Mat4Pod) -> Self {
        Mat4::from_cols_array_2d(&pod.0)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
pub struct MVP {
    pub model: Mat4Pod,
    pub view: Mat4Pod,
    pub proj: Mat4Pod,
}

impl Default for MVP {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY.into(),
            view: Mat4::IDENTITY.into(),
            proj: Mat4::perspective_rh_gl(80.0f32.to_radians(), 1.0, 0.01, 2000.0).into(),
        }
    }
}

impl MVP {
    pub fn apply_camera_transforms(&self, camera: Camera) -> Self {
        let translation = Mat4::from_translation(-camera.position);
        let rotation = Mat4::from_quat(camera.rotation_quat.conjugate());
        let view_mat = rotation * translation;

        MVP {
            model: self.model,
            view: view_mat.into(),
            proj: self.proj,
        }
    }
}
