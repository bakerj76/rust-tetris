use std::io;

use cgmath::Vector2;
use image;
use glium;

use sprite::Sprite;
use rootwindow::RootWindow;

pub struct Tetris
{
    pub sprites: Vec<Sprite>,
}

impl Tetris
{
    pub fn new() -> io::Result<Tetris>
    {
        Ok(Tetris
        {
            sprites: vec![],
        })
    }

    pub fn start(&mut self, display: &mut RootWindow)
    {
        //Load image
        let image = image::load(io::Cursor::new(&include_bytes!("../spritesheet.png")[..]),
            image::PNG).unwrap();

        let texture = glium::texture::Texture2d::new(&display.display, image);

        self.sprites.push(Sprite::new(&display.display, texture, 0,
            [0.0, 0.7, 0.3, 1.0], Vector2::new(200.0 as f32, 150.0))
            .unwrap());

        display.start(self);
    }

    pub fn update(&mut self, display: &RootWindow)
    {
        let piece = &mut self.sprites[0];

        let (x, y) = {
            (piece.position.x + 0.1 * display.delta_time as f32,
            piece.position.y)
        };

        //piece.set_position(Vector2::new(x, y));
    }
}
