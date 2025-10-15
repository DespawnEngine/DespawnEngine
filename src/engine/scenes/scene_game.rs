use crate::engine::scenes::handling::scene_manager::SceneManager;
use crate::engine::scenes::handling::scene_trait::Scene;

pub struct GameScene;

impl Scene for GameScene {

    fn draw(&self)
    {
        println!("Drawing Game Scene");
    }
}

impl GameScene {
    pub fn new() -> Self {
        GameScene
    }
}