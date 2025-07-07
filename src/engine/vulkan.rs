use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::{Version, VulkanLibrary};
use vulkano_win::VkSurfaceBuild;
use std::sync::Arc;

// Create the main Vulkan instance
pub fn create_instance() -> Arc<Instance> {
    let library = VulkanLibrary::new().unwrap(); // Load Vulkan library
    let extensions = vulkano_win::required_extensions(&library); // Get extensions needed for window display

    // Create Vulkan instance with specific settings
    Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: extensions, // Enable window-related extensions
            enumerate_portability: true, // Think this allows macOS compatibility (MoltenVK)
            max_api_version: Some(Version::V1_1), // Uses Vulkan 1.1 or higher
            ..Default::default() // Default settings for every other option
        },
    ).unwrap()
}

pub fn create_device_and_queue(instance: Arc<Instance>, surface: Arc<vulkano::swapchain::Surface>) -> (Arc<Device>, Arc<vulkano::device::Queue>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true, // Enable swapchain for rendering to window, still need to figure out what exactly this means
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type { // Prefer discrete GPU over integrated, CPU, etc.
            PhysicalDeviceType::DiscreteGpu => 0, // Dedicated GPU
            PhysicalDeviceType::IntegratedGpu => 1, // Integrated graphics
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No suitable physical device found");

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).unwrap();

    let queue = queues.next().unwrap();
    (device, queue)
}