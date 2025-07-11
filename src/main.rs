mod arguments;
mod engine;

use engine::core::app::App;
use winit::{
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11},
};

fn main() {
    // Use an EventLoopBuilder for more control over the EventLoop
    let mut event_loop_builder: EventLoopBuilder<()> = EventLoop::builder();

    match arguments::backend_to_use() {
        Some(arguments::Backends::Wayland) => event_loop_builder.with_wayland(),
        Some(arguments::Backends::X11) => event_loop_builder.with_x11(),
        None => &mut event_loop_builder,
    };

    let event_loop: EventLoop<()> = event_loop_builder
        .build()
        .expect("Failed to build event loop");

    // Use `Poll` for a continuous loop, ideal for game rendering,
    // rather than waiting for OS events.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
