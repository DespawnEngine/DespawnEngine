// a lot of this was lifted and adapted from
// ======================
// https://github.com/hakolao/egui_winit_vulkano/blob/master/src/renderer.rs
// ======================
// but like its awesome so idc + i am adapting it
//

use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferUsage, Subbuffer},
    command_buffer::{allocator::CommandBufferAllocator, SecondaryAutoCommandBuffer},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    render_pass::Subpass,
};

use crate::engine::ui::native_gui_vertex::GuiVertex;
use crate::engine::ui::{native_gui_renderer::NativeGuiRenderer, ui_element::UiElement};

pub struct NativeGui {
    renderer: NativeGuiRenderer,
    ui_elements: Vec<Arc<dyn UiElement>>,
}

impl NativeGui {
    pub fn new<Cba: CommandBufferAllocator>(
        device: Arc<Device>,
        subpass: Subpass,
        cmd_buff_allocator: Arc<Cba>,
        ui_elements: Vec<Arc<dyn UiElement>>,
    ) -> Self {
        NativeGui {
            renderer: NativeGuiRenderer::new(device, subpass, cmd_buff_allocator),
            ui_elements,
        }
    }
    fn draw_on_subpass_image(
        &self,
        vtx_buff: Subbuffer<[GuiVertex]>,
    ) -> Arc<SecondaryAutoCommandBuffer> {
        let pipeline = self.renderer.gfx_pipeline.clone();
        let mut builder = self.renderer.create_secondary_auto_cmd_buf_builder();
        builder
            .bind_pipeline_graphics(pipeline)
            .unwrap()
            .bind_vertex_buffers(0, vtx_buff.clone())
            .unwrap();
        unsafe {
            let _ = builder.draw(vtx_buff.len() as u32, 1, 0, 0);
        }
        let buffer = builder.build();
        buffer.clone().unwrap()
    }
    pub fn has_elements(&self) -> bool {
        !self.ui_elements.is_empty()
    }

    pub fn draw_elements(
        &self,
        allocator: Arc<StandardMemoryAllocator>,
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.draw_on_subpass_image(self.build_verts_from_elements(allocator))
    }

    fn build_verts_from_elements(
        &self,
        allocator: Arc<StandardMemoryAllocator>,
    ) -> Subbuffer<[GuiVertex]> {
        let mut total_vtx_buffer: Vec<GuiVertex> = vec![];
        for element in self.ui_elements.clone() {
            total_vtx_buffer.append(&mut element.get_mesh());
        }
        Buffer::from_iter(
            allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            total_vtx_buffer,
        )
        .unwrap()
    }

    pub fn test_square_vertex_buffer(
        allocator: Arc<StandardMemoryAllocator>,
    ) -> Subbuffer<[GuiVertex]> {
        const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

        let squares_verts = [
            GuiVertex::new([-1.0, 0.0], [1.0, 0.0, 0.0, 0.2]),
            GuiVertex::new([-1.0, 1.0], [0.0, 1.0, 0.0, 0.2]),
            GuiVertex::new([1.0, 0.0], [0.0, 0.0, 1.0, 0.2]),
            GuiVertex::new([1.0, 1.0], [1.0, 1.0, 1.0, 0.2]),
        ];

        let index_order = [0, 1, 2, 1, 2, 3];

        let full_vertex_data: Vec<GuiVertex> =
            index_order.iter().map(|&i| squares_verts[i]).collect();

        Buffer::from_iter(
            allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
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
}
