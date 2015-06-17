#[derive(Copy, Clone)]
pub enum Cell
{
    Occupied,
    Unoccupied
}

pub struct CellMatrix
{
    pub width: u8,
    pub height: u8,

    matrix: Vec<Vec<Cell>>,
}

impl CellMatrix
{
    pub fn new(width: u8, height: u8) -> CellMatrix
    {
        let mut matrix = Vec::new();

        for _ in 0..height
        {
            let mut row = Vec::new();

            for _ in 0..width
            {
                row.push(Cell::Unoccupied);
            }

            matrix.push(row);
        }

        CellMatrix
        {
            width: width,
            height: height,

            matrix: matrix,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Cell
    {
        self.matrix[y as usize][x as usize]
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: Cell)
    {
        self.matrix[y as usize][x as usize] = cell;
    }

    pub fn rotate_left(&mut self)
    {
        let mut temp = CellMatrix::new(self.height, self.width);

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                temp.set_cell(y, x,
                    self.get_cell(self.width - x - 1, y));
            }
        }

        self.height = temp.height;
        self.width = temp.width;
        self.matrix = temp.matrix;
    }

    pub fn rotate_right(&mut self)
    {
        let mut temp = CellMatrix::new(self.height, self.width);

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                temp.set_cell(y, x,
                    self.get_cell(x, self.height - y - 1));
            }
        }

        self.height = temp.height;
        self.width = temp.width;
        self.matrix = temp.matrix;
    }
}
