use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::scenes::handling::scene_trait::{Scene, SceneResources};
use crate::engine::scenes::handling::scene_types::SceneType;
use crate::engine::scenes::scene_game::GameScene;
use crate::engine::scenes::scene_menu::MenuScene;
use std::sync::{Arc, Mutex, OnceLock};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::viewport::Viewport;

#[derive(Clone)]
pub struct SceneManager {
    scenes: Arc<Mutex<Vec<(SceneType, Box<dyn Scene + Send>)>>>,
    current_scene: Arc<Mutex<Option<SceneType>>>,
    next_scene: Arc<Mutex<Option<SceneType>>>,
    scene_resources: Arc<Mutex<Option<SceneResources>>>,
}

#[rustfmt::skip]
impl SceneManager
{
    pub fn new() -> Self {
        let scenes = vec![
            (SceneType::Menu, Box::new(MenuScene) as Box<dyn Scene + Send>),
            (SceneType::Game, Box::new(GameScene::new()) as Box<dyn Scene + Send>),
        ];

        SceneManager {
            scenes: Arc::new(Mutex::new(scenes)),
            current_scene: Arc::new(Mutex::new(Some(SceneType::Menu))),
            next_scene: Arc::new(Mutex::new(None)),
            scene_resources: Arc::new(Mutex::new(None)),
        }
    }

    pub fn instance() -> Self
    {
        static INSTANCE: OnceLock<SceneManager> = OnceLock::new();
        INSTANCE.get_or_init(SceneManager::new).clone()
    }

    pub fn set_scene_resources(&self, resources: SceneResources) {
        *self.scene_resources.lock().unwrap() = Some(resources);
    }

    fn scene_resources(&self) -> Option<SceneResources> {
        self.scene_resources.lock().unwrap().clone()
    }

    pub fn switch_scene(&self, scene_type: SceneType)
    {
        let mut current_scene = self.current_scene.lock().unwrap();
        *current_scene = Some(scene_type);

        // When switching, run awake() and start() for the new scene
        let mut scenes = self.scenes.lock().unwrap();
        if let Some((_, scene)) = scenes.iter_mut().find(|(st, _)| *st == scene_type)
        {
            if let Some(resources) = self.scene_resources() {
                scene.inject_resources(&resources);
            }

            scene.awake();
            scene.start();
        }
    }
    pub fn queue_scene_switch(&self, scene_type: SceneType) {
        let mut next_scene = self.next_scene.lock().unwrap();
        *next_scene = Some(scene_type);
    }

    fn with_current_scene_mut<F>(&self, mut f: F)
    where
        F: FnMut(&mut dyn Scene),
    {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type
        {
            let mut scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter_mut().find(|(st, _)| *st == scene_type)
            {
                f(scene.as_mut());
            }
        }
    }

    fn with_current_scene<F>(&self, mut f: F)
    where
        F: FnMut(&dyn Scene),
    {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type
        {
            let scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter().find(|(st, _)| *st == scene_type)
            {
                f(scene.as_ref());
            }
        }
    }

    fn with_current_scene_mut_with_params<F>(&self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera, mut f: F)
    where
        F: FnMut(&mut dyn Scene, f32, &mut InputState, &mut Camera),
    {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type
        {
            let mut scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter_mut().find(|(st, _)| *st == scene_type)
            {
                f(scene.as_mut(), delta_time, input_state, camera);
            }
        }
    }

    // Lifecycle Methods
    pub fn awake(&self)
    {
        self.with_current_scene_mut(|scene| scene.awake());
    }

    pub fn start(&self)
    {
        self.with_current_scene_mut(|scene| scene.start());
    }

    pub fn update(&self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera)
    {
        // Update the current scene
        self.with_current_scene_mut_with_params(delta_time, input_state, camera, |scene, dt, input, cam| scene.update(dt, input, cam));

        // After updating, check if a new scene was queued
        let mut next_scene = self.next_scene.lock().unwrap();
        if let Some(scene_type) = next_scene.take() {
            drop(next_scene); // unlock before switching
            self.switch_scene(scene_type);
        }
    }

    pub fn fixed_update(&self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera)
    {
        self.with_current_scene_mut_with_params(delta_time, input_state, camera, |scene, dt, input, cam| scene.fixed_update(dt, input, cam));
    }

    pub fn late_update(&self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera)
    {
        self.with_current_scene_mut_with_params(delta_time, input_state, camera, |scene, dt, input, cam| scene.late_update(dt, input, cam));
    }

    pub fn draw(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        viewport: &Viewport,
        allocator: &StandardMemoryAllocator,
    ) {
        // Lock the resources
        let resources = self.scene_resources.lock().unwrap();
        if let Some(res) = &*resources {
            self.with_current_scene(|scene| {
                scene.draw(builder, viewport, allocator, res);
            });
        }
    }

    pub fn create_mvp_descriptor_set(&self,
        memory_allocator: &Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        layout: &Arc<DescriptorSetLayout>,
        camera: &Camera,
        texture_view: &Arc<ImageView>,
        sampler: &Arc<Sampler>,
    ) -> Option<Arc<DescriptorSet>> {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type {
            let scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter().find(|(st, _)| *st == scene_type) {
                return scene.create_mvp_descriptor_set(memory_allocator, descriptor_set_allocator, layout, camera, texture_view, sampler);
            }
        }
        None
    }

    pub fn inject_resources_to_all(&self, resources: &SceneResources) {
        let mut scenes = self.scenes.lock().unwrap();
        for (_, scene) in scenes.iter_mut() {
            scene.inject_resources(resources);
        }
    }
}
