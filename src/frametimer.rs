use tetris::Tetris;

pub struct FrameTimer
{
    current_frame: u32,
    pub end_frame: u32,

    enabled: bool,
    repeating: bool,

    callback: Box<FnMut()>
}


/// TODO: Move callback to constructor
impl FrameTimer
{
    pub fn new<F>(end_frame: u32, repeating: bool, callback: F)
        -> FrameTimer where F: FnMut()
    {
        FrameTimer
        {
            current_frame: 0,
            end_frame: end_frame,

            enabled: false,
            repeating: repeating,

            callback: Box::new(callback),
        }
    }

    pub fn start(&mut self)
    {
        self.enabled = false;
    }

    pub fn update(&mut self)
    {
        if (!self.enabled) { return; }

        self.current_frame += 1;

        if (self.current_frame >= self.end_frame)
        {
            self.callback();

            if (self.repeating) { self.reset(); } else { self.stop(); }
        }
    }

    pub fn stop(&mut self)
    {
        self.enabled = false;
    }

    pub fn reset(&mut self)
    {
        self.current_frame = 0;
    }
}
