mod alu;
mod arm;
mod bus;
mod register;
mod shifter;
mod thumb;

pub use util::Bus;

use register::{Cpsr, PsrMode};

// const LEN: usize = 1024;
// static mut INDEX: usize = 0;
// static mut TRACE: [Cpu; LEN] = [Cpu::new(); 1024];
// #[allow(dead_code)]
// fn push_cpu(c: Cpu) {
//     unsafe {
//         TRACE[INDEX] = c;
//         INDEX += 1;
//         INDEX %= LEN;
//     }
// }

// pub fn backtrace_on_panic(&self) {
//     std::panic::set_hook(Box::new(Self::panic_hook));
// }

// fn panic_hook(p: &std::panic::PanicInfo) {
//     unsafe {
//         for i in 0..LEN {
//             let c = &TRACE[(INDEX + i) % LEN];
//             util::error!("{:?}", c,);
//         }
//         util::error!("{}", p);
//     }
// }

#[derive(Clone)]
pub struct Cpu {
    ir: u32,      // Next instruction to execute
    r: [u32; 16], // General purpose registers

    cpsr: Cpsr,      // Current Program Status Register
    spsr: u32,       // Saved Program Status Register (of current mode)
    bank: [u32; 27], // Banked registers

    // 0 - 6:   R8_sys - R14_sys
    // 7 - 14:  R8_fiq - R14_fiq, SPSR_fiq
    // 15 - 17: R13_svc, R14_svc, SPSR_svc
    // 18 - 20: R13_abt, R14_abt, SPSR_abt
    // 21 - 23: R13_irq, R14_irq, SPSR_irq
    // 24 - 26: R13_und, R14_und, SPSR_und
    pub cycles: i32,            // Ticks consumed for current instruction
    pub remaining: i32,         // Remaining ticks till run finish,
    pub callback: Option<fn()>, // Callback before an instruction is executed
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            ir: 0,
            r: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],

            // On reset, CPSR is forced to supervisor mode
            // and I and F bits in CPSR is set.
            cpsr: Cpsr::new(),
            spsr: 0,
            bank: [0; 27],

            cycles: 0,
            remaining: 0,
            callback: None,
        }
    }

    pub fn skip_bios(&mut self) {
        self.r[15] = 0x08000004;
        self.r[13] = 0x03007f00;

        self.bank[5] = 0x03007f00; // User SP
        self.bank[12] = 0x03007fa0; // IRQ SP
        self.bank[15] = 0x03007fe0; // Supervisor SP
    }

    pub fn run(&mut self, cycles: i32, bus: &mut impl Bus) {
        self.remaining += cycles;

        while self.remaining > 0 {
            // Poll dma
            self.remaining -= self.step(bus);
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) -> i32 {
        if let Some(f) = self.callback {
            f();
        }

        // At least one sequential cycle for any instruction
        self.cycles = 1;

        if self.in_thumb_mode() {
            thumb::step(self, bus);
        } else {
            arm::step(self, bus);
        }

        self.cycles
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

    pub fn flush(&mut self) {
        // Instruction address are forcibly word / halfword aligned
        self.r[15] &= !(self.inst_width() - 1);

        self.r[15] += self.inst_width();

        // A write to R15 or branch will add 1S + 1N cycles
        // self.cycles += Bus::access_timing(self.r[15], self.inst_width() / 2);
    }

    pub fn interrupt(&mut self, mode: PsrMode, lr: u32, pc: u32) {
        let spsr = self.get_cpsr();

        // Switch mode(register bank), disable interrupt, save CPSR
        self.switch_mode(mode);
        self.set_spsr(spsr, false);
        self.cpsr.i = true;

        self.set_r(14, lr);
        self.set_r(15, pc);
    }

    pub fn software_interrupt(&mut self) {
        util::info!("Software interrupt!");
        util::info!("{:#?}", self);

        let lr = self.r[15] - self.inst_width();
        self.interrupt(PsrMode::Supervisor, lr, 0x8);
    }

    pub fn hardware_interrupt(&mut self) {
        if self.cpsr.i {
            return;
        }

        util::info!("Hardware interrupt!");
        util::info!("{:#?}", self);

        let lr = self.r[15] + if self.in_thumb_mode() { 2 } else { 0 };
        self.interrupt(PsrMode::Irq, lr, 0x18);
    }

    #[inline]
    pub fn in_thumb_mode(&self) -> bool {
        self.cpsr.t
    }

    #[inline]
    pub fn inst_width(&self) -> u32 {
        if self.in_thumb_mode() {
            2
        } else {
            4
        }
    }

    pub fn disassemble(instr: u32, thumb: bool) -> String {
        if thumb {
            thumb::disassemble(instr as u16)
        } else {
            arm::disassemble(instr)
        }
    }

    pub fn set_callback(&mut self, f: fn()) {
        self.callback = Some(f);
    }
}

use std::fmt::*;
impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            concat!(
                "\n",
                "R0  = {:08x} R1  = {:08x} R2  = {:08x} R3  = {:08x}\n",
                "R4  = {:08x} R5  = {:08x} R6  = {:08x} R7  = {:08x}\n",
                "R8  = {:08x} R9  = {:08x} R10 = {:08x} R11 = {:08x}\n",
                "R12 = {:08x} R13 = {:08x} R14 = {:08x} R15 = {:08x}\n",
                "PSR = {:?}\n",
                "IR  = {:08x} {}\n"
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
            self.cpsr,
            self.ir,
            Self::disassemble(self.ir, self.in_thumb_mode())
        )
    }
}
