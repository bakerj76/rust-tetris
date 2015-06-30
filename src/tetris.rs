use cgmath::{Vector2, Matrix4};

use glium;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use cellmatrix::CellMatrix;
use rect::Rect;
use rootwindow::GameState;
use sprite::Sprite;
use spritemanager::{SpriteManager, Textures};
use tetromino::{Tetromino, Shape};


const BOARD_POS: Vector2<f32> = Vector2 { x: 36.5, y: 8.5 };

pub struct Tetris
{
    width: u32,
    height: u32,

    key_held: Option<VirtualKeyCode>,

    board: CellMatrix,

    old_gravity: f32,
    gravity: f32,
    gravity_frame: u32,

    background: Option<Sprite>,
    tetrominos: Vec<Tetromino>,
    current_tetromino: Option<Tetromino>,
}

impl Tetris
{
    pub fn new(width: u32, height: u32) -> Tetris
    {
        Tetris
        {
            width: width,
            height: height,

            key_held: None,

            board: CellMatrix::new(10, 22),

            old_gravity: 0.0,
            gravity: 1.0/60.0,
            gravity_frame: 0,

            background: None,
            tetrominos: vec![],
            current_tetromino: None,
        }
    }

    pub fn start(&mut self, display: &glium::backend::glutin_backend::GlutinFacade)
    {
        self.new_piece(display);
        self.setup_background(display);
    }

    pub fn update(&mut self, event: Event) -> GameState
    {
        let gamestate = self.handle_input(event);

        self.gravity_frame += 1;

        if self.gravity_frame > (1.0/self.gravity) as u32
        {
            self.gravity();
            self.gravity_frame = 0;
        }

        gamestate
    }

    pub fn handle_input(&mut self, event: Event) -> GameState
    {
        match event
        {
            Event::KeyboardInput(state, _, keycode) =>
                self.handle_keyboard(state, keycode),

            _ => return GameState::Play
        }
    }

    fn handle_keyboard(&mut self, state: ElementState, keycode: Option<VirtualKeyCode>)
        -> GameState
    {

        let key = match keycode
        {
            Some(x) => x,
            None => return GameState::Play
        };

        match (key, state)
        {
            (VirtualKeyCode::Left, ElementState::Pressed) =>
                self.handle_key(key, |tetris| { tetris.move_piece(Vector2::new(-1, 0)) } ),

            (VirtualKeyCode::Right, ElementState::Pressed) =>
                self.handle_key(key, |tetris| { tetris.move_piece(Vector2::new(1, 0)) } ),

            (VirtualKeyCode::Up, ElementState::Pressed) | (VirtualKeyCode::Z, ElementState::Pressed)  =>
                self.handle_key(key, |tetris| { tetris.rotate_right() }),
                
            (VirtualKeyCode::X, ElementState::Pressed) =>
                self.handle_key(key, |tetris| { tetris.rotate_left() }),

            (VirtualKeyCode::Down, ElementState::Pressed) =>
            {
                self.handle_key(key, |tetris| { tetris.old_gravity = tetris.gravity });

                self.gravity += 1.0;
            },

            (VirtualKeyCode::Down, ElementState::Released) =>
                self.gravity = self.old_gravity,

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

        GameState::Play
    }

    fn handle_key<F>(&mut self, key: VirtualKeyCode, mut action: F)
        where F: FnMut(&mut Tetris)
    {
        if self.key_held.is_none() || self.key_held.unwrap() != key
        {
            self.key_held = Some(key);
            action(self);
        }
    }
    
    pub fn draw_sprites(&mut self, target: &mut glium::Frame, program: &glium::Program,
        sprite_manager: &SpriteManager, projection: &Matrix4<f32>)
    {
        {
            let ref bg = match self.background
            {
                Some(ref x) => x,
                None => panic!("No background sprite found!")
            };
            
            bg.draw(target, program, sprite_manager, projection);
        }
        
        for tetromino in self.tetrominos.iter()
        {
            for sprite in tetromino.sprites.iter()
            {
                sprite.draw(target, program, sprite_manager, projection);
            }
        }
    }

    fn move_piece(&mut self, direction: Vector2<i8>)
    {
        let piece = match self.current_tetromino
        {
            Some(ref mut x) => x,
            None => return
        };

        let next_pos = piece.cell_position + direction;

        if !piece.collides(&self.board, next_pos)
        {
            piece.set_position(next_pos);
        }
    }

    fn rotate_right(&mut self)
    {
        match self.current_tetromino
        {
            Some(ref mut x) => x,
            None => return
        }.rotate_right();
    }
    
    fn rotate_left(&mut self)
    {
        match self.current_tetromino
        {
            Some(ref mut x) => x,
            None => return
        }.rotate_left();
    }

    fn gravity(&mut self)
    {
        let velocity = Vector2::new(0, self.gravity.ceil() as i8);

        {
            let piece = match self.current_tetromino
            {
                Some(ref x) => x,
                None => return
            };

            let next_pos = piece.cell_position + velocity;

            if piece.collides(&self.board, next_pos)
            {
                return;
            }
        }

        self.move_piece(velocity);
    }

    fn new_piece(&mut self, display: &glium::backend::glutin_backend::GlutinFacade)
    {
        /*if self.current_tetromino.is_some()
        {
            let ct = self.current_tetromino;
            self.tetrominos.push(ct.expect("this is impossible"));
        }*/

        let ct = Tetromino::new(display, Shape::LBlock, BOARD_POS, Vector2::new(3, 0));
        self.current_tetromino = Some(ct);
    }

    /// Sets up background image
    fn setup_background(&mut self, display: &glium::backend::glutin_backend::GlutinFacade)
    {
        self.background = Some(
            Sprite::new(
                display,
                Textures::Background,
                Rect::new(0.0, 0.0, self.width as f32, self.height as f32),
                Vector2::new(0.0, 0.0)
            ).unwrap()
        );
    }
}
