use std::io;

use cgmath;
use cgmath::{Matrix4, Vector2, Vector3};

use glium;
use glium::Surface;
use glium::index;

use rect::Rect;
use rootwindow::Vertex;
use spritemanager::{SpriteManager, Textures};

pub struct Sprite
{
    pub texture: Textures,

    pub position: Vector3<f32>,
    pub rotation: f32,
    pub tint: [f32; 4],

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Sprite
{
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade,
        texture: Textures, rect: Rect, position: Vector2<f32>)
        -> io::Result<Sprite>
    {
        Sprite::new_tinted(display, texture, rect, position, [1.0, 1.0, 1.0, 1.0])
    }

    pub fn new_tinted(display: &glium::backend::glutin_backend::GlutinFacade,
        texture: Textures, rect: Rect, position: Vector2<f32>, tint: [f32; 4])
        -> io::Result<Sprite>
    {
        let vertex_buffer = glium::VertexBuffer::new(display,
            vec![
                Vertex
                {
                    position: [rect.left(), rect.top()],
                    color: tint,
                    tex_coords: [0.0, 1.0],
                },

                Vertex
                {
                    position: [rect.right(), rect.top()],
                    color: tint,
                    tex_coords: [1.0, 1.0],
                },

                Vertex
                {
                    position: [rect.right(), rect.bottom()],
                    color: tint,
                    tex_coords: [1.0, 0.0],
                },

                Vertex {
                    position: [rect.left(), rect.bottom()],
                    color: tint,
                    tex_coords: [0.0, 0.0],
                },
            ]
        );

        let index_buffer = glium::IndexBuffer::new(display,
            index::PrimitiveType::TrianglesList,
            vec![0, 2, 1, 0, 3, 2]
        );

        let mut sprite = Sprite
        {
            texture: texture,

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
        sprite_manager: &SpriteManager, projection: &Matrix4<f32>)
    {
        let rotmatrix = Matrix4::<f32>::identity();

        let translation = Matrix4::from_translation(&self.position);

        let model =  translation * rotmatrix;
        
        let tex_id = self.texture;
        let ref texture = sprite_manager.get_texture(tex_id);

        let draw_params = glium::DrawParameters
        {
            blending_function:
                Some(glium::BlendingFunction::Addition
                {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
                }),

                .. Default::default()
        };

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
                    &draw_params
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
