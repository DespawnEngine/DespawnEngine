// rust
// File: src/engine/scenes/scene_game.rs
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::mvp::MVP;
use crate::engine::rendering::descriptor_helpers::make_mvp_descriptor_set;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use std::sync::Arc;

pub struct GameScene;

impl Scene for GameScene {
    fn start(&mut self) {
        println!("Started Game Scene");
    }

    fn update(&mut self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera) {
        camera.update(delta_time, input_state);
    }

    fn fixed_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {}

    fn late_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {}

    fn draw(&self) {}

    fn create_mvp_descriptor_set(&self,
                                 memory_allocator: &Arc<StandardMemoryAllocator>,
                                 descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
                                 layout: &Arc<DescriptorSetLayout>,
                                 camera: &Camera
    ) -> Option<Arc<DescriptorSet>> {
        make_mvp_descriptor_set(memory_allocator, descriptor_set_allocator, layout, camera)
    }
}

impl GameScene {
    pub fn new() -> Self {
        GameScene
    }
}