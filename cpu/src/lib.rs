mod alu;
mod arm;
mod barrel_shifter;
mod bus;
mod register;
mod thumb;

pub use bus::Bus;

use register::PSRBit::*;

#[derive(Clone, Copy)]
pub struct CPU {
    ir: u32,      // Next instruction to execute
    r: [u32; 16], // General purpose registers

    cpsr: u32,       // Current Program Status Register
    spsr: u32,       // Saved Program Status Register (of current mode)
    bank: [u32; 27], // Banked registers

    // 0 - 6:   R8_sys - R14_sys
    // 7 - 14:  R8_fiq - R14_fiq, SPSR_fiq
    // 15 - 17: R13_svc, R14_svc, SPSR_svc
    // 18 - 20: R13_abt, R14_abt, SPSR_abt
    // 21 - 23: R13_irq, R14_irq, SPSR_irq
    // 24 - 26: R13_und, R14_und, SPSR_und
    pub cycles: i32, // Ticks consumed for current instruction
    pub remaining: i32, // Remaining ticks till run finish,

                     // The CPU halts when DMA is active.
                     // An unfortunate fact that I have to use an unsafe pointer
                     // to poll the current status of the DMA
}

impl CPU {
    pub const fn new() -> Self {
        let mut cpu = Self {
            ir: 0,
            r: [0; 16],

            // On reset, CPSR is forced to supervisor mode
            // and I and F bits in CPSR is set.
            cpsr: 0b11011111,
            spsr: 0,
            bank: [0; 27],

            cycles: 0,
            remaining: 0,
        };

        cpu.r[15] = 0x08000004;
        cpu.r[13] = 0x03007f00;

        cpu.bank[5] = 0x03007f00; // User SP
        cpu.bank[12] = 0x03007fa0; // IRQ SP
        cpu.bank[15] = 0x03007fe0; // Supervisor SP

        cpu
    }

    pub fn run(&mut self, cycles: i32, bus: &mut impl Bus) {
        self.remaining += cycles;

        while self.remaining > 0 {
            // Poll dma
            self.remaining -= self.step(bus);
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) -> i32 {
        // if self.booted {self.print(memory)}

        // At least one sequential cycle for any instruction
        self.cycles = 1;

        if self.r[15] == 0x08001ec0 {
            return 0;
        }

        if self.in_thumb_mode() {
            thumb::step(self, bus);
        } else {
            arm::step(self, bus);
        }

        return self.cycles;
    }

    #[inline]
    pub fn r(&self, n: u32) -> u32 {
        self.r[n as usize]
    }

    #[inline]
    pub fn set_r(&mut self, n: u32, value: u32) {
        self.r[n as usize] = value;
        if n == 15 {
            self.flush();
        }
    }

    #[inline]
    pub fn carry(&self) -> bool {
        self.get_cpsr_bit(C)
    }

    #[inline]
    pub fn set_carry(&mut self, carry: bool) {
        self.set_cpsr_bit(C, carry)
    }

    pub fn flush(&mut self) {
        // Instruction address are forcibly word / halfword aligned
        self.r[15] &= !(self.inst_width() - 1);

        self.r[15] += self.inst_width();

        // A write to R15 or branch will add 1S + 1N cycles
        // self.cycles += Bus::access_timing(self.r[15], self.inst_width() / 2);
    }

    pub fn software_interrupt(&mut self) {
        log::info!("Software interrupt!");
        let lr = self.r[15] - self.inst_width();
        let spsr = self.get_cpsr();

        // Switch mode(register bank), disable interrupt, save CPSR
        self.set_cpsr(register::PSRMode::Supervisor as u32, false);
        self.set_cpsr_bit(I, true);
        self.set_spsr(spsr, false);

        self.r[14] = lr;
        self.r[15] = 0x08;
        self.flush();
    }

    pub fn hardware_interrupt(&mut self) {
        if self.get_cpsr_bit(I) {
            return;
        }

        log::info!("Hardware interrupt!");
        log::info!("\n{}", self.trace());

        let lr = self.r[15]; // Not sure!
        let spsr = self.get_cpsr();

        self.set_cpsr(register::PSRMode::IRQ as u32, false);
        self.set_cpsr_bit(I, true);
        self.set_spsr(spsr, false);

        self.r[14] = lr;
        self.r[15] = 0x18;
        self.flush();
    }

    #[inline]
    pub fn in_thumb_mode(&self) -> bool {
        self.get_cpsr_bit(T)
    }

    #[inline]
    pub fn inst_width(&self) -> u32 {
        if self.in_thumb_mode() {
            2
        } else {
            4
        }
    }

    #[allow(dead_code)]
    pub fn trace(&self) -> String {
        format!(
            concat!(
                "R0  = {:08x} R1  = {:08x} R2  = {:08x} R3  = {:08x}\n",
                "R4  = {:08x} R5  = {:08x} R6  = {:08x} R7  = {:08x}\n",
                "R8  = {:08x} R9  = {:08x} R10 = {:08x} R11 = {:08x}\n",
                "R12 = {:08x} R13 = {:08x} R14 = {:08x} R15 = {:08x}\n",
                "PSR = {:08x} [{}{}{}{}{}{}{}]\n"
            ),
            self.r[0],
            self.r[1],
            self.r[2],
            self.r[3],
            self.r[4],
            self.r[5],
            self.r[6],
            self.r[7],
            self.r[8],
            self.r[9],
            self.r[10],
            self.r[11],
            self.r[12],
            self.r[13],
            self.r[14],
            self.r[15],
            self.get_cpsr(),
            if self.get_cpsr_bit(N) { "N" } else { "." },
            if self.get_cpsr_bit(Z) { "Z" } else { "." },
            if self.get_cpsr_bit(C) { "C" } else { "." },
            if self.get_cpsr_bit(V) { "V" } else { "." },
            if self.get_cpsr_bit(I) { "I" } else { "." },
            if self.get_cpsr_bit(F) { "F" } else { "." },
            if self.get_cpsr_bit(T) { "T" } else { "." },
        )
    }

    pub fn disassemble(instr: u32, thumb: bool) -> String {
        if thumb {
            thumb::disassemble(instr as u16)
        } else {
            arm::disassemble(instr)
        }
    }

    pub fn backtrace_on_panic(&self) {
        std::panic::set_hook(Box::new(|_| Self::panic_hook()));
    }

    fn panic_hook() {
        unsafe {
            for i in 0..LEN {
                let c = &TRACE[(INDEX + i) % LEN];
                println!(
                    "{}{}",
                    c.trace(),
                    Self::disassemble(c.ir, c.in_thumb_mode())
                );
            }
        }
    }
}

const LEN: usize = 1024;
static mut INDEX: usize = 0;
static mut TRACE: [CPU; LEN] = [CPU::new(); 1024];
fn push_cpu(c: CPU) {
    unsafe {
        TRACE[INDEX] = c;
        INDEX += 1;
        INDEX %= LEN;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace() {
        let cpu = CPU::new();
        println!("{}", cpu.trace());
    }
}
