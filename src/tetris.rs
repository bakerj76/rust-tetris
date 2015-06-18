use std::io;
use std::f32;

use cgmath::Vector2;

use glium::glutin::{Event, ElementState, VirtualKeyCode};

use cellmatrix::CellMatrix;
//use frametimer::FrameTimer;
use sprite::Sprite;
use spritemanager::Textures;
use tetromino::{Tetromino, Shape};
use rootwindow::{RootWindow, LoopState};
use rect::Rect;

const BOARD_POS: Vector2<f32> = Vector2 { x: 36.5, y: 8.5 };

pub struct Tetris
{
    width: u32,
    height: u32,

    key_held: Option<VirtualKeyCode>,

    board: CellMatrix,

    gravity: f32,
    gravity_frame: u32,

    background: Option<Sprite>,
    tetrominos: Vec<Tetromino>,
}

impl Tetris
{
    pub fn new(width: u32, height: u32) -> io::Result<Tetris>
    {
        Ok(Tetris
        {
            width: width,
            height: height,

            key_held: None,

            board: CellMatrix::new(10, 22),

            gravity: 1.0/60.0,
            gravity_frame: 0,

            background: None,
            tetrominos: vec![],
        })
    }

    pub fn start(&mut self, display: &mut RootWindow)
    {
        let tetromino = Tetromino::new(display, Shape::LBlock, BOARD_POS, Vector2::new(3, 0));

        self.tetrominos.push(tetromino);
        self.setup_background(display);

        display.start(self);
    }

    pub fn update(&mut self)
    {
        self.gravity_frame += 1;

        if self.gravity_frame > (1.0/self.gravity) as u32
        {
            self.gravity();
            self.gravity_frame = 0;
        }

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

    pub fn handle_input(&mut self, event: Event) -> LoopState
    {
        match event
        {
            Event::KeyboardInput(state, _, keycode) =>
                self.handle_keyboard(state, keycode),

            _ => return LoopState::Play
        }
    }

    fn handle_keyboard(&mut self, state: ElementState, keycode: Option<VirtualKeyCode>)
        -> LoopState
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
        let position = piece.cell_position + direction;
        piece.set_position(position);
    }

    fn rotate_piece(&mut self)
    {
        let piece = &mut self.tetrominos[0];
        piece.rotate_right();
    }

    fn gravity(&mut self)
    {
        let velocity = self.gravity.ceil() as i8;
        self.move_piece(Vector2::new(0, velocity));
    }

    /// Sets up background image
    fn setup_background(&mut self, display: &mut RootWindow)
    {
        self.background = Some(
            Sprite::new(
                &display.display,
                Textures::Background,
                Rect::new(0.0, 0.0, self.width as f32, self.height as f32),
                Vector2::new(0.0, 0.0)
            ).unwrap()
        );
    }
}
