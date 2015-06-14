use std::io;

use cgmath::Vector2;

use glium;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use image;

use sprite::Sprite;
use tetromino::{Tetromino, Shape};
use rootwindow::{RootWindow, LoopState};

pub struct Tetris
{
    key_held: Option<VirtualKeyCode>,

    pub tetrominos: Vec<Tetromino>,
}

impl Tetris
{
    pub fn new() -> io::Result<Tetris>
    {
        Ok(Tetris
        {
            key_held: None,

            tetrominos: vec![],
        })
    }

    pub fn start(&mut self, display: &mut RootWindow)
    {
        let mut tetromino = Tetromino::new(display, Shape::LBlock, Vector2::new(400.0, 300.0));

        self.tetrominos.push(tetromino);

        display.start(self);
    }

    pub fn update(&mut self, display: &RootWindow)
    {
        /*let piece = &mut self.sprites[0];

        let (x, y) = {
            (piece.position.x + 32.0 * display.delta_time as f32,
            piece.position.y)
        };

        let rotation = {
            piece.rotation + 60.0 * display.delta_time as f32
        };

        piece.set_position(Vector2::new(x, y));
        piece.set_rotation(rotation);*/

    }

    pub fn handle_input(&mut self, display: &RootWindow, event: Event) -> LoopState
    {
        match event
        {
            Event::KeyboardInput(state, code, keycode) =>
                self.handle_keyboard(display, state, keycode),

            _ => return LoopState::Play
        }
    }

    ///TODO: Split this up into methods to reduce repeated code
    fn handle_keyboard(&mut self, display: &RootWindow, state: ElementState,
        keycode: Option<VirtualKeyCode>) -> LoopState
    {

        let key = match keycode
        {
            Some(x) => x,
            None => return LoopState::Play
        };

        match (key, state)
        {
            (VirtualKeyCode::Left, ElementState::Pressed) =>
            {
                if self.key_held.is_none() || self.key_held.unwrap() != key
                {
                    self.key_held = Some(key);
                    self.move_piece(Vector2::new(-1, 0))
                }
            },

            (VirtualKeyCode::Right, ElementState::Pressed) =>
            {
                if self.key_held.is_none() || self.key_held.unwrap() != key
                {
                    self.key_held = Some(key);
                    self.move_piece(Vector2::new(1, 0));
                }
            },

            (VirtualKeyCode::Up, ElementState::Pressed) =>
            {
                if self.key_held.is_none() || self.key_held.unwrap() != key
                {
                    self.key_held = Some(key);
                    self.rotate_piece();
                }
            }


            (_, ElementState::Released) =>
            {
                match self.key_held
                {
                    Some(x) =>
                    {
                        if x == key
                        {
                            self.key_held = None;
                        }
                    },

                    None => ()
                }
            },

            _ => ()
        }

        LoopState::Play
    }

    fn move_piece(&mut self, direction: Vector2<i8>)
    {
        let piece = &mut self.tetrominos[0];

        let (x, y) = {
            (piece.position.x + direction.x as f32 * 16.0,
             piece.position.y + direction.y as f32 * 16.0)
        };

        piece.set_position(Vector2::new(x, y));
    }

    fn rotate_piece(&mut self)
    {
        let piece = &mut self.tetrominos[0];

        piece.rotate_right();
    }
}
