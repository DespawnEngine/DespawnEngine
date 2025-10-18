use std::sync::Arc;
use vulkano::buffer::{Subbuffer, BufferCreateInfo, BufferUsage, Buffer};
use vulkano::memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter};

use crate::engine::rendering::vertex::MyVertex;
use crate::engine::rendering::cube;
use crate::content::world::chunks::chunk::{Chunk, CHUNK_SIZE};
use crate::utils::math::Vec3;

/// Builds chunk mesh by adding one cube for each non-air block.
/// TODO: Optimize this with face culling!
pub fn build_chunk_mesh(
    allocator: Arc<StandardMemoryAllocator>,
    chunk: &Chunk,
) -> Subbuffer<[MyVertex]> {
    let mut vertices: Vec<MyVertex> = Vec::new();

    // Get a cube’s vertices (centered at origin)
    let cube_vertices = cube::get_cube_vertices();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let idx = Chunk::index(x, y, z);
                let palette_idx = chunk.blocks[idx] as usize;
                let block_id = &chunk.palette[palette_idx];

                // Ignore air blocks
                if block_id.contains("air") {
                    continue;
                }

                // Add cube vertices offset by block position
                for v in &cube_vertices {
                    let mut v = *v;
                    v.position = Vec3::from([
                        v.position[0] + x as f32,
                        v.position[1] + y as f32,
                        v.position[2] + z as f32,
                    ]);
                    vertices.push(v);
                }
            }
        }
    }

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
    ).unwrap()
}
