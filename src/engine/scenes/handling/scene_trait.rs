use crate::engine::scenes::handling::scene_manager::SceneManager;

pub trait Scene {
    fn start(&mut self, manager: &SceneManager) {}
    fn update(&mut self, manager: &SceneManager);
    fn draw(&self);
}