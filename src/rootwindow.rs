use glium;
use glium::{DisplayBuild, Surface};
use glium::glutin;

use clock_ticks;

use std::thread;
use std::io;

use sprite::Sprite;
use tetris::Tetris;

/// The window
pub struct RootWindow
{
    pub display: glium::backend::glutin_backend::GlutinFacade,

    program: glium::Program,

    pub max_frame_rate: u32,
    pub delta_time: f64,
}

#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2],
}

impl RootWindow
{
    /// Creates a new root window
    pub fn new() -> io::Result<RootWindow>
    {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(800, 600)
            .build_glium()
            .unwrap();

        let program = program!(&display,
            140 =>
            {
                    vertex: include_str!("shaders/140.vert"),
                    fragment: include_str!("shaders/140.frag"),
            },
        ).unwrap();

        implement_vertex!(Vertex, position, color, tex_coord);

        Ok(RootWindow
        {
            display: display,

            program: program,

            max_frame_rate: 60,
            delta_time: 0.0,
        })
    }

    /// Starts the draw loop
    pub fn start(&mut self, tetris: &mut Tetris)
    {
        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();

        loop
        {
            self.draw(&tetris.sprites);
            if self.do_input()
            {
                break;
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
    fn draw(&mut self, sprites: &Vec<Sprite>)
    {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for ref sprite in sprites.iter()
        {
            sprite.draw(&mut target, &self.program);
        }

        target.finish();
    }

    /// Handles events
    fn do_input(&self) -> bool
    {
        for event in self.display.poll_events()
        {
            match event
            {
                glutin::Event::Closed => return true,
                _ => ()
            }
        }

        false
    }
}
