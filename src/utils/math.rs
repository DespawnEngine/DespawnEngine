use std::f32::consts::PI;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::process::Output;

use glam::EulerRot;
use glam::Mat4 as glam_mat4;
use glam::Quat as glam_quat;
use glam::Vec3 as glam_vec3;
use glam::Vec4 as glam_vec4;
use glam::mat4 as glam_mat4_construct;
use vulkano::buffer::BufferContents;

#[derive(BufferContents, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Vec3(pub(crate) [f32; 3]);

#[derive(BufferContents, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Vec4([f32; 4]);

#[derive(BufferContents, Copy, Clone, Debug)]
#[repr(C)]
pub struct Mat4([Vec4; 4]);

#[derive(BufferContents, Copy, Clone, Debug)]
#[repr(C)]
pub struct Quat(Vec4);

impl From<[f32; 4]> for Quat {
    fn from(value: [f32; 4]) -> Self {
        Quat(value.into())
    }
}

impl From<Quat> for glam_quat {
    fn from(value: Quat) -> Self {
        glam_quat::from_vec4(value.0.into())
    }
}

impl From<glam_quat> for Quat {
    fn from(value: glam_quat) -> Self {
        value.to_array().into()
    }
}

impl Quat {
    pub const IDENTITY: Quat = Quat(Vec4([0.0, 0.0, 0.0, 1.0]));
    pub fn from_euler_rad(x_rot: f32, y_rot: f32, z_rot: f32) -> Self {
        glam_quat::from_euler(EulerRot::XYZ, x_rot, y_rot, z_rot).into()
    }

    pub fn from_euler_deg(x_rot: f32, y_rot: f32, z_rot: f32) -> Self {
        glam_quat::from_euler(
            EulerRot::YXZ,
            y_rot.to_radians(),
            x_rot.to_radians(),
            z_rot.to_radians(),
        )
        .into()
    }
}

impl Vec3 {
    pub fn int_div(self, rhs: i32) -> [i32; 3] {
        [
            self.0[0] as i32 / rhs,
            self.0[1] as i32 / rhs,
            self.0[2] as i32 / rhs,
        ]
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::IDENTITY
    }
}

impl Default for Quat {
    fn default() -> Self {
        Quat::IDENTITY
    }
}

// Taken from
// https://docs.rs/glm/latest/src/glm/ext/matrix/transform.rs.html#65-88

/// Creates a matrix for a right handed, symetric perspective-view frustum.
/// `fov_y` is the field of view angle in the y direction in radians.
/// The `aspect` ratio determines the field of view in the x direction.
/// `near_z` is the distance from the viewer to the near clipping plane (always positive) and
/// `far_z` is the distance from the viewer to the far clipping plane (always positive).
pub fn perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4 {
    let zero = 0.0;
    let one = 1.0;
    let two = 2.0;

    let q = one / (fov_y / two).tan();
    let a = q / aspect;
    let b = (z_near + z_far) / (z_near - z_far);
    let c = (two * z_near * z_far) / (z_near - z_far);

    Mat4([
        Vec4::from([a, zero, zero, zero]),
        Vec4::from([zero, q, zero, zero]),
        Vec4::from([zero, zero, b, zero - one]),
        Vec4::from([zero, zero, c, zero]),
    ])
}

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Self) -> Self::Output {
        (glam_mat4::from(self) * glam_mat4::from(rhs)).into()
    }
}

impl From<Vec3> for Mat4 {
    fn from(value: Vec3) -> Self {
        glam_mat4::from_translation(glam_vec3::from(value)).into()
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3(value)
    }
}

impl From<[i32; 3]> for Vec3 {
    fn from(value: [i32; 3]) -> Self {
        Vec3([value[0] as f32, value[1] as f32, value[2] as f32])
    }
}

impl From<glam_vec3> for Vec3 {
    fn from(value: glam_vec3) -> Self {
        Vec3(value.to_array())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl From<Vec3> for glam_vec3 {
    fn from(value: Vec3) -> Self {
        glam_vec3::from_array(value.0)
    }
}

impl<Idx: Into<i32>> Index<Idx> for Vec3 {
    type Output = f32;
    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index.into() as usize)
    }
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4([
        Vec4([1.0, 0.0, 0.0, 0.0]),
        Vec4([0.0, 1.0, 0.0, 0.0]),
        Vec4([0.0, 0.0, 1.0, 0.0]),
        Vec4([0.0, 0.0, 0.0, 1.0]),
    ]);
    pub fn inverse(self) -> Self {
        glam_mat4::from(self).inverse().into()
    }
    pub fn from_quat(quat: Quat) -> Self {
        glam_mat4::from_quat(quat.into()).into()
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Vec4(value)
    }
}

impl From<Vec4> for glam_vec4 {
    fn from(value: Vec4) -> Self {
        glam_vec4::from_array(value.0)
    }
}

impl From<glam_vec4> for Vec4 {
    fn from(value: glam_vec4) -> Self {
        Vec4(value.to_array())
    }
}

impl<Idx: Into<i32>> Index<Idx> for Mat4 {
    type Output = Vec4;
    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index.into() as usize)
    }
}

impl From<Mat4> for glam_mat4 {
    fn from(value: Mat4) -> Self {
        glam_mat4_construct(
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
        )
    }
}

impl From<glam_mat4> for Mat4 {
    fn from(value: glam_mat4) -> Self {
        Mat4([
            value.x_axis.into(),
            value.y_axis.into(),
            value.z_axis.into(),
            value.w_axis.into(),
        ])
    }
}
