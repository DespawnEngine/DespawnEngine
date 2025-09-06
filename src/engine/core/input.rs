use std::collections::HashMap;

use fixedstr::zstr;
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum KeyState {
    #[default]
    Unpressed,
    JustPressed,
    Held,
}

impl KeyState {
    fn is_pressed(&self) -> bool {
        self == &KeyState::JustPressed || self == &KeyState::Held
    }
    fn is_just_pressed(&self) -> bool {
        self == &KeyState::JustPressed
    }

    fn update_state(&self, state: ElementState) -> KeyState {
        match self {
            KeyState::Unpressed => match state {
                ElementState::Pressed => KeyState::JustPressed,
                ElementState::Released => KeyState::Unpressed,
            },
            KeyState::JustPressed => match state {
                ElementState::Pressed => KeyState::Held,
                ElementState::Released => KeyState::Unpressed,
            },
            KeyState::Held => match state {
                ElementState::Pressed => KeyState::Held,
                ElementState::Released => KeyState::Unpressed,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct KeyBind {
    name: zstr<32>,
}

impl KeyBind {
    pub fn new<T: Into<zstr<32>>>(name: T) -> Self {
        KeyBind { name: name.into() }
    }
}

#[derive(Debug, Clone)]
pub struct InputState {
    pub keybind_states: HashMap<KeyBind, KeyState>,
    pub key_states: HashMap<KeyCode, KeyBind>,
    pub mouse_delta_x: f32,
    pub mouse_delta_y: f32,
}

impl InputState {
    pub fn handle_events(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key
                    && let Some(keybind) = self.key_states.get(&keycode)
                {
                    self.update_keybind(*keybind, event.state);
                }
            }
            _ => {}
        }
        //println!("{:?}", self.keybind_states);
    }

    #[inline]
    fn get_keybind_state(&self, keybind: KeyBind) -> KeyState {
        *self
            .keybind_states
            .get(&keybind)
            .expect("supplied keybind is not is the keybind states! likely not a real keybind")
    }
#[inline]
    pub fn get_keybind_is_pressed(&self, keybind: KeyBind) -> bool {
        self.get_keybind_state(keybind).is_pressed()
    }
    #[inline]
    pub fn get_keybind_is_just_pressed(&self, keybind: KeyBind) -> bool {
        self.get_keybind_state(keybind).is_just_pressed()
    }

    #[inline]
    fn set_keybind_state(&mut self, keybind: KeyBind, new_state: KeyState) {
        *self.keybind_states.get_mut(&keybind).expect(
            "failed to get the mutable reference of supplied keybind, probably your fault.",
        ) = new_state;
    }

    #[inline]
    fn update_keybind(&mut self, keybind: KeyBind, state: ElementState) {
        self.set_keybind_state(keybind, self.get_keybind_state(keybind).update_state(state));
    }

    pub fn update_just_pressed_into_held(&mut self){
        for (keybind, key_state) in self.keybind_states.iter_mut(){
            *key_state = match key_state{
                KeyState::JustPressed => KeyState::Held,
                _ => *key_state
            }
        }
    }

    pub fn reset_deltas(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
    }

    pub fn update_mouse(&mut self, delta: (f64, f64)) {
        self.mouse_delta_x = delta.0 as f32;
        self.mouse_delta_y = delta.1 as f32;
    }
}

impl Default for InputState {
    fn default() -> Self {
        let mut keybind_map: HashMap<KeyBind, KeyState> = HashMap::new();
        let mut key_map: HashMap<KeyCode, KeyBind> = HashMap::new();

        keybind_map.insert(KeyBind::new("FreeMouse"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgForward"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgBackward"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgLeft"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgRight"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgUp"), KeyState::Unpressed);
        keybind_map.insert(KeyBind::new("DbgDown"), KeyState::Unpressed);

        key_map.insert(KeyCode::Escape, KeyBind::new("FreeMouse"));
        key_map.insert(KeyCode::KeyW, KeyBind::new("DbgForward"));
        key_map.insert(KeyCode::KeyS, KeyBind::new("DbgBackward"));
        key_map.insert(KeyCode::KeyA, KeyBind::new("DbgLeft"));
        key_map.insert(KeyCode::KeyD, KeyBind::new("DbgRight"));
        key_map.insert(KeyCode::Space, KeyBind::new("DbgUp"));
        key_map.insert(KeyCode::ShiftLeft, KeyBind::new("DbgDown"));
        key_map.insert(KeyCode::ShiftRight, KeyBind::new("DbgDown"));

        InputState {
            keybind_states: keybind_map,
            key_states: key_map,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,
        }
    }
}
