use crate::cpu::CPU;
use crate::memory::Memory;

pub struct Console
{
    pub cpu   : CPU,
    pub memory: Memory,
}

impl Console
{
    pub fn new() -> Console
    {
        Self
        {
            cpu   : CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn step(&mut self)
    {
        self.cpu.register.r[15] += 4;
    }

    pub fn load_gamepak(&mut self, gamepak: &String)
    {
        self.memory.load_rom(gamepak);
        self.cpu.register.r[15] = 0x08000000;
    }

}