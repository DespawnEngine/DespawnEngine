use std::collections::HashMap;
use serde::Deserialize;
use crate::engine::rendering::texture_atlas::AtlasUV;
use crate::engine::resources::load_json5_file;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub id: String,

    #[serde(default)]
    pub block_states: BlockStates,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BlockStates {
    #[serde(default)]
    pub default: Option<BlockState>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BlockState {
    #[serde(default)]
    pub model_name: String, // path to model JSON
    #[serde(skip)]
    pub model: Option<BlockModel>, // loaded at runtime
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: String::new(),
            block_states: BlockStates::default(),
        }
    }
}

impl Block {
    pub fn load_model(&mut self) {
        if let Some(state) = &mut self.block_states.default {
            if !state.model_name.is_empty() {
                // path relative to assets/models/blocks/
                let path = format!("assets/models/blocks/{}", state.model_name);
                // load model JSON into BlockModel struct
                let model: BlockModel = load_json5_file(&path);
                state.model = Some(model);
            }
        }
    }
}

/// Block model definition
#[derive(Deserialize, Debug, Clone)]
pub struct BlockModel {
    #[serde(default)]
    pub textures: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextureEntry {
    pub file_name: String,
}