use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::viewport::Viewport;
use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::scenes::handling::scene_trait::{Scene, SceneResources};
use crate::engine::scenes::handling::scene_types::SceneType;
use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;

pub struct MenuScene;
impl Scene for MenuScene
{
    fn start(&mut self)
    {
        // Auto swaps to the Game Scene for now until we add a main menu
        println!("Swapping to Game Scene");
        let scene_manager = SceneManager::instance();
        scene_manager.queue_scene_switch(SceneType::Game);
    }

    fn update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {

    }

    fn fixed_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {

    }

    fn late_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {

    }

    fn draw(
        &self,
        _builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        _viewport: &Viewport,
        _allocator: &StandardMemoryAllocator,
        _resources: &SceneResources,
    ) {
        //println!("Drawing Menu Scene");
    }
}

impl MenuScene
{
    pub fn new() -> Self {
        MenuScene
    }
}