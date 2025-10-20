use crate::engine::rendering::vertex::BlockVertex;
use crate::utils::math::Vec3;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

// constant layout of a cube's vertices
// yes its too large but think of all the performance gained (none probably)
pub const CUBE_VERTICES: [BlockVertex; 36] = [
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
];

pub const FRONT_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
];

pub const REAR_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
];
pub const RIGHT_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
];
pub const LEFT_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
];
pub const BOTTOM_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, 0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, -0.5, -0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, -0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, -0.5, 0.5]),
        tex_coords: [1.0, 1.0],
    },
];
pub const TOP_FACE: [BlockVertex; 6] = [
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, -0.5]),
        tex_coords: [0.0, 1.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([0.5, 0.5, 0.5]),
        tex_coords: [0.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, 0.5]),
        tex_coords: [1.0, 0.0],
    },
    BlockVertex {
        position: Vec3([-0.5, 0.5, -0.5]),
        tex_coords: [1.0, 1.0],
    },
];

// old function to get the proper vertices for a cube
/// Returns cube vertices
// pub fn get_cube_vertices() -> Vec<BlockVertex> {
//     let vertex_data = [
//         // Front face (z+)
//         BlockVertex::new([-0.5, -0.5, 0.5], [1.0, 1.0]), // E
//         BlockVertex::new([0.5, -0.5, 0.5], [0.0, 1.0]),  // F
//         BlockVertex::new([0.5, 0.5, 0.5], [0.0, 0.0]),   // G
//         BlockVertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),  // H
//         // Rear face (z-)
//         BlockVertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // E
//         BlockVertex::new([0.5, -0.5, -0.5], [1.0, 1.0]),  // F
//         BlockVertex::new([0.5, 0.5, -0.5], [1.0, 0.0]),   // G
//         BlockVertex::new([-0.5, 0.5, -0.5], [0.0, 0.0]),  // H
//         // Left face (x-)
//         BlockVertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // A
//         BlockVertex::new([-0.5, -0.5, 0.5], [1.0, 1.0]),  // E
//         BlockVertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),   // H
//         BlockVertex::new([-0.5, 0.5, -0.5], [0.0, 0.0]),  // D
//         // Right face (x+)
//         BlockVertex::new([0.5, -0.5, -0.5], [1.0, 1.0]), // B
//         BlockVertex::new([0.5, -0.5, 0.5], [0.0, 1.0]),  // F
//         BlockVertex::new([0.5, 0.5, 0.5], [0.0, 0.0]),   // G
//         BlockVertex::new([0.5, 0.5, -0.5], [1.0, 0.0]),  // C
//         // Bottom face (y-)
//         BlockVertex::new([-0.5, -0.5, 0.5], [1.0, 1.0]), // E
//         BlockVertex::new([0.5, -0.5, 0.5], [0.0, 1.0]),  // F
//         BlockVertex::new([0.5, -0.5, -0.5], [0.0, 0.0]), // B
//         BlockVertex::new([-0.5, -0.5, -0.5], [1.0, 0.0]), // A
//         // Top face (y+)
//         BlockVertex::new([-0.5, 0.5, -0.5], [1.0, 1.0]), // D
//         BlockVertex::new([0.5, 0.5, -0.5], [0.0, 1.0]),  // C
//         BlockVertex::new([0.5, 0.5, 0.5], [0.0, 0.0]),   // G
//         BlockVertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),  // H
//     ];
//
//     // indices use CCW winding per face
//     let index_order = [
//         0, 3, 2, 2, 1, 0, // front
//         4, 5, 6, 6, 7, 4, // back
//         8, 11, 10, 10, 9, 8, // left
//         12, 13, 14, 14, 15, 12, // right
//         16, 17, 18, 18, 19, 16, // bottom
//         20, 21, 22, 22, 23, 20, // top
//     ];
//
//     let out = index_order.iter().map(|&i| vertex_data[i]).collect();
//
//     println!("{out:?}\n\n\n");
//
//     out
// }
//
/// Creates a GPU vertex buffer for a unit cube (debug single cube)
pub fn create_cube_vertex_buffer(
    allocator: Arc<StandardMemoryAllocator>,
) -> Subbuffer<[BlockVertex]> {
    let vertices = CUBE_VERTICES.clone();

    Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices,
    )
    .unwrap()
}
