use crate::{
    empty_scene::EmptyScene,
    key_listener::KeyListener,
    level_editor_scene::LevelEditorScene,
    mouse_listener::MouseListener,
    scene::Scene,
    util::{GLObject, Vertex},
};
use glium::glutin;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use std::time::{Duration, Instant};

pub struct Window {
    pub window: glium::Display,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    scene: Box<dyn Scene>,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let size: glutin::dpi::LogicalSize<u32> = (800, 600).into();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Mario")
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, event_loop).unwrap();

        Window {
            window: display,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
            scene: Box::new(EmptyScene {}),
        }
    }

    pub fn change_scene(&mut self, new_scene: Box<dyn Scene>) {
        self.scene = new_scene;
        self.scene.clone().init(self);
    }

    pub fn run(mut self: Self, event_loop: EventLoop<()>) {
        let mut mouse_listener = MouseListener::new();
        let mut key_listener = KeyListener::new();

        let mut dt = 0.0;

        // mock data
        let vertex_shader_source = r#"
		#version 330 core

		layout (location=0) in vec3 position;
		layout (location=1) in vec4 color;

		out vec4 fColor;

		void main() {
		fColor = color;

		gl_Position = vec4(position, 1.0);
		}
            "#;
        let fragment_shader_source = r#"
		#version 330 core

		in vec4 fColor;

		out vec4 color;

		void main() {
		color = fColor;
		}
            "#;
        let vertex_array = vec![
            Vertex::new([0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]), // bottom right
            Vertex::new([-0.5, 0.5, 0.0], [0.0, 1.0, 0.0, 1.0]), // top left
            Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0, 1.0, 1.0]),  // top right
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 1.0, 0.0, 1.0]), // bottom left
        ];
        // Counter clockwise order
        let element_array = vec![
            2, 1, 0, // top right triangle
            0, 1, 3, // bottom left triangle
        ];

        self.change_scene(Box::new(LevelEditorScene::new()));

        let mut last_update = Instant::now();
        let object: GLObject = GLObject::new(
            vertex_array,
            element_array,
            vertex_shader_source,
            fragment_shader_source,
            &self,
        );

        event_loop.run(move |ev, _, control_flow| {
            let sixteen_ms = Duration::from_millis(16);
            let duration_since_last_update = Instant::now().duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            if dt > 0.0 {
                self.scene
                    .clone()
                    .update(dt, &key_listener, &mut self, &object);
            }

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_listener.cursor_moved_callback(&self, position);
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        mouse_listener.mouse_button_callback(&self, button, state);
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        mouse_listener.mouse_scroll_callback(&self, delta);
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        key_listener.key_callback(&self, input.virtual_keycode, input.state);
                    }
                    _ => return,
                },
                _ => return,
            };

            let elapsed = last_update.elapsed();
            dt = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 * 1e-9);
            last_update = Instant::now();
        })
    }
}
