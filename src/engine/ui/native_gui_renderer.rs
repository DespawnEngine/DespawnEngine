// a lot of this was lifted and adapted from
// ======================
// https://github.com/hakolao/egui_winit_vulkano/blob/master/src/renderer.rs
// ======================
// but like its awesome so idc + i am adapting it
//

use std::sync::Arc;

use vulkano::{
    command_buffer::{
        allocator::CommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferInheritanceInfo,
        SecondaryAutoCommandBuffer,
    },
    device::Device,
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition, VertexInputState},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
};

use crate::engine::ui::native_gui_vertex::GuiVertex;

#[derive(Clone)]
pub(crate) struct NativeGuiRenderer {
    pub gfx_pipeline: Arc<GraphicsPipeline>,
    pub cmd_buf_allocator: Arc<dyn CommandBufferAllocator>,
    pub subpass: Subpass,
}

impl NativeGuiRenderer {
    pub fn new<Cba: CommandBufferAllocator>(
        device: Arc<Device>,
        subpass: Subpass,
        cmd_buff_allocator: Arc<Cba>,
    ) -> Self {
        NativeGuiRenderer {
            gfx_pipeline: Self::create_pipeline(device, subpass.clone()),
            cmd_buf_allocator: cmd_buff_allocator,
            subpass,
        }
    }

    pub fn create_pipeline(device: Arc<Device>, subpass: Subpass) -> Arc<GraphicsPipeline> {
        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                path: "assets/shaders/native_gui/vertex.glsl"

            }
        }
        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "assets/shaders/native_gui/fragment.glsl"

            }
        }

        let vs = vs::load(device.clone())
            .expect("failed to create shader module")
            .entry_point("main")
            .unwrap();

        let fs = fs::load(device.clone())
            .expect("failed to create shader module")
            .entry_point("main")
            .unwrap();

        let vertex_input_state: Option<VertexInputState> = GuiVertex::per_vertex()
            .definition(&vs)
            .expect("failed to create vertex input state definition eto bleh")
            .into();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .expect("failed to create ui graphics pipeline layout");

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                subpass: Some(
                    vulkano::pipeline::graphics::subpass::PipelineSubpassType::BeginRenderPass(
                        subpass.clone(),
                    ),
                ),
                rasterization_state: Some(RasterizationState::default()),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState{
                        blend: Some(
                            AttachmentBlend{
                                src_alpha_blend_factor: BlendFactor::SrcAlpha,
                                dst_alpha_blend_factor: BlendFactor::DstAlpha,
                                alpha_blend_op: BlendOp::Add,
                                src_color_blend_factor: BlendFactor::SrcAlpha,
                                dst_color_blend_factor: BlendFactor::DstAlpha,
                                color_blend_op: BlendOp::Max,
                                // ..Default::default()
                            }
                        ),
                        ..Default::default()
                    },
                )),
                vertex_input_state,
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .expect("failed to create ui graphcis pipeline")
    }

    pub fn create_secondary_auto_cmd_buf_builder(
        &self,
    ) -> AutoCommandBufferBuilder<SecondaryAutoCommandBuffer> {
        AutoCommandBufferBuilder::secondary(
            self.cmd_buf_allocator.clone(),
            0,
            vulkano::command_buffer::CommandBufferUsage::MultipleSubmit,
            CommandBufferInheritanceInfo {
                render_pass: Some(self.subpass.clone().into()),
                ..Default::default()
            },
        )
        .unwrap()
    }
}
