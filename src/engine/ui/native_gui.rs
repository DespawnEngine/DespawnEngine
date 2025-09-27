// a lot of this was lifted and adapted from
// ======================
// https://github.com/hakolao/egui_winit_vulkano/blob/master/src/renderer.rs
// ======================
// its awesome
//

use std::{default, fs::File, io::BufReader, sync::Arc};

use png;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::CommandBufferAllocator, AutoCommandBufferBuilder, CopyBufferToImageInfo, PrimaryAutoCommandBuffer, SecondaryAutoCommandBuffer
    },
    descriptor_set::{
        self,
        allocator::{
            DescriptorSetAlloc, StandardDescriptorSetAllocator,
            StandardDescriptorSetAllocatorCreateInfo,
        },
        layout::{
            DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags,
            DescriptorSetLayoutCreateInfo, DescriptorType,
        },
        DescriptorSet, WriteDescriptorSet,
    },
    device::Device,
    format::Format,
    image::{
        sampler::{Sampler, SamplerCreateInfo},
        view::{ImageView, ImageViewCreateInfo},
        Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    memory::allocator::{
        AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::Pipeline,
    render_pass::Subpass,
    shader::DescriptorBindingRequirements,
    DeviceSize,
};

use crate::engine::ui::native_gui_vertex::GuiVertex;
use crate::engine::ui::{native_gui_renderer::NativeGuiRenderer, ui_element::UiElement};

// yeah im basically copying the structure of egui integration stuff, its good ok
pub struct NativeGui {
    device: Arc<Device>,
    renderer: NativeGuiRenderer,
    gpu_buff_allocator: Arc<dyn MemoryAllocator>,
    ui_elements: Vec<Arc<dyn UiElement>>,
    texture: Option<Arc<ImageView>>,
}

impl NativeGui {
    pub fn new<Cba: CommandBufferAllocator, Gba: MemoryAllocator>(
        device: Arc<Device>,
        subpass: Subpass,
        cmd_buff_allocator: Arc<Cba>,
        gpu_buff_allocator: Arc<Gba>,
        ui_elements: Vec<Arc<dyn UiElement>>,
    ) -> Self {
        NativeGui {
            device: device.clone(),
            renderer: NativeGuiRenderer::new(device, subpass, cmd_buff_allocator),
            gpu_buff_allocator,
            ui_elements,
            texture: None,
        }
    }

    pub fn copy_buffer_data_to_image(&mut self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>){
        // https://stackoverflow.com/questions/77882994/what-is-the-process-for-uploading-raw-pixels-into-an-image-texture-using-the-rus
        // yay thanksies <33
        let decoder = png::Decoder::new(BufReader::new(
            File::open("/home/onezoop/Projects/Despawn/DespawnEngine/assets/diamond.png").unwrap(),
        ));
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let extent = [info.width, info.height, 1];

        let upload_buffer = Buffer::new_slice(
            self.gpu_buff_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (info.width * info.height * 4) as DeviceSize,
        )
        .unwrap();

        reader
            .next_frame(&mut upload_buffer.write().unwrap())
            .unwrap();

        let image = Image::new(
            self.gpu_buff_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent,
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap();

        let texture = ImageView::new_default(image.clone()).unwrap();

        self.texture = Some(texture);

        builder.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                upload_buffer,
                image.clone(),
            ))
            .unwrap();
    }

    fn draw_on_subpass_image(
        &self,
        vtx_buff: Subbuffer<[GuiVertex]>,
    ) -> Arc<SecondaryAutoCommandBuffer> {
        let pipeline = self.renderer.gfx_pipeline.clone();

        //stolen from tutorial
        // Create a buffer in device-local memory with enough space for a slice of `10_000` floats.
        // let device_local_buffer = Buffer::from_iter(
        //     self.gpu_buff_allocator.clone(),
        //     BufferCreateInfo {
        //         // Specify use as a storage buffer
        //         usage: BufferUsage::STORAGE_BUFFER | BufferUsage::UNIFORM_BUFFER,
        //         ..Default::default()
        //     },
        //     AllocationCreateInfo {
        //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
        //             | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        //         ..Default::default()
        //     },
        //     *include_bytes!("../../../assets/diamond.png"),
        // )
        // .expect("failed to create device test dummy buffer");

        let desc_set_alloc = Arc::new(StandardDescriptorSetAllocator::new(
            self.device.clone(),
            StandardDescriptorSetAllocatorCreateInfo::default(),
        ));

        
        let layout = self
            .renderer
            .gfx_pipeline
            .layout()
            .set_layouts()
            .first()
            .expect("failed to get descriptor set layout from pipeline");

        let descriptor_set = DescriptorSet::new(
            desc_set_alloc,
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                self.texture.clone().unwrap(),
                Sampler::new(self.device.clone(), SamplerCreateInfo::default())
                    .expect("failed to create sampler???? somehow???"),
            )],
            [],
        )
        .expect("failed to create descriptor set");

        // we gotta do this weird stuff to satisfy the borrow checker, among other things
        let mut builder = self.renderer.create_secondary_auto_cmd_buf_builder();

        builder
            .bind_pipeline_graphics(pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                pipeline.layout().clone(),
                0,
                descriptor_set,
            )
            .unwrap()
            .bind_vertex_buffers(0, vtx_buff.clone())
            .unwrap();
        unsafe {
            let _ = builder.draw(vtx_buff.len() as u32, 1, 0, 0);
        }
        // tbh its not that weird but its kinda annoying
        let buffer = builder.build();
        buffer.clone().unwrap()
    }

    //i dont think is really necessary but like i can't resist a good handler function
    pub fn has_elements(&self) -> bool {
        !self.ui_elements.is_empty()
    }

    // kinda wack stuff, but good for testing
    pub fn draw_elements(
        &self,
        allocator: Arc<StandardMemoryAllocator>,
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.draw_on_subpass_image(self.build_verts_from_elements(allocator))
    }

    // very strange stuff really, gotta change this eventually
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
}
