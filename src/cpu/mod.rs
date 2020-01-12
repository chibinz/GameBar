pub mod register;
pub mod arm;
pub mod thumb;
pub mod alu;
pub mod barrel_shifter;

use std::fmt;

use crate::memory::Memory;

pub struct CPU
{
    pub instruction: u32,   // Next instruction to execute
    pub flushed: bool,      // Determine whether the pipeline is empty
    pub r: [u32; 16],       // General purpose registers

    cpsr : u32,
    spsr : [u32; 5],
}

impl CPU
{
    pub fn new() -> Self
    {
        Self
        {
            instruction: 0,
            flushed: true,
            r   : [0; 16],
            cpsr: 0b10011, // On reset, the CPSR is forced to supervisor mode
            spsr: [0; 5],
        }
    }

    pub fn step(&mut self, memory: &mut Memory)
    {
        if self.in_thumb_mode()
        {
            thumb::step(self, memory);
        }
        else
        {
            arm::step(self, memory);
        }
    }

    #[inline]
    pub fn in_thumb_mode(&self) -> bool
    {
        self.get_cpsr_bit(register::PSRBit::T)
    }
}

impl fmt::Display for CPU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for i in 0..16
        {
            if i % 4 == 0 && i > 0 {print!("\n");}

            print!("R{:<2} = {:08x} ", i, self.r[i as usize]);
        }

        write!(f, "\nCPSR = {:032b}", self.get_cpsr())
    }
}