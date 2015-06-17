use cgmath::Vector2;

use rect::Rect;
use cellmatrix::{Cell, CellMatrix};
use sprite::Sprite;
use rootwindow::RootWindow;
use spritemanager::Textures;


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

    pub board_position: Vector2<f32>,
    pub cell_position: Vector2<i8>,

    matrix: CellMatrix,
    pub sprites: Vec<Sprite>,
}

impl  Tetromino
{
    pub fn new(display: &mut RootWindow, shape: Shape,
        board_position: Vector2<f32>, cell_position: Vector2<i8>) -> Tetromino
    {
        let matrix = Tetromino::build_matrix(&shape);
        let sprites = Tetromino::build_sprites(display, &matrix);

        let mut tetromino = Tetromino
        {
            shape: shape,

            board_position: board_position,
            cell_position: cell_position,

            matrix: matrix,
            sprites: sprites
        };

        tetromino.update_sprites();

        tetromino
    }

    pub fn set_position(&mut self, position: Vector2<i8>)
    {
        self.cell_position = position;
        self.update_sprites();
    }


    pub fn rotate_right(&mut self)
    {
        self.matrix.rotate_right();
        self.update_sprites();
    }

    /// Moves all of the sprites based on the matrix and position
    fn update_sprites(&mut self)
    {
        let mut sprite_index = 0;

        for y in 0..self.matrix.height
        {
            for x in 0..self.matrix.width
            {
                match self.matrix.get_cell(x, y)
                {
                    Cell::Occupied =>
                    {
                        let sprite = &mut self.sprites[sprite_index];
                        let position = self.board_position +
                            Vector2::new((self.cell_position.x + x as i8) as f32 * 16.0,
                                (self.cell_position.y + y as i8) as f32 * 16.0);


                        sprite.set_position(position);
                        sprite_index += 1;
                    },

                    _ => ()
                }
            }
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

    fn build_sprites(display: &mut RootWindow, matrix: &CellMatrix)
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
                            Sprite::new_tinted(&display.display, Textures::SpriteSheet,
                                Rect::new(-8.0, -8.0, 16.0, 16.0),
                                Vector2::new(0.0, 0.0),
                                [0.0, 1.0, 0.0, 1.0])
                                .unwrap()
                        ),
                    _ => ()
                }
            }
        }

        sprites
    }
}
