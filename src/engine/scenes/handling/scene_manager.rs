use std::sync::{Arc, Mutex, OnceLock};
use crate::engine::scenes::handling::scene_trait::Scene;
use crate::engine::scenes::handling::scene_types::SceneType;
use crate::engine::scenes::scene_game::GameScene;
use crate::engine::scenes::scene_menu::MenuScene;

#[derive(Clone)]
pub struct SceneManager
{
    scenes: Arc<Mutex<Vec<(SceneType, Box<dyn Scene + Send>)>>>,
    current_scene: Arc<Mutex<Option<SceneType>>>,
}

impl SceneManager
{
    pub fn new() -> Self
    {
        let scenes = vec!
        [
            (SceneType::Menu, Box::new(MenuScene) as Box<dyn Scene + Send>),
            (SceneType::Game, Box::new(GameScene) as Box<dyn Scene + Send>),
        ];

        SceneManager
        {
            scenes: Arc::new(Mutex::new(scenes)),
            current_scene: Arc::new(Mutex::new(Some(SceneType::Menu))),
        }
    }

    pub fn instance() -> Self
    {
        static INSTANCE: OnceLock<SceneManager> = OnceLock::new();
        INSTANCE.get_or_init(SceneManager::new).clone()
    }

    pub fn switch_scene(&self, scene_type: SceneType)
    {
        let mut current_scene = self.current_scene.lock().unwrap();
        *current_scene = Some(scene_type);

        // When switching, run awake() and start() for the new scene
        let mut scenes = self.scenes.lock().unwrap();
        if let Some((_, scene)) = scenes.iter_mut().find(|(st, _)| *st == scene_type)
        {
            scene.awake();
            scene.start();
        }
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

    // Lifecycle Methods
    pub fn awake(&self)
    {
        self.with_current_scene_mut(|scene| scene.awake());
    }

    pub fn start(&self)
    {
        self.with_current_scene_mut(|scene| scene.start());
    }

    pub fn update(&self)
    {
        self.with_current_scene_mut(|scene| scene.update());
    }

    pub fn fixed_update(&self)
    {
        self.with_current_scene_mut(|scene| scene.fixed_update());
    }

    pub fn late_update(&self)
    {
        self.with_current_scene_mut(|scene| scene.late_update());
    }

    pub fn draw(&self)
    {
        self.with_current_scene(|scene| scene.draw());
    }
}
