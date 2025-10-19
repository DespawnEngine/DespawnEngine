use crate::content::block::block::{Block, BlockModel};
use crate::utils::registry::Registry;
use crate::engine::resources::{load_json5_dir, load_json5_file};
use std::collections::HashMap;

pub fn load_blocks() -> (Registry<Block>, Registry<BlockModel>) {
    let mut blocks = Registry::new();
    let mut block_models = Registry::new();

    // load all block definitions
    for mut block in load_json5_dir::<Block>("assets/data/blocks") {
        if let Some(state) = &mut block.block_states.default {
            if !state.model_name.is_empty() {
                let model_path = format!("assets/models/blocks/{}", state.model_name);
                let model: BlockModel = load_json5_file(&model_path);

                // register model in models registry
                block_models.register(&state.model_name, model.clone());
                state.model = Some(model);
            }
        }
        blocks.register(&block.id.clone(), block.clone());
    }

    (blocks, block_models)
}
