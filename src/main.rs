use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    VulkanLibrary,
    swapchain::Surface, // Used for the window, though not used yet
};

use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

// Struct for this application.
// Holds a window to make sure it stays in memory.
struct DespawnEngine {
    window: Option<Window>, // Prevent the window from being removed.
}

// Implement the winit ApplicationHandler to respond to app.
impl ApplicationHandler for DespawnEngine {
    // Called when the app resumes
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a window with a title.
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Despawn Engine"))
            .expect("Failed to create window");

        // Store the window for persisting.
        self.window = Some(window);
    }

    // Called whenever a window event occurs (like close, resize, input, etc.)
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // Ensure the event is for our tracked window.
        if let Some(ref win) = self.window {
            if window_id == win.id() {
                // Exit this program completely when the window is closed!
                if let WindowEvent::CloseRequested = event {
                    event_loop.exit();
                }
            }
        }
    }
}

fn main() {
    // Load the Vulkan shared library from the system.
    let library = VulkanLibrary::new().expect("No Vulkan library found");

    // Create a Vulkan instance for interacting with the Vulkan API.
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("Failed to create Vulkan instance");

    println!("Vulkan loaded successfully :)");

    // Create an event loop.
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Initialize the engine and run the application.
    let mut app = DespawnEngine { window: None };
    event_loop.run_app(&mut app).expect("Application run failed");
}
