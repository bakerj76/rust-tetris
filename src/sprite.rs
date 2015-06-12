use std::io;

use glium;
use glium::Surface;
use glium::index;

use cgmath;
use cgmath::{Matrix4, Vector2, Vector3};

use rootwindow::Vertex;

pub struct Sprite
{
    pub sprite_id: u8,

    pub position: Vector3<f32>,
    pub tint: [f32; 4],

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Sprite
{
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, sprite: u8, position: Vector2<f32>) -> io::Result<Sprite>
    {
        let vertex_buffer = glium::VertexBuffer::new(display,
            vec![
                Vertex
                {
                    position: [position.x - 0.1, position.y - 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 1.0],
                },

                Vertex
                {
                    position: [position.x + 0.1, position.y - 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [1.0, 1.0],
                },

                Vertex
                {
                    position: [position.x + 0.1, position.y + 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [0.0, 0.0],
                },

                Vertex {
                    position: [position.x - 0.1, position.y + 0.1],
                    color: [1.0, 1.0, 1.0, 1.0],
                    tex_coord: [1.0, 0.0],
                },
            ]
        );

        let index_buffer = glium::IndexBuffer::new(display, index::PrimitiveType::TrianglesList,
            vec![0, 2, 1, 0, 3, 2]
        );

        let mut sprite = Sprite
        {
            sprite_id: sprite,

            position: cgmath::zero(),
            tint: [1.0, 1.0, 1.0, 1.0],

            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        };

        sprite.set_position(position);

        Ok(sprite)
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program)
    {
        let matrix = Matrix4::from_translation(&self.position);

        target.draw(&self.vertex_buffer,
                    &self.index_buffer,
                    program,
                    &uniform! { matrix: matrix },
                    &Default::default()
        ).unwrap();
    }

    pub fn set_position(&mut self, position: Vector2<f32>)
    {
        self.position = Vector3::new(position.x, position.y, 0.0);
    }
}
