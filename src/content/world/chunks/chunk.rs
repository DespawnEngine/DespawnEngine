use crate::content::block::block::Block;
use crate::engine::core::content_loader::GameContent;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Single chunk dimensions (16^3 because cubic)
pub const CHUNK_SIZE: usize = 16;

pub const MAX_CHUNK_INDEX: usize = 4095;

/// Hardcoded ID for air (palette index 0)
pub const AIR_BLOCK_ID: &str = "base:air";

#[derive(Clone)]
pub struct Chunk {
    pub position: [i32; 3],
    pub blocks: Vec<u16>,                  // block palette indices
    pub palette: Vec<String>,              // palette: index -> block ID
    pub palette_map: HashMap<String, u16>, // block ID -> palette index
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let mut palette = Vec::new();
        let mut palette_map = HashMap::new();

        // Inserting air into the palette
        palette.push(AIR_BLOCK_ID.to_string());
        palette_map.insert(AIR_BLOCK_ID.to_string(), 0);

        Self {
            position,
            blocks: vec![0; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            palette,
            palette_map,
        }
    }

    pub fn is_air_at_idx(&self, idx: usize) -> bool {
        self.blocks[idx] == 0 // direct index check
    }

    #[inline(always)]
    pub(crate) fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
    }

    /// Sets a block by ID (lookup in the palette)
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: &str) {
        let idx = Self::index(x, y, z);

        // Skip air
        if block_id == AIR_BLOCK_ID {
            self.blocks[idx] = 0;
            return;
        }

        let palette_idx = if let Some(&i) = self.palette_map.get(block_id) {
            i
        } else {
            let i = self.palette.len() as u16;
            self.palette.push(block_id.to_string());
            self.palette_map.insert(block_id.to_string(), i);
            i
        };

        self.blocks[idx] = palette_idx;
    }

    /// Gets a block from the registry with palette index
    pub fn get_block<'a>(
        &self,
        x: usize,
        y: usize,
        z: usize,
        content: &'a GameContent,
    ) -> Option<Arc<Block>> {
        let idx = Self::index(x, y, z);
        let palette_idx = self.blocks[idx] as usize;
        let block_id = self.palette.get(palette_idx)?;
        content.blocks.get(block_id)
    }

    /// Simple halved chunk generation using blocks from the registry
    /// bottom half is dirt, top half is air
    pub fn generate_flat(&mut self, dirt_id: &str, _content: &GameContent) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_id = if y < CHUNK_SIZE / 2 {
                        dirt_id
                    } else {
                        //"template:engine" // 2nd block type to test multi-textures
                        AIR_BLOCK_ID // Air
                    };
                    self.set_block(x, y, z, block_id);
                }
            }
        }
    }

    /// Prints the chunk layer by layer (shows block IDs)
    pub fn print_layers(&self) {
        for y in (0..CHUNK_SIZE).rev() {
            println!("Layer y={}", y);
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let palette_idx = self.blocks[Self::index(x, y, z)] as usize;
                    let id_str = self
                        .palette
                        .get(palette_idx)
                        .cloned()
                        .unwrap_or_else(|| ".".to_string());
                    print!("{:>15} ", id_str);
                }
                println!();
            }
            println!();
        }
    }
}
