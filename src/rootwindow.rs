use std::thread;
use std::io;

use cgmath;

use clock_ticks;

use glium;
use glium::{DisplayBuild, Surface};
use glium::glutin;

use spritemanager::SpriteManager;
use tetris::Tetris;

/// The window
pub struct RootWindow
{
    tetris: Tetris,

    pub display: glium::backend::glutin_backend::GlutinFacade,
    sprite_manager: Option<SpriteManager>,

    program: glium::Program,
    ortho_matrix: cgmath::Matrix4<f32>,

    max_frame_rate: u32,
    delta_time: f64,
}

#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

pub enum GameState
{
    Exit,
    Play,
}

impl RootWindow
{
    /// Creates a new root window
    pub fn new(width: u32, height: u32) -> io::Result<RootWindow>
    {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(width, height)
            .with_title(format!("Rustris"))
            .build_glium()
            .unwrap();

        let program = program!(&display,
            140 =>
            {
                vertex: include_str!("shaders/140.vert"),
                fragment: include_str!("shaders/140.frag"),
            },
        ).unwrap();

        implement_vertex!(Vertex, position, color, tex_coords);

        Ok(RootWindow
        {
            tetris: Tetris::new(width, height),

            display: display,
            sprite_manager: None,

            program: program,
            ortho_matrix: cgmath::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0),

            max_frame_rate: 60,
            delta_time: 0.0,
        })
    }

    /// Starts the draw loop
    pub fn start(&mut self)
    {
        self.sprite_manager = Some(SpriteManager::new(self));
        
        self.tetris.start(&self.display);

        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();

        loop
        {
            // Get input
            match self.do_input()
            {
                GameState::Exit => return,
                GameState::Play => ()
            }

        
            let now = clock_ticks::precise_time_ns();

            // Add the time between the last loop
            accumulator += now - previous_clock;

            // The time for each frame in nanoseconds
            let fixed_time_stamp = (1.0 / self.max_frame_rate as f64 * 1E+9) as u64;

            // Loop the amount of update ticks that are saved up
            while accumulator >= fixed_time_stamp
            {
                // The time since the last frame
                // TODO: Make this the actual time
                self.delta_time = fixed_time_stamp as f64 / 1E+9;//(now - delta) as f64 / 1E+9;

                accumulator -= fixed_time_stamp;


                // Update the game logic
                self.tetris.update();
            }
            
            // Finally, draw the sprites
            self.draw();

            previous_clock = now;

            thread::sleep_ms(((fixed_time_stamp - accumulator) as f64 / 1E+6) as u32);
        }
    }

    /// Handles sprite drawing
    /// TODO: sprite batching
    fn draw(&mut self)
    {
        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let sprite_manager = match self.sprite_manager
        {
            Some(ref x) => x,
            None => panic!("Missing sprite manager!")
        };

        self.tetris.draw_sprites(&mut target, &self.program, sprite_manager, &self.ortho_matrix);

        target.finish();
    }

    /// Handles events
    fn do_input(&mut self) -> GameState
    {
        let mut state = GameState::Play;

        for event in self.display.poll_events()
        {
            state = match event
            {
                glutin::Event::Closed => GameState::Exit,
                _ => self.tetris.handle_input(event),
            };

            match state
            {
                GameState::Exit => return state,
                _ => ()
            }
        }

        state
    }
}
