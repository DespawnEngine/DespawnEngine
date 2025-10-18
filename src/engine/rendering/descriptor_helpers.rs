use std::sync::Arc;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::mvp::MVP;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::image::view::ImageView;
use vulkano::image::sampler::Sampler;

/// Create an MVP uniform buffer and descriptor set with texture sampler.
/// Returns `Some(Arc<DescriptorSet>)` on success, otherwise `None`.
pub fn make_mvp_descriptor_set(
    memory_allocator: &Arc<StandardMemoryAllocator>,
    descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
    layout: &Arc<DescriptorSetLayout>,
    camera: &Camera,
    texture_view: &Arc<ImageView>,
    sampler: &Arc<Sampler>,
) -> Option<Arc<DescriptorSet>> {
    let mvp = MVP::default().apply_camera_transforms(*camera);

    let mvp_buffer = Buffer::from_data(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        mvp,
    ).ok()?;

    let mut writes = vec![WriteDescriptorSet::buffer(0, mvp_buffer)];
    writes.push(WriteDescriptorSet::image_view_sampler(1, texture_view.clone(), sampler.clone()));

    let set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        layout.clone(),
        writes,
        [],
    ).ok()?;

    Some(set)
}