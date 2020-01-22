use crate::cpu;
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::memory::Memory;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub memory: Memory,
}

impl Console
{
    pub fn new() -> Console
    {
        Self
        {
            cpu   : CPU::new(),
            ppu   : PPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self)
    {
        loop
        {
            self.step();
        }
    }

    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);
        self.ppu.render(&mut self.memory);
    }

    pub fn print(&self)
    {
        self.cpu.print();
        
        if self.cpu.in_thumb_mode()
        {
            let address = self.cpu.r[15] - 2;
            let instruction = self.memory.load16(address);
            print!("{:08x}: ", address);
            print!("{:04x} ", instruction);
            println!("{}", cpu::thumb::disassemble::disassemble(instruction));
        }
        else
        {
            let address = self.cpu.r[15] - 4;
            let instruction = self.memory.load32(address);
            print!("{:08x}: ", address);
            print!("{:08x} ", instruction);
            println!("{}", cpu::arm::disassemble::disassemble(instruction));
        }
    }
}