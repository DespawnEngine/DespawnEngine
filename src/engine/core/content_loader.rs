use crate::content::block::block::Block;
use crate::utils::registry::Registry;
use crate::engine::resources::load_json5_dir;

pub struct GameContent {
    pub blocks: Registry<Block>,
    // pub items: Registry<Item>,
    // pub entities: Registry<Entity>,
}

impl GameContent {
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
