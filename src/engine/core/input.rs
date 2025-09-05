use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone)]
pub struct InputState {
    pub esc_pressed: bool,
    pub w_pressed: bool,
    pub s_pressed: bool,
    pub a_pressed: bool,
    pub d_pressed: bool,
    pub space_pressed: bool,
    pub shift_pressed: bool,
    pub mouse_delta_x: f32,
    pub mouse_delta_y: f32,
}

impl InputState {
    pub fn handle_events(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    match keycode {
                        KeyCode::Escape => self.esc_pressed = pressed,
                        KeyCode::KeyW => self.w_pressed = pressed,
                        KeyCode::KeyS => self.s_pressed = pressed,
                        KeyCode::KeyA => self.a_pressed = pressed,
                        KeyCode::KeyD => self.d_pressed = pressed,
                        KeyCode::Space => self.space_pressed = pressed,
                        KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift_pressed = pressed,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    pub fn reset_deltas(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
    }
    
    pub fn update_mouse(&mut self, delta: (f64, f64)){
        self.mouse_delta_x = delta.0 as f32;
        self.mouse_delta_y = delta.1 as f32;

    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            esc_pressed: false,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            space_pressed: false,
            shift_pressed: false,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,
        }
    }
}
