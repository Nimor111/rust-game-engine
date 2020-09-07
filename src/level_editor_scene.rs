use crate::{key_listener::KeyListener, scene::Scene, util::GLObject, window::Window};
use glium::Surface;

#[derive(Clone)]
pub struct LevelEditorScene {}

impl LevelEditorScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for LevelEditorScene {
    fn update(
        self: Box<Self>,
        _dt: f32,
        _key_listener: &KeyListener,
        window: &mut Window,
        ctx: &GLObject,
    ) {
        let mut target = window.window.draw();
        target.clear_color(window.r, window.g, window.b, window.a);
        target
            .draw(
                &ctx.vertex_buffer,
                &ctx.index_buffer,
                &ctx.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    fn init(self: Box<Self>, _window: &Window) {}
}
