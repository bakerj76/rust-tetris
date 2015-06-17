use std::io;

use cgmath::Vector2;

use glium;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use image;

use sprite::Sprite;
use spritemanager::Textures;
use tetromino::{Tetromino, Shape};
use rootwindow::{RootWindow, LoopState};
use rect::Rect;

pub struct Tetris
{
    key_held: Option<VirtualKeyCode>,

    background: Option<Sprite>,
    tetrominos: Vec<Tetromino>,
}

impl Tetris
{
    pub fn new() -> io::Result<Tetris>
    {
        Ok(Tetris
        {
            key_held: None,

            background: None,
            tetrominos: vec![],
        })
    }

    pub fn start(&mut self, display: &mut RootWindow)
    {
        let mut tetromino = Tetromino::new(display, Shape::LBlock, Vector2::new(400.0, 300.0));

        self.tetrominos.push(tetromino);
        self.setup_background(display);

        display.start(self);
    }

    pub fn update(&mut self, display: &RootWindow)
    {

    }

    pub fn get_sprites(&mut self) -> Vec<&Sprite>
    {
        let bg = match self.background
        {
            Some(ref x) => x,
            None => panic!("Couldn't find background?!?")
        };

        let mut sprites = vec![bg];

        for piece in self.tetrominos.iter()
        {
            for sprite in piece.sprites.iter()
            {
                sprites.push(sprite);
            }
        }

        sprites
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
                self.handle_key(key, |tetris| { tetris.move_piece(Vector2::new(-1, 0)) } ),

            (VirtualKeyCode::Right, ElementState::Pressed) =>
                self.handle_key(key, |tetris| { tetris.move_piece(Vector2::new(1, 0)) } ),

            (VirtualKeyCode::Up, ElementState::Pressed) =>
                self.handle_key(key, |tetris| { tetris.rotate_piece() }),

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

    fn handle_key<F>(&mut self, key: VirtualKeyCode, mut action: F) where F: FnMut(&mut Tetris)
    {
        if self.key_held.is_none() || self.key_held.unwrap() != key
        {
            self.key_held = Some(key);
            action(self);
        }
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

    /// Sets up background image
    fn setup_background(&mut self, display: &mut RootWindow)
    {
        //TODO: un-hard-code the width and height
        self.background = Some(
            Sprite::new(
                &display.display,
                Textures::Background,
                Rect::new(0.0, 0.0, 400.0, 376.0),
                Vector2::new(0.0, 0.0)
            ).unwrap()
        );
    }
}
