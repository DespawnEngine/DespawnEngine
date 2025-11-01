use std::sync::Arc;
use vulkano::{
    VulkanLibrary,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
        physical::PhysicalDeviceType,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    swapchain::Surface,
};
use winit::event_loop::ActiveEventLoop;

pub fn create_instance(event_loop: &ActiveEventLoop) -> Arc<Instance> {
    let library = VulkanLibrary::new().expect("No local Vulkan library found");
    let required_extensions = Surface::required_extensions(event_loop).unwrap();

    Instance::new(
        library,
        InstanceCreateInfo {
            // Required for compatibility with MoltenVK on macOS.
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("Failed to create instance")
}

// Selects a suitable physical device (GPU) and creates a logical device to interface with it.
pub fn create_device_and_queue(
    instance: Arc<Instance>,
    surface: Arc<Surface>,
) -> (Arc<Device>, Arc<Queue>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..Default::default()
    };

    // Select a physical device that supports our requirements.
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .expect("Could not enumerate physical devices")
        // Must support swapchains.
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        // Must have a queue family that supports graphics and can draw to our surface.
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .find(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(*i as u32, &surface).unwrap_or(false)
                })
                .map(|(i, _)| (p.clone(), i as u32))
        })
        // Prioritize device types: Discrete > Integrated > Virtual > CPU.
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No suitable physical device found");

    // Create a logical device and the command queues.
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
    )
    .expect("Failed to create device");

    (device, queues.next().unwrap())
}
