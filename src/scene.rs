use crate::{key_listener::KeyListener, util::{GLObject, Vertex}, window::Window};
use std::error::Error;

pub trait CloneScene {
    fn clone_scene(&self) -> Box<dyn Scene>;
}

impl<T> CloneScene for T
where
    T: Scene + Clone + 'static,
{
    fn clone_scene(&self) -> Box<dyn Scene> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Scene> {
    fn clone(&self) -> Self {
        self.clone_scene()
    }
}

pub trait Scene: CloneScene {
    fn init(self: Box<Self>, window: &Window);
    fn update(self: Box<Self>, dt: f32, key_listener: &KeyListener, window: &mut Window, ctx: &GLObject);
}
