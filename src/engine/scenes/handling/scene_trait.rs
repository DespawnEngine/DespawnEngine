use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::texture_atlas::AtlasUV;
use rapidhash::RapidHashMap;
use std::collections::HashMap;
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::viewport::Viewport;

#[derive(Clone)]
pub struct SceneResources {
    pub default_pipeline: Arc<GraphicsPipeline>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub texture: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
    pub block_uvs: Option<RapidHashMap<String, AtlasUV>>,
}

pub trait Scene: Send {
    fn awake(&mut self) {
        // Initialize systems, load assets. Runs ONCE when scene is created, before Start
    }

    fn start(&mut self) {
        // Called when the scene becomes active.
    }

    fn update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {
        // Per frame. Main logic. (input, world, AI, etc.)
    }

    fn fixed_update(
        &mut self,
        _delta_time: f32,
        _input_state: &mut InputState,
        _camera: &mut Camera,
    ) {
        // Runs on a fixed timestep. Preferred for physics and collisions
    }

    fn late_update(
        &mut self,
        _delta_time: f32,
        _input_state: &mut InputState,
        _camera: &mut Camera,
    ) {
        // Runs after update. Good for things like camera follow logic, etc.
    }

    fn draw(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        viewport: &Viewport,
        allocator: &StandardMemoryAllocator,
        resources: &SceneResources,
        // Runs after Update, Fixed Update, and Late Update.
    );

    fn create_mvp_descriptor_set(
        &self,
        _memory_allocator: &Arc<StandardMemoryAllocator>,
        _descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        _layout: &Arc<DescriptorSetLayout>,
        _camera: &Camera,
        _texture_view: &Arc<ImageView>,
        _sampler: &Arc<Sampler>,
    ) -> Option<Arc<DescriptorSet>> {
        None
    }

    fn inject_resources(&mut self, _resources: &SceneResources) {}
}
