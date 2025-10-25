use crate::content::block::block::Block;
use crate::content::world::chunks::chunk::{CHUNK_SIZE, Chunk};
use crate::engine::core::content_loader::GameContent;
use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;

#[derive(Default)]
pub struct World {
    pub chunks: HashMap<[i32; 3], Arc<Chunk>>,
    pub loaded_chunks: HashMap<[i32; 3], Arc<Chunk>>,
    pub memory_allocator: Option<Arc<StandardMemoryAllocator>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            loaded_chunks: HashMap::new(),
            memory_allocator: None,
        }
    }

    pub fn init(&mut self) {
        self.load_chunk([0, 0, 0]);
        self.load_chunk([2, 0, 0]);
    }

    /// Creates or loads a chunk at the given chunk coordinates
    pub fn get_chunk(&mut self, pos: [i32; 3], content: &GameContent) -> Result<Arc<Chunk>, Error> {
        if let Some(chunk_data) = self.chunks.get(&pos) {
            return Ok(chunk_data.clone());
        }

        let mut chunk = Chunk::new(pos);

        // Fill bottom half with dirt from registry
        if pos[1] > 0 {
            chunk.generate_empty(content);
        } else if pos[1] == 0 {
            chunk.generate_flat("template:dirt", content);
        } else {
            chunk.generate_full("template:dirt", content);
        }

        let chunk_arc = Arc::new(chunk);
        self.chunks.insert(pos, chunk_arc.clone());
        Ok(chunk_arc)
    }

    /// Gets block in world space coordinates using the content registry
    pub fn get_block_world(
        &self,
        wx: i32,
        wy: i32,
        wz: i32,
        content: &GameContent,
    ) -> Option<Arc<Block>> {
        let (cx, lx) = Self::to_chunk_coord(wx);
        let (cy, ly) = Self::to_chunk_coord(wy);
        let (cz, lz) = Self::to_chunk_coord(wz);
        let chunk_pos = [cx, cy, cz];

        self.chunks
            .get(&chunk_pos)
            .and_then(|chunk| chunk.get_block(lx as usize, ly as usize, lz as usize, content))
    }

    /// Converts world coordinate -> chunk plus local coordinate
    #[inline(always)]
    fn to_chunk_coord(world_coord: i32) -> (i32, i32) {
        let chunk = world_coord.div_euclid(CHUNK_SIZE as i32);
        let local = world_coord.rem_euclid(CHUNK_SIZE as i32);

        (chunk, local)
    }

    pub fn set_allocator(&mut self, allocator: Arc<StandardMemoryAllocator>) {
        self.memory_allocator = Some(allocator);
    }

    pub fn load_chunk(&mut self, chunk_pos: [i32; 3]) {
        let chunk = self.get_chunk(chunk_pos, &GameContent::get()).unwrap();

        self.loaded_chunks.insert(chunk_pos, chunk);
    }
    pub fn unload_chunk(&mut self, chunk_pos: [i32; 3]) {
        let _ = self.loaded_chunks.remove(&chunk_pos);
    }
}
