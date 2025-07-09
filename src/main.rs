mod engine;

use engine::core::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // Use `Poll` for a continuous loop, ideal for game rendering,
    // rather than waiting for OS events.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
