extern crate clock_ticks;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod cellmatrix;
//mod frametimer;
mod rootwindow;
mod rect;
mod sprite;
mod spritemanager;
mod tetris;
mod tetromino;

use rootwindow::RootWindow;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 376;

fn main()
{
    let mut rootwindow = RootWindow::new(WIDTH, HEIGHT)
        .unwrap();
    rootwindow.start();
}
