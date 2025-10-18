use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub id: String,
    pub name: String,
    pub texture: String,
}
