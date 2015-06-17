extern crate clock_ticks;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod cellmatrix;
mod rootwindow;
mod rect;
mod sprite;
mod spritemanager;
mod tetris;
mod tetromino;

use rootwindow::RootWindow;
use tetris::Tetris;

fn main()
{
    let mut rootwindow = RootWindow::new()
        .unwrap();

    let mut tetris = Tetris::new()
        .unwrap();

    tetris.start(&mut rootwindow);
}
