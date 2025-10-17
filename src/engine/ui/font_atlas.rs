use std::sync::Arc;

use fontdue::{Font, Metrics};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CopyBufferInfo, CopyBufferToImageInfo, PrimaryAutoCommandBuffer,
    },
    format::Format,
    image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView},
    memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter},
};

#[derive(Clone)]
pub struct FontAtlas {
    pub font_name: String,
    pub size: f32,
    pub bitmaps: Vec<(Metrics, Vec<u8>)>,
    pub image_views: Vec<Arc<ImageView>>,
}

impl FontAtlas {
    pub fn new(
        font: Font,
        size: f32,
        gpu_buff_allocator: Arc<dyn MemoryAllocator>,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) -> Self {
        let mut bitmaps = vec![];
        let mut image_views = vec![];
        for glpyh_idx in 0..font.clone().glyph_count() {
            let (metrics, bitmap) = font.rasterize_indexed(glpyh_idx, size);
            if metrics.height * metrics.width == 0 {
                continue;
            }
            let image = Image::new(
                gpu_buff_allocator.clone(),
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format: Format::R8_SRGB,
                    extent: [metrics.width as u32, metrics.height as u32, 1],
                    usage: ImageUsage::SAMPLED | ImageUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                    ..Default::default()
                },
            )
            .expect("failed to create image for somethin");
            let buffer = Buffer::from_iter(
                gpu_buff_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                bitmap.clone(),
            )
            .expect("failed to create src buffer");

            builder
                .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(buffer, image.clone()))
                .expect("failed to copy buffer to image");

            bitmaps.push((metrics, bitmap));
            image_views.push(ImageView::new_default(image.clone()).unwrap());
        }

        FontAtlas {
            font_name: font.name().unwrap().into(),
            size,
            bitmaps,
            image_views,
        }
    }
}
