use crate::content::block::block::Block;
use crate::utils::registry::Registry;
use crate::engine::resources::load_json5_dir;
use std::sync::{OnceLock, Arc};

/// Singleton instance
pub static GAME_CONTENT: OnceLock<Arc<GameContent>> = OnceLock::new();

pub struct GameContent {
    pub blocks: Registry<Block>,
    // pub items: Registry<Item>,
    // pub entities: Registry<Entity>,
}

impl GameContent {
    pub fn init(content: Arc<GameContent>) -> Arc<GameContent> {
        GAME_CONTENT.get_or_init(|| content.clone()).clone()
    }

    /// Access singleton (read-only)
    pub fn get() -> Arc<GameContent> {
        GAME_CONTENT.get().expect("GameContent not initialized").clone()
    }

    /// Load all content from JSON5 files
    pub fn load_all() -> Self {
        println!("--- Loading game content ---");

        let mut blocks = Registry::new();
        for block in load_json5_dir::<Block>("assets/block") {
            let id = block.id.clone();
            println!("Registering block: {}", id);
            blocks.register(&id, block);
        }

        println!("--- Finished loading game content ---");
        Self {
            blocks,
            // items,
            // entities,
        }
    }
}
