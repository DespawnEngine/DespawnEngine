use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

use crate::content::block::block::Block;
use crate::content::world::chunks::chunk::{CHUNK_SIZE, Chunk, MAX_CHUNK_INDEX};
use crate::engine::rendering::cube;
use crate::engine::rendering::vertex::BlockVertex;
use crate::utils::math::Vec3;

/// Builds chunk mesh by adding one cube for each non-air block.
/// TODO: Optimize this with face culling!
pub fn build_chunk_mesh(
    allocator: Arc<StandardMemoryAllocator>,
    chunk: &Chunk,
) -> Subbuffer<[BlockVertex]> {
    let start_build: Instant = Instant::now();
    let mut vertices: Vec<BlockVertex> = Vec::new();

    // + 1 because rust is weird
    for idx in 0..MAX_CHUNK_INDEX + 1 {
        let palette_idx = chunk.blocks[idx] as usize;
        let block_id = &chunk.palette[palette_idx];

        if block_id.contains("air") {
            continue;
        }

        // deriving the coords instead of nested loops
        let x: usize = idx % CHUNK_SIZE;
        let y: usize = (idx % (CHUNK_SIZE * CHUNK_SIZE)).div_euclid(CHUNK_SIZE);
        let z: usize = idx.div_euclid(CHUNK_SIZE * CHUNK_SIZE);

        // if this ever fails, math has somehow broken.
        debug_assert!(Chunk::index(x, y, z) == idx);

        // conditionally adding vertices if they are on the edge or are next to air
        if y == 15 || chunk.is_air_at_idx(idx + CHUNK_SIZE) {
            vertices.append(&mut Vec::from(cube::TOP_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
        if y == 0 || chunk.is_air_at_idx(idx - CHUNK_SIZE) {
            vertices.append(&mut Vec::from(cube::BOTTOM_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
        if z == 0 || chunk.is_air_at_idx(idx - (CHUNK_SIZE * CHUNK_SIZE)) {
            vertices.append(&mut Vec::from(cube::REAR_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
        if z == 15 || chunk.is_air_at_idx(idx + (CHUNK_SIZE * CHUNK_SIZE)) {
            vertices.append(&mut Vec::from(cube::FRONT_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
        if x == 0 || chunk.is_air_at_idx(idx - 1) {
            vertices.append(&mut Vec::from(cube::RIGHT_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
        if x == 15 || chunk.is_air_at_idx(idx + 1) {
            vertices.append(&mut Vec::from(cube::LEFT_FACE.map(|mut v| {
                v.position += Vec3::from([x as f32, y as f32, z as f32]);
                v
            })));
        }
    }

    println!("it took {:?} to build the chunk", start_build.elapsed());

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
    .unwrap()
}
