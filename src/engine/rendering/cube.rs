use crate::engine::rendering::vertex::MyVertex;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

pub fn create_cube_vertex_buffer(allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[MyVertex]> {
    let vertex_data = [
        // Front face (z+)
        MyVertex { position: [-0.5, -0.5,  0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 0.0] }, // E
        MyVertex { position: [ 0.5, -0.5,  0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 0.0] }, // F
        MyVertex { position: [ 0.5,  0.5,  0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // G
        MyVertex { position: [-0.5,  0.5,  0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 1.0] }, // H

        // Back face (z-)
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [1.0, 0.0] }, // A
        MyVertex { position: [ 0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [0.0, 0.0] }, // B
        MyVertex { position: [ 0.5,  0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [0.0, 1.0] }, // C
        MyVertex { position: [-0.5,  0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // D

        // Left face (x-)
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [1.0, 0.0] }, // A
        MyVertex { position: [-0.5, -0.5,  0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 0.0] }, // E
        MyVertex { position: [-0.5,  0.5,  0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 1.0] }, // H
        MyVertex { position: [-0.5,  0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // D

        // Right face (x+)
        MyVertex { position: [0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [0.0, 0.0] }, // B
        MyVertex { position: [0.5, -0.5,  0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 0.0] }, // F
        MyVertex { position: [0.5,  0.5,  0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // G
        MyVertex { position: [0.5,  0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [0.0, 1.0] }, // C

        // Bottom face (y-)
        MyVertex { position: [-0.5, -0.5,  0.5].into(), color: [1.0, 0.0, 0.0].into(), tex_coords: [0.0, 0.0] }, // E
        MyVertex { position: [ 0.5, -0.5,  0.5].into(), color: [0.0, 1.0, 0.0].into(), tex_coords: [1.0, 0.0] }, // F
        MyVertex { position: [ 0.5, -0.5, -0.5].into(), color: [0.0, 1.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // B
        MyVertex { position: [-0.5, -0.5, -0.5].into(), color: [1.0, 0.0, 1.0].into(), tex_coords: [0.0, 1.0] }, // A

        // Top face (y+)
        MyVertex { position: [-0.5, 0.5, -0.5].into(), color: [1.0, 1.0, 1.0].into(), tex_coords: [0.0, 0.0] }, // D
        MyVertex { position: [ 0.5, 0.5, -0.5].into(), color: [0.5, 0.5, 0.5].into(), tex_coords: [1.0, 0.0] }, // C
        MyVertex { position: [ 0.5, 0.5,  0.5].into(), color: [0.0, 0.0, 1.0].into(), tex_coords: [1.0, 1.0] }, // G
        MyVertex { position: [-0.5, 0.5,  0.5].into(), color: [1.0, 1.0, 0.0].into(), tex_coords: [0.0, 1.0] }, // H
    ];

    // indices use CCW winding per face
    let index_order = [
        0, 3, 2, 2, 1, 0, // front
        4, 5, 6, 6, 7, 4, // back
        8, 11, 10, 10, 9, 8, // left
        12, 13, 14, 14, 15, 12, // right
        16, 17, 18, 18, 19, 16, // bottom
        20, 21, 22, 22, 23, 20, // top
    ];

    let full_vertex_data: Vec<MyVertex> = index_order.iter().map(|&i| vertex_data[i]).collect();

    Buffer::from_iter(
        allocator,
        BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default() },
        AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE, ..Default::default() },
        full_vertex_data,
    ).unwrap()
}