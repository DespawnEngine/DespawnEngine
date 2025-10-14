use crate::engine::scenes::handling::scene_manager::SceneManager;

pub trait Scene: Send {
    fn awake(&mut self, _manager: &SceneManager) {
        // Initialize systems, load assets. Runs ONCE when scene is created, before Start
    }

    fn start(&mut self, _manager: &SceneManager) {
        // Called when the scene becomes active.
    }

    fn update(&mut self, _manager: &SceneManager) {
        // Per frame. Main logic. (input, world, AI, etc.)
    }

    fn fixed_update(&mut self, _manager: &SceneManager) {
        // Runs on a fixed timestep. Preferred for physics and collisions
    }

    fn late_update(&mut self, _manager: &SceneManager) {
        // Runs after update. Good for things like camera follow logic, etc.
    }

    fn draw(&self)
    {
        // Runs after Update, Fixed Update, and Late Update.
    }
}
