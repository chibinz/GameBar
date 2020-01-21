pub mod register;
pub mod arm;
pub mod thumb;
pub mod alu;
pub mod barrel_shifter;

use register::PSRBit::*;
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
        let mut cpu =
        Self
        {
            instruction: 0,
            r   : [0; 16],

            // On reset, CPSR is forced to supervisor mode
            // and I and F bits in CPSR is set.
            cpsr: 0b11010011, 
            spsr: 0,
            bank: [0; 27],
        };

        cpu.r[15] = 0x08000004;
        cpu.r[13] = 0x03007f00;
        
        cpu
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
        self.get_cpsr_bit(T)
    }

    pub fn print(&self)
    {
        let mut str = String::new();

        // Print general purpose registers R0 - R15
        for i in 0..16
        {
            if i % 4 == 0 && i > 0 {str += "\n";}

            str += &format!("R{:<2} = {:08x} ", i, self.r[i as usize]);
        }

        // Print current program status register
        str += "\n";
        str += &format!("PSR = {:08x} ", self.get_cpsr());
        str += "[";
        str += if self.get_cpsr_bit(N) {"N"} else {"."};
        str += if self.get_cpsr_bit(Z) {"Z"} else {"."};
        str += if self.get_cpsr_bit(C) {"C"} else {"."};
        str += if self.get_cpsr_bit(V) {"V"} else {"."};
        str += if self.get_cpsr_bit(I) {"I"} else {"."};
        str += if self.get_cpsr_bit(F) {"F"} else {"."};
        str += if self.get_cpsr_bit(T) {"T"} else {"."};
        str += "]";

        println!("{}", str)
    }
}