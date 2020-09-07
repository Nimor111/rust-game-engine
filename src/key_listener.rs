use crate::window::Window;
use glium::glutin::event::{ElementState, VirtualKeyCode};
use maplit::hashmap;
use std::collections::HashMap;

pub struct KeyListener {
    keys_state: HashMap<VirtualKeyCode, ElementState>,
}

impl KeyListener {
    pub fn new() -> Self {
        KeyListener {
            keys_state: hashmap![],
        }
    }

    pub fn key_callback(
        &mut self,
        window: &Window,
        virtual_key_code: Option<VirtualKeyCode>,
        state: ElementState,
    ) {
	println!("Key is {:?}, State is {:?} ", virtual_key_code, state);
        match virtual_key_code {
            None => {}
            Some(code) => *self.keys_state.entry(code).or_insert(state) = state,
        }
    }

    pub fn key_pressed(&self, virtual_key_code: Option<VirtualKeyCode>) -> bool {
        match virtual_key_code {
            None => false,
            Some(code) => match self.keys_state.get(&code) {
                None => false,
                Some(state) => state == &ElementState::Pressed,
            },
        }
    }
}
