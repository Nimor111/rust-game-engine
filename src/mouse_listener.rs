use crate::window::Window;
use glium::glutin;
use glutin::dpi::PhysicalPosition;
use glutin::event::{ElementState, MouseButton, MouseScrollDelta};
use maplit::hashmap;
use std::collections::HashMap;

pub struct MouseListener {
    scroll_x: f64,
    scroll_y: f64,
    x_pos: f64,
    y_pos: f64,
    last_x: f64,
    last_y: f64,
    is_dragging: bool,
    mouse_buttons_state: HashMap<MouseButton, ElementState>,
}

impl MouseListener {
    pub fn new() -> Self {
        MouseListener {
            scroll_x: 0.0,
            scroll_y: 0.0,
            x_pos: 0.0,
            y_pos: 0.0,
            last_x: 0.0,
            last_y: 0.0,
            is_dragging: false,
            mouse_buttons_state: hashmap![],
        }
    }

    pub fn cursor_moved_callback(&mut self, window: &Window, pos: PhysicalPosition<f64>) {
        self.last_x = self.x_pos;
        self.last_y = self.y_pos;
        self.x_pos = pos.x;
        self.y_pos = pos.y;

        // if the mouse has moved and a button is pressed, that means the user is dragging something
        self.is_dragging = self
            .mouse_buttons_state
            .values()
            .any(|&val| val == ElementState::Pressed);
    }

    pub fn mouse_button_callback(
        &mut self,
        window: &Window,
        button: MouseButton,
        state: ElementState,
    ) {
        if state == ElementState::Released {
            self.is_dragging = false;
        }

        *self.mouse_buttons_state.entry(button).or_insert(state) = state;
    }

    pub fn mouse_scroll_callback(&mut self, window: &Window, offset: MouseScrollDelta) {
        match offset {
            MouseScrollDelta::LineDelta(x, y) => {
                self.scroll_x = x as f64;
                self.scroll_y = y as f64;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_x = pos.x;
                self.scroll_y = pos.y;
            }
        }
    }

    // TODO: Can this be done with Drop trait?
    pub fn end_frame(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.last_x = self.x_pos;
        self.last_y = self.y_pos;
    }

    /// The number of elapsed x position in the current frame
    pub fn dx(&self) -> f64 {
        self.last_x - self.x_pos
    }

    /// The number of elapsed y position in the current frame
    pub fn dy(&self) -> f64 {
        self.last_y - self.y_pos
    }
}
