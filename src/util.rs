use crate::window::Window;
use glium::{implement_vertex, index::PrimitiveType};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

implement_vertex!(Vertex, position, color);

pub struct GLObject {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

impl GLObject {
    // TODO add errors here, don't unwrap
    pub fn new(
        vertex_array: Vec<Vertex>,
        element_array: Vec<u16>,
        vertex_shader_source: &'static str,
        fragment_shader_source: &'static str,
        window: &Window,
    ) -> Self {
        let program = glium::Program::from_source(
            &window.window,
            vertex_shader_source,
            fragment_shader_source,
            None,
        )
        .unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&window.window, &vertex_array).unwrap();
        let index_buffer =
            glium::IndexBuffer::new(&window.window, PrimitiveType::TrianglesList, &element_array)
                .unwrap();

        Self {
            program,
            vertex_buffer,
            index_buffer,
        }
    }
}
