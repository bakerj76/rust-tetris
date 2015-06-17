use cgmath;

use clock_ticks;

use glium;
use glium::{DisplayBuild, Surface};
use glium::glutin;

use std::thread;
use std::io;

use sprite::Sprite;
use spritemanager::SpriteManager;
use tetris::Tetris;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 376;

/// The window
pub struct RootWindow
{
    pub display: glium::backend::glutin_backend::GlutinFacade,
    sprite_manager: Option<SpriteManager>,

    program: glium::Program,
    ortho_matrix: cgmath::Matrix4<f32>,

    pub max_frame_rate: u32,
    pub delta_time: f64,
}

#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}


/// The current update/draw state
/// TODO: Rename this
pub enum LoopState
{
    Stop,
    Play,
}

impl RootWindow
{
    /// Creates a new root window
    pub fn new() -> io::Result<RootWindow>
    {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(WIDTH, HEIGHT)
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
            display: display,
            sprite_manager: None,

            program: program,
            ortho_matrix: cgmath::ortho(0.0, WIDTH as f32, HEIGHT as f32, 0.0, -1.0, 1.0),

            max_frame_rate: 60,
            delta_time: 0.0,
        })
    }

    /// Starts the draw loop
    pub fn start(&mut self, tetris: &mut Tetris)
    {
        self.sprite_manager = Some(SpriteManager::new(self));

        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();

        loop
        {
            self.draw(tetris);
            match self.do_input(tetris)
            {
                LoopState::Stop => break,
                _ => ()
            }

            let now = clock_ticks::precise_time_ns();

            // Add the time between the last loop
            accumulator += now - previous_clock;

            // The time for each frame in nanoseconds
            let fixed_time_stamp = (1.0 / self.max_frame_rate as f64 * 1E+9) as u64;

            // While the amount of time in the accumulator is greater than the time to draw a frame
            while accumulator >= fixed_time_stamp
            {
                // The time since the last frame
                // TODO: Make this the actual time
                self.delta_time = fixed_time_stamp as f64 / 1E+9;//(now - delta) as f64 / 1E+9;

                accumulator -= fixed_time_stamp;

                // Update the game logic
                tetris.update(self);
            }

            previous_clock = now;

            thread::sleep_ms(((fixed_time_stamp - accumulator) as f64 / 1E+6) as u32);
        }
    }

    /// Handles sprite drawing
    /// TODO: sprite batching
    fn draw(&mut self, tetris: &mut Tetris)
    {
        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let sprite_manager = match self.sprite_manager
        {
            Some(ref x) => x,
            None => panic!("Missing sprite manager!")
        };

        for ref sprite in tetris.get_sprites().iter()
        {
            let tex_id = sprite.texture;
            let ref texture = sprite_manager.get_texture(tex_id);

            sprite.draw(&mut target, &self.program, texture, &self.ortho_matrix);
        }

        target.finish();
    }

    /// Handles events
    fn do_input(&self, tetris: &mut Tetris) -> LoopState
    {
        let mut state = LoopState::Play;

        for event in self.display.poll_events()
        {
            state = match event
            {
                glutin::Event::Closed => LoopState::Stop,
                _ => tetris.handle_input(self, event),
            };

            match state
            {
                LoopState::Stop => return state,
                _ => ()
            }
        }

        state
    }
}
