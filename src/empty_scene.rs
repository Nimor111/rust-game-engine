use crate::{key_listener::KeyListener, scene::Scene, util::GLObject, window::Window};

#[derive(Clone)]
pub struct EmptyScene {}

impl Scene for EmptyScene {
    fn init(self: Box<Self>, _window: &Window) {}
    fn update(
        self: Box<Self>,
        _dt: f32,
        _key_listener: &KeyListener,
        _window: &mut Window,
        _ctx: &GLObject,
    ) {
    }
}
