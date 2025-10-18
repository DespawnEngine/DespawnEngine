use crate::engine::rendering::vertex::MyVertex;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

pub fn create_cube_vertex_buffer(allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[MyVertex]> {
    let vertex_data = [
        // Front face
        MyVertex { position: [-0.5, -0.5, 0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [0.5, -0.5, 0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [0.5, 0.5, 0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 1.0] },
        MyVertex { position: [-0.5, 0.5, 0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 1.0] },
        // Back face
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [0.5, 0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [0.0, 1.0] },
        MyVertex { position: [-0.5, 0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [1.0, 1.0] },
        // Right face
        MyVertex { position: [0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [0.5, -0.5, 0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [0.5, 0.5, 0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 1.0] },
        MyVertex { position: [0.5, 0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [0.0, 1.0] },
        // Left face
        MyVertex { position: [-0.5, -0.5, 0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [-0.5, 0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [1.0, 1.0] },
        MyVertex { position: [-0.5, 0.5, 0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 1.0] },
        // Top face
        MyVertex { position: [-0.5, 0.5, 0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [0.5, 0.5, 0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [0.5, 0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [1.0, 1.0] },
        MyVertex { position: [-0.5, 0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [0.0, 1.0] },
        // Bottom face
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [0.0, 0.0] },
        MyVertex { position: [0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [1.0, 0.0] },
        MyVertex { position: [0.5, -0.5, 0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 1.0] },
        MyVertex { position: [-0.5, -0.5, 0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 1.0] },
    ];

    let index_order = [
        0, 1, 2, 2, 3, 0, // front
        4, 5, 6, 6, 7, 4, // back
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ];

    let full_vertex_data: Vec<MyVertex> = index_order.iter().map(|&i| vertex_data[i]).collect();

    Buffer::from_iter(
        allocator,
        BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default() },
        AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE, ..Default::default() },
        full_vertex_data,
    ).unwrap()
}