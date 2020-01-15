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
    pub r: [u32; 16],       // General purpose registers

    cpsr : u32,             // Current Program Status Register
    spsr : u32,             // Saved Program Status Register (of current mode)
    bank : [u32; 27],       // Banked registers

    // 0 - 6:   R8_sys - R14_sys
    // 7 - 14:  R8_fiq - R14_fiq, SPSR_fiq
    // 15 - 17: R13_svc, R14_svc, SPSR_svc
    // 18 - 20: R13_abt, R14_abt, SPSR_abt
    // 21 - 23: R13_irq, R14_irq, SPSR_irq
    // 24 - 26: R13_und, R14_und, SPSR_und
}

impl CPU
{
    pub fn new() -> Self
    {
        Self
        {
            instruction: 0,
            r   : [0; 16],

            // On reset, CPSR is forced to supervisor mode
            // and I and F bits in CPSR is set.
            cpsr: 0b11010011, 
            spsr: 0,
            bank: [0; 27],
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

    pub fn flush(&mut self)
    {
        if self.in_thumb_mode()
        {
            self.r[15] &= 0xfffffffe;
            self.r[15] += 2;
        }
        else
        {
            self.r[15] &= 0xfffffffc;
            self.r[15] += 4;
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