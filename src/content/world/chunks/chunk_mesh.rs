use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

use crate::content::block::block::Block;
use crate::content::world::chunks::chunk::{CHUNK_SIZE, Chunk, MAX_CHUNK_INDEX};
use crate::engine::rendering::cube;
use crate::engine::rendering::texture_atlas::AtlasUV;
use crate::engine::rendering::vertex::BlockVertex;
use crate::utils::math::Vec3;

/// Builds chunk mesh by adding one cube for each non-air block.
pub fn build_chunk_mesh(
    allocator: Arc<StandardMemoryAllocator>,
    chunk: &Chunk,
    block_uvs: &HashMap<String, AtlasUV>,
) -> Option<Subbuffer<[BlockVertex]>> {
    let start_build = Instant::now();

    let mut vertices = Vec::new();

    let map_uv = |orig: [f32; 2], atlas: AtlasUV| -> [f32; 2] {
        [
            atlas.uv_min[0] + orig[0] * (atlas.uv_max[0] - atlas.uv_min[0]),
            atlas.uv_min[1] + orig[1] * (atlas.uv_max[1] - atlas.uv_min[1]),
        ]
    };

    let is_air = |idx: usize| chunk.is_air_at_idx(idx);

    for idx in 0..MAX_CHUNK_INDEX + 1 {
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
        let top_air = y == CHUNK_SIZE - 1 || is_air(idx + CHUNK_SIZE);
        let bottom_air = y == 0 || is_air(idx - CHUNK_SIZE);
        let rear_air = z == 0 || is_air(idx - CHUNK_SIZE * CHUNK_SIZE);
        let front_air = z == CHUNK_SIZE - 1 || is_air(idx + CHUNK_SIZE * CHUNK_SIZE);
        let right_air = x == 0 || is_air(idx - 1);
        let left_air = x == CHUNK_SIZE - 1 || is_air(idx + 1);

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
            vertices.extend(cube::TOP_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if bottom_air {
            vertices.extend(cube::BOTTOM_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if rear_air {
            vertices.extend(cube::REAR_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if front_air {
            vertices.extend(cube::FRONT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if right_air {
            vertices.extend(cube::RIGHT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
        if left_air {
            vertices.extend(cube::LEFT_FACE.map(|mut v| {
                v.position += pos_offset;
                v.tex_coords = map_uv(v.tex_coords, atlas);
                v
            }));
        }
    }

    println!("it took {:?} to build the chunk", start_build.elapsed());

    if vertices.len() > 0 {
        Some(
            Buffer::from_iter(
                allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                vertices,
            )
            .unwrap(),
        )
    } else {
        None
    }
}
