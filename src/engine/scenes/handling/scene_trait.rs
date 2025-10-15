use crate::engine::scenes::handling::scene_manager::SceneManager;

pub trait Scene: Send {
    fn awake(&mut self) {
        // Initialize systems, load assets. Runs ONCE when scene is created, before Start
    }

    fn start(&mut self) {
        // Called when the scene becomes active.
    }

    fn update(&mut self) {
        // Per frame. Main logic. (input, world, AI, etc.)
    }

    fn fixed_update(&mut self) {
        // Runs on a fixed timestep. Preferred for physics and collisions
    }

    fn late_update(&mut self) {
        // Runs after update. Good for things like camera follow logic, etc.
    }

    fn draw(&self)
    {
        // Runs after Update, Fixed Update, and Late Update.
    }
}
