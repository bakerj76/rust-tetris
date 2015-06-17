pub struct Rect
{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect
{
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect
    {
        Rect
        {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn top(&self) -> f32
    {
        Rect::min(self.y, self.y + self.height)
    }

    pub fn bottom(&self) -> f32
    {
        Rect::max(self.y, self.y + self.height)
    }

    pub fn left(&self) -> f32
    {
        Rect::min(self.x, self.x + self.width)
    }

    pub fn right(&self) -> f32
    {
        Rect::max(self.x, self.x + self.width)
    }

    fn min(x: f32, y: f32) -> f32
    {
        if x < y { x } else { y }
    }

    fn max(x: f32, y: f32) -> f32
    {
        if x > y { x } else { y }
    }
}
