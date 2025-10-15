use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::scenes::handling::scene_types::SceneType;

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

    fn update(&mut self) {

    }

    fn draw(&self) {
        //println!("Drawing Menu Scene");
    }
}

impl MenuScene
{
    pub fn new() -> Self {
        MenuScene
    }
}