use rapidhash::RapidHashMap;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

use crate::content::world::chunks::chunk::{BLOCKS_IN_CHUNK, CHUNK_SIZE, Chunk};
use crate::engine::rendering::cube;
use crate::engine::rendering::texture_atlas::AtlasUV;
use crate::engine::rendering::vertex::BlockVertex;
use crate::utils::math::Vec3;

#[derive(Default)]
pub struct ChunkMesh {
    pub x_pos: Vec<BlockVertex>,
    pub y_pos: Vec<BlockVertex>,
    pub z_pos: Vec<BlockVertex>,
    pub x_neg: Vec<BlockVertex>,
    pub y_neg: Vec<BlockVertex>,
    pub z_neg: Vec<BlockVertex>,
}

impl ChunkMesh {
    fn is_empty(&self) -> bool {
        self.x_pos.is_empty()
            && self.x_neg.is_empty()
            && self.y_pos.is_empty()
            && self.y_neg.is_empty()
            && self.z_pos.is_empty()
            && self.z_neg.is_empty()
    }
}

/// Builds chunk mesh by adding one cube for each non-air block.
pub fn build_chunk_mesh(
    allocator: Arc<StandardMemoryAllocator>,
    chunk: &Chunk,
    block_uvs: &RapidHashMap<String, AtlasUV>,
) -> Option<ChunkMesh> {
    if chunk.palette.len() == 1 {
        return None; // early return if only air is in the palette
    }
    let mut chunk_mesh = ChunkMesh::default();

    let map_uv = |orig: [f32; 2], atlas: AtlasUV| -> [f32; 2] {
        [
            atlas.uv_min[0] + orig[0] * (atlas.uv_max[0] - atlas.uv_min[0]),
            atlas.uv_min[1] + orig[1] * (atlas.uv_max[1] - atlas.uv_min[1]),
        ]
    };

    let is_air = |idx: usize| chunk.is_air_at_idx(idx);

    for idx in 0..BLOCKS_IN_CHUNK {
        let palette_idx = chunk.blocks[idx] as usize;
        let block_id = &chunk.palette[palette_idx];

        if chunk.is_air_at_idx(idx) {
            continue;
        }

        // deriving the coords instead of nested loops
        let x = idx % CHUNK_SIZE;
        let y = (idx / CHUNK_SIZE) % CHUNK_SIZE;
        let z = idx / (CHUNK_SIZE * CHUNK_SIZE);

        // Precompute neighbor "air" checks
        let top_air = y == CHUNK_SIZE - 1 || is_air(idx + CHUNK_SIZE); // y+
        let bottom_air = y == 0 || is_air(idx - CHUNK_SIZE); // y-

        let front_air = z == CHUNK_SIZE - 1 || is_air(idx + CHUNK_SIZE * CHUNK_SIZE); // z+
        let rear_air = z == 0 || is_air(idx - CHUNK_SIZE * CHUNK_SIZE); // z-

        let left_air = x == CHUNK_SIZE - 1 || is_air(idx + 1); // x+
        let right_air = x == 0 || is_air(idx - 1); // x-

        let atlas = block_uvs.get(block_id).copied().unwrap_or(AtlasUV {
            uv_min: [0.0, 0.0],
            uv_max: [1.0, 1.0],
        });

        // if this ever fails, math has somehow broken.
        debug_assert!(Chunk::index(x, y, z) == idx);

        let pos_offset = Vec3::from([x as f32, y as f32, z as f32])
            + (Vec3::from(chunk.position) * CHUNK_SIZE as f32);

        // conditionally adding vertices if they are on the edge or are next to air
        if top_air {
            chunk_mesh.y_pos.extend(cube::TOP_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if bottom_air {
            chunk_mesh.y_neg.extend(cube::BOTTOM_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }

        if front_air {
            chunk_mesh.z_pos.extend(cube::FRONT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if rear_air {
            chunk_mesh.z_neg.extend(cube::REAR_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }

        if left_air {
            chunk_mesh.x_pos.extend(cube::LEFT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if right_air {
            chunk_mesh.x_neg.extend(cube::RIGHT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
    }

    if !chunk_mesh.is_empty() {
        Some(chunk_mesh)
    } else {
        None
    }
}
