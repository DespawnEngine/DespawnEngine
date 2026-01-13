use foldhash::HashMap;

use crate::{content::world::chunks::chunk::ChunkCoords, engine::rendering::vertex::BlockVertex};

const VERTEX_BUFFER_MAX_SIZE: usize = 1024 * 1024 * 10;
pub const VERTEX_BUFFER_MAX_LEN: usize = VERTEX_BUFFER_MAX_SIZE / size_of::<BlockVertex>();

pub struct VertexBuffer {
    verticies: [BlockVertex; VERTEX_BUFFER_MAX_LEN],
    current_position: usize,
    chunk_mesh_information: HashMap<ChunkCoords, (usize, usize)>, //start_pos, len (in verts)
}

impl VertexBuffer {
    const START_POS: usize = 0;

    pub fn new() -> Self {
        Self {
            verticies: [BlockVertex::ZERO; VERTEX_BUFFER_MAX_LEN],
            current_position: 0,
            chunk_mesh_information: HashMap::default(),
        }
    }

    pub fn get_verticies(self) -> [BlockVertex; VERTEX_BUFFER_MAX_LEN] {
        self.verticies
    }

    pub fn add_chunk_mesh(
        &mut self,
        chunk_pos: ChunkCoords,
        chunk_mesh: Vec<BlockVertex>,
    ) -> Result<bool, i32> {
        let chunk_len = chunk_mesh.len();

        self.chunk_mesh_information
            .insert(chunk_pos, (self.current_position, chunk_len));

        let possible_pos = self.can_fit_chunk_len(chunk_mesh.len()).clone();

        match possible_pos {
            Some(pos) => {
                for (buff_vert, mesh_vert) in
                    self.verticies.iter_mut().skip(pos).zip(chunk_mesh.iter())
                {
                    *buff_vert = mesh_vert.clone();
                }

                // current_position += chunk_len;

                // self.current_position = current_position;

                return Ok(true);
            }
            None => return Err(1),
        }
    }

    pub fn can_fit_chunk(&self, chunk_mesh: Vec<BlockVertex>) -> Option<usize> {
        let mut current_pos = Self::START_POS;
        let chunk_mesh_length = chunk_mesh.len();
        for (_chunk_pos, (start_pos, len)) in self.chunk_mesh_information.clone() {
            if current_pos + chunk_mesh_length < start_pos {
                return Some(current_pos);
            }

            current_pos = len - start_pos;

            if current_pos + chunk_mesh_length > VERTEX_BUFFER_MAX_LEN - 128 {
                return None;
            }
        }
        return None;
    }

    pub fn can_fit_chunk_len(&self, chunk_mesh_len: usize) -> Option<usize> {
        let mut current_pos = Self::START_POS;
        for (_chunk_pos, (start_pos, len)) in self.chunk_mesh_information.clone() {
            if current_pos + chunk_mesh_len < start_pos {
                return Some(current_pos);
            }

            current_pos = len - start_pos;

            if current_pos + chunk_mesh_len > VERTEX_BUFFER_MAX_LEN - 128 {
                return None;
            }
        }
        return None;
    }

    pub fn remove_chunk_mesh(mut self, chunk_pos: ChunkCoords) {
        if let Some((start_pos, len)) = self.chunk_mesh_information.get_mut(&chunk_pos) {
            for buff_vert in self.verticies.iter_mut().skip(*start_pos).take(*len) {
                *buff_vert = BlockVertex::ZERO
            }
            self.chunk_mesh_information.remove_entry(&chunk_pos);
        }
    }
}
