use crate::content::block::block::Block;
use crate::utils::registry::Registry;
use crate::engine::resources::load_json5_dir;

pub fn load_blocks() -> Registry<Block> {
    let mut registry = Registry::new();

    for block in load_json5_dir::<Block>("assets/block") {
        let id = block.id.clone();
        registry.register(&id, block);
    }

    registry
}
