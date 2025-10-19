use std::collections::HashMap;
use std::sync::Arc;
use image::{RgbaImage, GenericImage, GenericImageView};
use vulkano::device::{DeviceOwned, Queue};
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter};
use vulkano::format::Format;

use crate::content::block::block::Block;
use crate::engine::core::content_loader::GameContent;

/// UV mapping for one block in the atlas
#[derive(Debug, Clone, Copy)]
pub struct AtlasUV {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
}

/// Contains the generated atlas and lookup table
pub struct TextureAtlas {
    pub image_view: Arc<ImageView>,
    pub block_uvs: HashMap<String, AtlasUV>,
}

impl TextureAtlas {
    // TODO: This will actually create a new texture in the atlas for every defined texture. Meaning duplicates can arise. Fix.
    /// Creates the texture atlas from all block textures
    pub fn generate(
        allocator: Arc<StandardMemoryAllocator>,
        queue: Arc<Queue>,
        content: &GameContent,
    ) -> Self {
        println!("--- Generating Texture Atlas ----");

        // Get block textures
        let blocks: Vec<&Block> = content.blocks.iter().map(|(_, b)| b.as_ref()).collect();
        let count = blocks.len();
        assert!(count > 0, "No block textures found.");

        // Load all block images
        let mut images: Vec<(String, RgbaImage)> = Vec::new();
        for block in &blocks {
            let path = format!("assets/{}", block.texture);
            match image::open(&path) {
                Ok(img) => {
                    let rgba = img.to_rgba8();
                    images.push((block.id.clone(), rgba));
                }
                Err(e) => {
                    println!("Failed to load texture for {}: {}", block.id, e);
                }
            }
        }

        // Assume all block textures are the same size for now //TODO: Maybe change that.
        let tile_size = images[0].1.width();
        let tiles_per_row = (images.len() as f32).sqrt().ceil() as u32;
        let atlas_width = tile_size * tiles_per_row;
        let atlas_height = tile_size * ((images.len() as u32 + tiles_per_row - 1) / tiles_per_row);

        // Create empty atlas
        let mut atlas = RgbaImage::new(atlas_width, atlas_height);

        let mut block_uvs = HashMap::new();

        // Paste each block image into atlas
        for (i, (id, img)) in images.iter().enumerate() {
            let x = (i as u32 % tiles_per_row) * tile_size;
            let y = (i as u32 / tiles_per_row) * tile_size;

            atlas.copy_from(img, x, y).unwrap();

            // Calculate normalized UVs
            let uv_min = [
                x as f32 / atlas_width as f32,
                y as f32 / atlas_height as f32,
            ];
            let uv_max = [
                (x + tile_size) as f32 / atlas_width as f32,
                (y + tile_size) as f32 / atlas_height as f32,
            ];

            block_uvs.insert(id.clone(), AtlasUV { uv_min, uv_max });
        }

        // This saves a debug copy so I can make sure it works fine and see what's going on easily.
        //atlas.save("atlas_debug.png").unwrap();
        //println!("Saved texture atlas to atlas_debug.png ({}x{})", atlas_width, atlas_height);

        // ---- Upload to GPU ----
        let img_data = atlas.into_raw();

        use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
        use vulkano::command_buffer::{allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo};
        use vulkano::sync::{self, GpuFuture};

        let texture_image = Image::new(
            allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent: [atlas_width, atlas_height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )
            .unwrap();

        let staging = Buffer::from_iter(
            allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            img_data,
        )
            .unwrap();

        // Allocate command buffer properly
        let command_allocator = Arc::new(StandardCommandBufferAllocator::new(
            allocator.device().clone(),
            Default::default(),
        ));

        let mut builder = AutoCommandBufferBuilder::primary(
            command_allocator.clone(),
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(allocator.device().clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let image_view = ImageView::new_default(texture_image.clone()).unwrap();

        println!("Texture Atlas uploaded to GPU, {}x{}", atlas_width, atlas_height);

        Self { image_view, block_uvs }
    }
}
