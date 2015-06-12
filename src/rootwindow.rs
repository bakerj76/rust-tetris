use glium;
use glium::{DisplayBuild, Surface};
use clock_ticks;
use std::thread;
use std::io;

use sprite::Sprite;

/// The window
pub struct RootWindow
{
    pub display: glium::backend::glutin_backend::GlutinFacade,

    program: glium::Program,
    sprites: Vec<Sprite>,

    pub max_frame_rate: u32,
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
            sprites: vec![],

            max_frame_rate: 60,
        })
    }

    pub fn start(&mut self)
    {
        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();

        loop
        {
            if self.draw()
            {
                break;
            }

            let now = clock_ticks::precise_time_ns();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp: u64 = (1.0 / self.max_frame_rate as f64 * 1E+9) as u64;
            while accumulator >= fixed_time_stamp
            {
                accumulator -= fixed_time_stamp;
            }

            thread::sleep_ms(((fixed_time_stamp - accumulator) as f64 / 1E+6) as u32);
        }

    }

    pub fn draw(&mut self) -> bool
    {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for ref sprite in self.sprites
        {
            sprite.draw(&target, &self.program);
        }

        target.finish();

        return self.display.is_closed();
    }

    pub fn add_sprite(&mut self, sprite: Sprite)
    {
        self.sprites.push(sprite);
    }
}
