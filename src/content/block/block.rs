use serde::Deserialize;

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
    pub is_air: bool, // whether this block should be treated as empty/air
    #[serde(default)]
    pub is_solid: bool,
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
    pub fn is_air(&self) -> bool {
        self.properties.is_air
    }

    pub fn is_solid(&self) -> bool {
        self.properties.is_solid
    }
}