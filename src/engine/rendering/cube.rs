use crate::engine::rendering::vertex::MyVertex;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

// Create vertex buffer and simple cube rendering
pub fn create_cube_vertex_buffer(allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[MyVertex]> {
    // Vertex positions and colors
    let vertex_data = [
        // Front face
        MyVertex::new([-0.5, -0.5, 0.5], [1.0, 0.0, 0.0]),
        MyVertex::new([0.5, -0.5, 0.5], [0.0, 1.0, 0.0]),
        MyVertex::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
        MyVertex::new([-0.5, 0.5, 0.5], [1.0, 1.0, 0.0]),
        // Back face
        MyVertex::new([-0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),
        MyVertex::new([0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),
        MyVertex::new([0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
        MyVertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
    ];

    let index_order = [
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        5, 4, 7, 7, 6, 5, // back
        4, 0, 3, 3, 7, 4, // left
        3, 2, 6, 6, 7, 3, // top
        4, 5, 1, 1, 0, 4, // bottom
    ];

    let full_vertex_data: Vec<MyVertex> = index_order.iter().map(|&i| vertex_data[i]).collect();

    Buffer::from_iter(
        allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        full_vertex_data,
    )
        .unwrap()
}