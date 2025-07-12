use std::sync::Arc;
use vulkano::{
    device::Device,
    image::{view::ImageView, Image, ImageUsage},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{CompositeAlpha, Surface, Swapchain, SwapchainCreateInfo},
};
use vulkano::format::Format;

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
    let image_format = Format::R8G8B8A8_SRGB;

    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count.max(2),
            image_format,
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
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].extent();
    viewport.extent = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
