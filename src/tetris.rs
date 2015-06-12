use std::io;

use cgmath::Vector2;

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
        self.sprites.push(Sprite::new(&display.display, 0, Vector2::new(0.0 as f32, 0.0))
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

        piece.set_position(Vector2::new(x, y));
    }
}
