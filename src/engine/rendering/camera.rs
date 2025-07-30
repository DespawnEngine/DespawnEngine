use crate::utils::math::{Quat, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Camera{
    pub position: Vec3,
    pub rotation_quat: Quat,
}

impl Camera{
    pub fn from_pos(pos_x: f32, pos_y: f32, pos_z: f32) -> Self{
        Camera{
            position: [pos_x, pos_y, pos_z].into(),
            rotation_quat: Quat::from_euler_deg(-45.0, 45.0, 0.0)
        }
    }

    pub fn from_vec3_pos(position: Vec3) -> Self{
        Camera{
            position,
            rotation_quat: Quat::IDENTITY,
        }
    }

    pub fn from_pos_and_rot(position: Vec3, rotation_quat: Quat) -> Self{
        Camera { position, rotation_quat }
    }
}
