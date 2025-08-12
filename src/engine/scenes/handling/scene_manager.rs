use std::sync::{Arc, Mutex, OnceLock};
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::scenes::handling::scene_types::SceneType;
use crate::engine::scenes::scene_game::GameScene;
use crate::engine::scenes::scene_menu::MenuScene;

#[derive(Clone)]
pub struct SceneManager {
    scenes: Arc<Mutex<Vec<(SceneType, Box<dyn Scene + Send>)>>>,
    current_scene: Arc<Mutex<Option<SceneType>>>,
}

impl SceneManager {
    pub fn new() -> Self {
        let scenes = vec![
            (SceneType::Menu, Box::new(MenuScene) as Box<dyn Scene + Send>),
            (SceneType::Game, Box::new(GameScene) as Box<dyn Scene + Send>),
        ];
        SceneManager {
            scenes: Arc::new(Mutex::new(scenes)),
            current_scene: Arc::new(Mutex::new(Some(SceneType::Menu))),
        }
    }

    pub fn instance() -> Self {
        static INSTANCE: OnceLock<SceneManager> = OnceLock::new();
        INSTANCE.get_or_init(SceneManager::new).clone()
    }

    pub fn switch_scene(&self, scene_type: SceneType) {
        *self.current_scene.lock().unwrap() = Some(scene_type);
    }

    pub fn update(&self) {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type {
            let mut scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter_mut().find(|(st, _)| *st == scene_type) {
                scene.update(self); // Pass self directly
            }
        }
    }

    pub fn draw(&self) {
        let scene_type = *self.current_scene.lock().unwrap();
        if let Some(scene_type) = scene_type {
            let scenes = self.scenes.lock().unwrap();
            if let Some((_, scene)) = scenes.iter().find(|(st, _)| *st == scene_type) {
                scene.draw();
            }
        }
    }
}