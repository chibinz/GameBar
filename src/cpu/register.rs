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

    pub fn get_cpsr_t(&self) -> bool
    {
        self.cpsr & 0b00010000 == 1
    }

    pub fn set_cpsr_t(&mut self, t: bool)
    {
        if t {self.cpsr |= 0b00010000};
    }
}