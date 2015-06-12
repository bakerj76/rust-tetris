use std::io;

use glium;
use glium::{DisplayBuild, Surface};
use glium::index::PrimitiveType;

use rootwindow::Vertex;

pub struct Position
{
    pub x: f32,
    pub y: f32,
}

pub struct Sprite
{
    sprite_id: u8,

    position: Position,
    tint: [f32; 4],

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Sprite
{
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, sprite: u8, position: Position) -> io::Result<Sprite>
    {
        let vertex_buffer = glium::VertexBuffer::new(display,
            vec![
                Vertex {
                    position: [position.x - 0.1, position.y - 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 0.0],
                },

                Vertex {
                    position: [position.x + 0.1, position.y - 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 0.0],
                },

                Vertex {
                    position: [position.x + 0.1, position.y + 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 0.0],
                },

                Vertex {
                    position: [position.x - 0.1, position.y + 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 0.0],
                },
            ]
        );

        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
            vec![0, 2, 1, 0, 3, 2]
        );

        let sprite = Sprite
        {
            sprite_id: sprite,

            position: position,
            tint: [1.0, 1.0, 1.0, 1.0],

            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        };

        Ok(sprite)
    }

    pub fn draw(self, target: &glium::Frame, program: &glium::Program)
    {
        target.draw(&self.vertex_buffer,
                    &self.index_buffer,
                    program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default()
        );
    }
}
