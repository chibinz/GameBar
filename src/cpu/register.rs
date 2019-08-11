pub struct Register
{
    pub r: [u32; 16],
    pub cpsr: u32,
    pub spsr: u32,
}

impl Register
{
    pub fn new() -> Self
    {
        Self
        {
            r: [0; 16],
            cpsr: 0,
            spsr: 0,
        }
    }
}