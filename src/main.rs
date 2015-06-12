extern crate clock_ticks;
#[macro_use]
extern crate glium;

mod rootwindow;
mod sprite;

use rootwindow::RootWindow;
use sprite::{Sprite, Position};

fn main()
{
    let mut rootwindow = RootWindow::new()
        .unwrap();

    let sprite = Sprite::new(&rootwindow.display, 0, Position{ x: 0.0, y: 0.0 }).unwrap();

    rootwindow.start();
    rootwindow.add_sprite(sprite);
}
