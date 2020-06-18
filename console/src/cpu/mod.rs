pub mod arm;
pub mod thumb;

mod register;
mod alu;
mod barrel_shifter;

use register::PSRBit::*;
use crate::memory::Memory;
use crate::dma::DMA;
use crate::interrupt::IRQController;

#[derive(Debug)]
pub struct CPU
{
    pub instruction: u32,   // Next instruction to execute
    pub prefetched: u32,    // Prefetched instruction
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

    pub booted   : bool,
    pub remaining: i32,     // Remaining ticks till run finish,

    // The CPU halts when DMA is active.
    // An unfortunate fact that I have to use an unsafe pointer
    // to poll the current status of the DMA
    pub dma: *mut DMA,
}

impl CPU
{
    pub fn new() -> Self
    {
        let mut cpu =
        Self
        {
            instruction: 0,
            prefetched: 0,
            r   : [0; 16],

            // On reset, CPSR is forced to supervisor mode
            // and I and F bits in CPSR is set.
            cpsr: 0b11011111,
            spsr: 0,
            bank: [0; 27],

            booted   : false,
            remaining: 0,

            dma: 0 as *mut DMA,
        };

        cpu.r[15] = 0x00000004;
        cpu.r[13] = 0x03007f00;

        cpu.bank[5]  = 0x03007f00; // User SP
        cpu.bank[12] = 0x03007fa0; // IRQ SP
        cpu.bank[15] = 0x03007fe0; // Supervisor SP

        cpu
    }

    pub fn run(&mut self, cycles: i32, memory: &mut Memory, irqcnt: &mut IRQController)
    {
        self.remaining += cycles;
        let dma = unsafe {&mut *self.dma};

        while self.remaining > 0
        {
            // Poll dma
            if dma.is_active()
            {
                dma.step(irqcnt, memory);
                self.remaining -= 2;
            }
            else
            {
                self.step(memory);
            }
        }
    }

    pub fn step(&mut self, memory: &mut Memory) -> i32
    {
        self.booted = self.booted || (self.r[15] >= 0x08000000);
        // if self.booted {self.print()}

        if self.in_thumb_mode()
        {
            thumb::step(self, memory);
        }
        else
        {
            arm::step(self, memory);
        }

        // Normal execution takes 1S cycle
        self.remaining -= 1;

        2
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

        // A write to R15 or branch will add 1S + 1N cycles
        self.remaining -= 2;
    }

    pub fn software_interrupt(&mut self)
    {
        let lr = self.r[15] - if self.in_thumb_mode() {2} else {4};
        let spsr = self.get_cpsr();

        // Switch mode(register bank), disable interrupt, save CPSR
        self.set_cpsr(register::PSRMode::Supervisor as u32, false);
        self.set_cpsr_bit(I, true);
        self.set_spsr(spsr, false);

        self.r[14] = lr;
        self.r[15] = 0x08;
        self.flush();
    }

    pub fn hardware_interrupt(&mut self)
    {
        if self.get_cpsr_bit(I) {return}

        let lr = self.r[15] - if self.in_thumb_mode() {0} else {4};
        let spsr = self.get_cpsr();

        self.set_cpsr(register::PSRMode::IRQ as u32, false);
        self.set_cpsr_bit(I, true);
        self.set_spsr(spsr, false);

        self.r[14] = lr;
        self.r[15] = 0x18;
        self.flush();
    }

    #[inline]
    pub fn in_thumb_mode(&self) -> bool
    {
        self.get_cpsr_bit(T)
    }

    #[allow(dead_code)]
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
        str += "\n";

        let instruction = self.prefetched;

        if self.in_thumb_mode()
        {
            let address = self.r[15] - 2;
            str += &format!("{:08x}: {:04x} ", address, instruction);
            str += &format!("{}", thumb::disassemble::disassemble(instruction as u16));
        }
        else
        {
            let address = self.r[15] - 4;
            str += &format!("{:08x}: {:08x} ", address, instruction);
            str += &format!("{}", arm::disassemble::disassemble(instruction));
        }

        str += "\n";

        println!("{}", str)
    }
}