use vulkano::buffer::BufferContents;

const IDENTITY_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
pub struct MVP {
    pub model: [[f32; 4]; 4],
    pub view:  [[f32; 4]; 4],
    pub proj:  [[f32; 4]; 4],
}

impl Default for MVP {
    fn default() -> Self {
        Self {
            model: IDENTITY_MATRIX,
            view:  IDENTITY_MATRIX,
            proj:  IDENTITY_MATRIX,
        }
    }
}