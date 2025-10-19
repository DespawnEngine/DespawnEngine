use crate::content::block::block::Block;
use crate::content::block::block_registry::load_blocks;
use crate::utils::registry::Registry;
use std::sync::{OnceLock, Arc};
use crate::content::block::block::BlockModel;

/// Singleton instance
pub static GAME_CONTENT: OnceLock<Arc<GameContent>> = OnceLock::new();

pub struct GameContent {
    pub blocks: Registry<Block>,
    pub block_model: Registry<BlockModel>,
}

impl GameContent {
    pub fn init(content: Arc<GameContent>) -> Arc<GameContent> {
        GAME_CONTENT.get_or_init(|| content.clone()).clone()
    }

    /// Access singleton (read-only)
    pub fn get() -> Arc<GameContent> {
        GAME_CONTENT.get().expect("GameContent not initialized").clone()
    }

    /// Loads everything from the BlockRegistry
    pub fn load_all() -> Self {
        println!("--- Loading game content ---");

        let (blocks, block_model) = load_blocks();

        println!("--- Finished loading game content ---");
        Self { blocks, block_model }
    }
}
