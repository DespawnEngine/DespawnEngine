mod arguments;
mod engine;
mod utils;

use engine::core::app::App;
use winit::{
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
};

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"))]
use winit::platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11};

fn main() {
    // Use an EventLoopBuilder for more control over the EventLoop
    let mut event_loop_builder: EventLoopBuilder<()> = EventLoop::builder();

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"))]
    {
        match arguments::backend_to_use() {
            Some(arguments::Backends::Wayland) => event_loop_builder.with_wayland(),
            Some(arguments::Backends::X11) => event_loop_builder.with_x11(),
            None => &mut event_loop_builder,
        };
    }

    // For platforms like Windows or macOS, skip backend selection.
    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly")))]
    {
        let _ = &event_loop_builder; // No-op to avoid unused warning
    }

    let event_loop: EventLoop<()> = event_loop_builder
        .build()
        .expect("Failed to build event loop");

    // Use `Poll` for a continuous loop, ideal for game rendering,
    // rather than waiting for OS events.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
