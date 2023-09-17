

mod graphics;


pub struct App
{
    gfx: graphics::Graphics,
}

impl App
{
    pub fn new() -> Self
    {
        Self {
            gfx: graphics::Graphics::new()
        }
    }

    pub fn run(&self)
    {
        
    }
}