use cellmatrix::{Cell, CellMatrix};
use sprite::Sprite;
use rootwindow::RootWindow;

use cgmath::Vector2;

use glium::texture;

pub enum Shape
{
    IBlock,
    OBlock,
    TBlock,
    SBlock,
    ZBlock,
    JBlock,
    LBlock,
}

pub struct Tetromino
{
    pub shape: Shape,

    pub position: Vector2<f32>,

    matrix: CellMatrix,
    pub sprites: Vec<Sprite>,
}

impl Tetromino
{
    pub fn new(display: &mut RootWindow, shape: Shape,
        position: Vector2<f32>) -> Tetromino
    {
        let matrix = Tetromino::build_matrix(&shape);
        let sprites = Tetromino::build_sprites(display, position, &matrix);

        Tetromino
        {
            shape: shape,

            position: position,

            matrix: matrix,
            sprites: sprites
        }
    }

    pub fn set_position(&mut self, position: Vector2<f32>)
    {
        for ref sprite in self.sprites.iter()
        {
            //sprite.set_position
        }
    }

    fn build_matrix(shape: &Shape) -> CellMatrix
    {
        let mut matrix = CellMatrix::new(4, 4);

        matrix.set_cell(0, 1, Cell::Occupied);
        matrix.set_cell(1, 1, Cell::Occupied);
        matrix.set_cell(2, 1, Cell::Occupied);
        matrix.set_cell(3, 1, Cell::Occupied);

        matrix
    }

    fn build_sprites(display: &mut RootWindow, position: Vector2<f32>, matrix: &CellMatrix)
        -> Vec<Sprite>
    {
        let mut sprites = Vec::<Sprite>::new();

        for y in 0..matrix.height
        {
            for x in 0..matrix.width
            {
                match matrix.get_cell(x, y)
                {
                    Cell::Occupied =>
                        sprites.push(
                            Sprite::new(&display.display, 0, [0.0, 1.0, 0.0, 1.0],
                            position + Vector2::new(x as f32 * 8.0, y as f32 * 8.0)).unwrap()
                        ),
                    _ => ()
                }
            }
        }

        sprites
    }
}
