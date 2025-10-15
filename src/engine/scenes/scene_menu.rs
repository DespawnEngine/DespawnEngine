use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::scenes::handling::scene_types::SceneType;

pub struct MenuScene;
impl Scene for MenuScene
{
    fn start(&mut self){}
    
    fn update(&mut self) {
        // Auto switch to game scene. Later add a real main menu
        
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