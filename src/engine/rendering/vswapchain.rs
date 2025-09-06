use std::sync::Arc;
use vulkano::{
    device::Device,
    image::{view::ImageView, Image, ImageUsage},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{ColorSpace, CompositeAlpha, Surface, Swapchain, SwapchainCreateInfo},
};
use vulkano::format::Format;
use vulkano::image::{ImageCreateInfo, ImageType};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

pub const IMAGE_FORMAT: Format = Format::B8G8R8A8_SRGB;

pub fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    image_extent: [u32; 2],
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();

    // Force linear format

    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count.max(2),
            image_format: IMAGE_FORMAT,
            image_color_space: ColorSpace::SrgbNonLinear,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha: CompositeAlpha::Opaque,
            ..Default::default()
        },
    )
        .unwrap()
}


// Creates framebuffers that link the render pass to the swapchain images.
// This must be reran whenever the window size changes.
pub fn window_size_dependent_setup(
    images: &[Arc<Image>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
    memory_allocator: &Arc<StandardMemoryAllocator>,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].extent();
    viewport.extent = [dimensions[0] as f32, dimensions[1] as f32];

    let depth_image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::D32_SFLOAT,
            extent: [dimensions[0], dimensions[1], 1],
            usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    ).unwrap();

    let depth_view = ImageView::new_default(depth_image.clone()).unwrap();

    images
        .iter()
        .map(|image| {
            let color_view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![color_view, depth_view.clone()],
                    ..Default::default()
                },
            )
                .unwrap()
        })
        .collect()
}
