use glam::{Quat, Vec3};
use crate::engine::core::input::InputState;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation_quat: Quat,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn from_pos(pos_x: f32, pos_y: f32, pos_z: f32) -> Self {
        Camera {
            position: Vec3::new(pos_x, pos_y, pos_z),
            rotation_quat: Quat::from_euler(glam::EulerRot::YXZ, 45.0f32.to_radians(), 45.0f32.to_radians(), 0.0),
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn yaw(&self) -> f32 {
        self.rotation_quat.to_euler(glam::EulerRot::YXZ).0.to_degrees()
    }
    pub fn pitch(&self) -> f32 {
        self.rotation_quat.to_euler(glam::EulerRot::YXZ).1.to_degrees()
    }

    pub fn from_vec3_pos(position: Vec3) -> Self {
        Camera {
            position,
            rotation_quat: Quat::IDENTITY,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn from_pos_and_rot(position: Vec3, rotation_quat: Quat) -> Self {
        Camera {
            position,
            rotation_quat,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputState) {
        // Movement
        let forward = self.rotation_quat * Vec3::new(0.0, 0.0, -1.0);
        let right = self.rotation_quat * Vec3::new(1.0, 0.0, 0.0);
        let up = Vec3::new(0.0, -1.0, 0.0);

        if input.w_pressed {
            self.position += forward * self.speed * delta_time;
        }
        if input.s_pressed {
            self.position -= forward * self.speed * delta_time;
        }
        if input.a_pressed {
            self.position -= right * self.speed * delta_time;
        }
        if input.d_pressed {
            self.position += right * self.speed * delta_time;
        }
        if input.space_pressed {
            self.position += up * self.speed * delta_time;
        }
        if input.shift_pressed {
            self.position -= up * self.speed * delta_time;
        }

        // Mouse rotation
        let new_yaw = self.yaw() - input.mouse_delta_x;
        let new_pitch = self.pitch() + input.mouse_delta_y;

        println!("{:?}", input);

        self.rotation_quat = Quat::from_euler(glam::EulerRot::YXZ,  new_yaw.to_radians(), new_pitch.to_radians(), 0.0);
    }
}
