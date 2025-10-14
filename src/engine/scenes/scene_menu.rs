use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::scenes::handling::scene_types::SceneType;

pub struct MenuScene;
impl Scene for MenuScene
{
    fn update(&mut self, manager: &SceneManager) {
        // Auto switch to game scene. Later add a real main menu
        if true {
            manager.switch_scene(SceneType::Game);
        }
    }

    fn draw(&self) {
        println!("Drawing Menu Scene");
    }
}

impl MenuScene
{
    pub fn new() -> Self {
        MenuScene
    }
}