use std::io;

use glium::texture;

use image;

use rootwindow::RootWindow;

///TODO: Rename this
#[derive(Debug, Copy, Clone)]
pub enum Textures
{
    SpriteSheet = 0,
    Background,
}

pub struct SpriteManager
{
    textures: Vec<texture::Texture2d>,
}

impl SpriteManager
{
    pub fn new(root_window: &RootWindow) -> SpriteManager
    {
        //Load image
        let sprite_sheet = image::load(
            io::Cursor::new(&include_bytes!("../images/spritesheet.png")[..]),
            image::PNG
        ).unwrap();

        let background = image::load(
            io::Cursor::new(&include_bytes!("../images/background.png")[..]),
            image::PNG
        ).unwrap();

        SpriteManager
        {
            textures: vec![
                texture::Texture2d::new(&root_window.display, sprite_sheet),
                texture::Texture2d::new(&root_window.display, background),
            ]
        }
    }

    pub fn get_texture<'a>(&'a self, texture: Textures) -> &'a texture::Texture2d
    {
        &self.textures[texture as usize]
    }
}
