use glium::glutin;

mod empty_scene;
mod key_listener;
mod level_editor_scene;
mod level_scene;
mod mouse_listener;
mod scene;
mod util;
mod window;

use crate::window::Window;
use glutin::event_loop::EventLoop;

fn main() {
    let event_loop: EventLoop<()> = EventLoop::new();
    let window: Window = Window::new(&event_loop);
    window.run(event_loop);
}
