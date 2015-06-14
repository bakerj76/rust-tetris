use std;
use std::io;

use glium;
use glium::Surface;
use glium::{index, texture};

use cgmath;
use cgmath::{Matrix4, Vector2, Vector3};

use rootwindow::Vertex;

pub struct Sprite
{
    pub sprite_id: u8,

    pub position: Vector3<f32>,
    pub rotation: f32,
    pub tint: [f32; 4],

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Sprite
{
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, sprite: u8,
        tint: [f32; 4], position: Vector2<f32>) -> io::Result<Sprite>
    {
        let vertex_buffer = glium::VertexBuffer::new(display,
            vec![
                Vertex
                {
                    position: [position.x - 8.0, position.y - 8.0],
                    color: tint,
                    tex_coords: [0.0, 1.0],
                },

                Vertex
                {
                    position: [position.x + 8.0, position.y - 8.0],
                    color: tint,
                    tex_coords: [0.5, 1.0],
                },

                Vertex
                {
                    position: [position.x + 8.0, position.y + 8.0],
                    color: tint,
                    tex_coords: [0.5, 0.5],
                },

                Vertex {
                    position: [position.x - 8.0, position.y + 8.0],
                    color: tint,
                    tex_coords: [0.0, 0.5],
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
            rotation: 0.0,
            tint: tint,

            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        };

        sprite.set_position(position);
        sprite.set_rotation(0.0);

        Ok(sprite)
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program,
        texture: &texture::Texture2d, projection: &Matrix4<f32>)
    {
        /*
        let rotation = self.rotation * consts::PI / 180.0;

        let cosrot = rotation.cos();
        let sinrot = rotation.sin();

        let rotmatrix: Matrix4<f32> = Matrix4::new(cosrot, -sinrot, 0.0, 0.0,
                                                  sinrot,  cosrot, 0.0, 0.0,
                                                  0.0,     0.0,    1.0, 0.0,
                                                  0.0,     0.0,    0.0, 1.0);

        println!("[[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]]",
            rotation.x.x, rotation.y.x, rotation.z.x, rotation.w.x,
            rotation.x.y, rotation.y.y, rotation.z.y, rotation.w.y,
            rotation.x.z, rotation.y.z, rotation.z.z, rotation.w.z,
            rotation.x.w, rotation.y.w, rotation.z.w, rotation.w.w);
        */

        let rotmatrix = Matrix4::<f32>::identity();

        let translation = Matrix4::from_translation(&self.position);

        let model =  translation * rotmatrix;

        target.draw(&self.vertex_buffer,
                    &self.index_buffer,
                    program,
                    &uniform!
                    {
                        matrix: *projection * model,
                        tex: texture
                            .sampled()
                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    },
                    &Default::default()
        ).unwrap();
    }

    pub fn set_position(&mut self, position: Vector2<f32>)
    {
        self.position = Vector3::new(position.x, position.y, 0.0);
    }

    pub fn set_rotation(&mut self, rotation: f32)
    {
        self.rotation = rotation;
    }
}
