use serde::Deserialize;
use crate::engine::rendering::texture_atlas::AtlasUV;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub id: String,
    pub name: String,
    pub texture: String,

    #[serde(default)]
    pub properties: BlockProperties,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BlockProperties {
    #[serde(default)]
    pub is_solid: bool,

    // Runtime only, for texture atlas
    #[serde(skip)]
    pub uv_min: [f32; 2],
    #[serde(skip)]
    pub uv_max: [f32; 2],
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: "".to_string(),
            texture: String::new(),
            properties: BlockProperties::default(),
        }
    }
}

// Helper method for checking properties
impl Block {
    pub fn is_solid(&self) -> bool {
        self.properties.is_solid
    }

    pub fn set_uv(&mut self, uv: AtlasUV) {
        self.properties.uv_min = uv.uv_min;
        self.properties.uv_max = uv.uv_max;
    }
}