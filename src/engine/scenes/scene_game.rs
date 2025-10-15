use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::mvp::MVP;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use std::sync::Arc;

pub struct GameScene;

impl Scene for GameScene {

    fn start(&mut self)
    {
        println!("Started Game Scene");
    }

    fn update(&mut self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera)
    {
        // Update the camera with the current input state and delta time
        camera.update(delta_time, input_state);
    }

    fn fixed_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera)
    {
        // Physics and collision logic can go here
    }

    fn late_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera)
    {
        // Camera follow logic, etc.
    }

    fn draw(&self)
    {
        //println!("Drawing Game Scene");
    }

    fn create_mvp_descriptor_set(&self, 
        memory_allocator: &Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        layout: &Arc<DescriptorSetLayout>,
        camera: &Camera
    ) -> Option<Arc<DescriptorSet>> {
        // Create the MVP buffer from the camera
        let mvp_buffer = Buffer::from_data(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            MVP::default().apply_camera_transforms(*camera),
        ).ok()?;

        // Create the descriptor set
        let set = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            layout.clone(),
            [WriteDescriptorSet::buffer(0, mvp_buffer)],
            [],
        ).ok()?;

        Some(set)
    }
}

impl GameScene {
    pub fn new() -> Self {
        GameScene
    }
}