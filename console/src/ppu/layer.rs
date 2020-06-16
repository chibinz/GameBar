use super::TRANSPARENT;
use super::window::Window;

#[derive(Clone)]
pub struct Layer
{
    pub pixel: Vec<u16>,
}

impl Layer
{
    pub fn new() -> Self
    {
        Self
        {
            pixel: vec![TRANSPARENT; 240]
        }
    }

    pub fn paint(&mut self, x: u32, color: u16, window: &Window, index: u32)
    {
        if color == TRANSPARENT || x >= 240 {return}

        if window.get_display_flag(x, index)
        {
            self.pixel[x as usize] = color;
        }
    }

    pub fn clear(&mut self)
    {
        for p in self.pixel.iter_mut()
        {
            *p = TRANSPARENT;
        }
    }
}