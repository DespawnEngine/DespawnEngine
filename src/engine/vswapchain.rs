use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageAccess, ImageUsage, SwapchainImage};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{Swapchain, SwapchainCreateInfo, CompositeAlpha};
use vulkano::pipeline::graphics::viewport::Viewport;
use std::sync::Arc;
use winit::window::Window;

// Create a swapchain (manages images for rendering and display)
pub fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<vulkano::swapchain::Surface>,
) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();

    let (image_format, _) = device
        .physical_device()
        .surface_formats(&surface, Default::default())
        .unwrap()
        .into_iter()
        .min_by_key(|(format, _color_space)| match format {
            Format::B8G8R8A8_SRGB => 0,
            _ => 1,
        })
        .unwrap();

    let composite_alpha = if surface_capabilities.supported_composite_alpha.opaque {
        CompositeAlpha::Opaque
    } else {
        CompositeAlpha::PreMultiplied
    };

    let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
    let image_extent: [u32; 2] = window.inner_size().into();

    // New swapchain with specifics settings
    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count,
            image_format: Some(image_format),
            image_extent,
            image_usage: ImageUsage {
                color_attachment: true,
                ..ImageUsage::empty()
            },
            composite_alpha,
            ..Default::default()
        },
    ).unwrap()
}

// Helper function to create framebuffers based on window size
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>], // Swapchain images
    render_pass: Arc<RenderPass>, // RenderPass to hook
    viewport: &mut Viewport, // Viewport to update
    ) -> Vec<Arc<Framebuffer>> {

    // Get image dimensions and update viewport
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    // Create a framebuffer for each image
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
            ).unwrap()
        })
        .collect::<Vec<_>>()
}