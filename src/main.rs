extern crate clock_ticks;
#[macro_use]
extern crate glium;
extern crate cgmath;

mod rootwindow;
mod sprite;
mod tetris;

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
