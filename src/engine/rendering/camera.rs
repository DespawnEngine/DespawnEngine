use glam::{Quat, Vec3};
use crate::engine::rendering::display::InputState;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation_quat: Quat,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn from_pos(pos_x: f32, pos_y: f32, pos_z: f32) -> Self {
        Camera {
            position: Vec3::new(pos_x, pos_y, pos_z),
            rotation_quat: Quat::from_euler(glam::EulerRot::XYZ, -45.0f32.to_radians(), 45.0f32.to_radians(), 0.0),
            yaw: 45.0,
            pitch: -45.0,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn from_vec3_pos(position: Vec3) -> Self {
        Camera {
            position,
            rotation_quat: Quat::IDENTITY,
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn from_pos_and_rot(position: Vec3, rotation_quat: Quat) -> Self {
        Camera {
            position,
            rotation_quat,
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
            sensitivity: 0.1,
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputState) {
        // Movement
        let forward = self.rotation_quat * Vec3::new(0.0, 0.0, 1.0);
        let right = self.rotation_quat * Vec3::new(1.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        print!("yo");
        if input.w_pressed {
            self.position += forward * self.speed * delta_time;
            print!("w pressed");
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
        self.yaw += input.mouse_delta_x * self.sensitivity;
        self.pitch = (self.pitch + input.mouse_delta_y * self.sensitivity).clamp(-89.0, 89.0);

        self.rotation_quat = Quat::from_euler(glam::EulerRot::XYZ, self.pitch.to_radians(), self.yaw.to_radians(), 0.0);
    }
}
