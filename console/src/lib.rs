mod dma;
mod event;
mod interrupt;
mod keypad;
mod memory;
mod ppu;
mod timer;

use crate::dma::DMA;
use crate::interrupt::IRQController;
use crate::keypad::Keypad;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::timer::Timers;
use cpu::CPU;

pub struct Console {
    pub cpu: CPU,
    pub ppu: PPU,
    pub dma: DMA,
    pub timers: Timers,
    pub irqcnt: IRQController,
    pub keypad: Keypad,
    pub memory: Memory,

    pub magic: u32,
}

impl Console {
    pub fn new() -> Console {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            dma: DMA::new(),
            irqcnt: IRQController::new(),
            timers: Timers::new(),
            keypad: Keypad::new(),
            memory: Memory::new(),

            magic: 0xdeadbeef,
        }
    }

    pub fn init(&mut self) {
        self.memory.console = self as *mut Self;
    }

    /// Render a frame
    pub fn step_frame(&mut self) {
        // self.schedule();
        let cpu = &mut self.cpu;
        let ppu = &mut self.ppu;
        let memory = &mut self.memory;
        let timers = &mut self.timers;
        let dma = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        for _ in 0..160 {
            ppu.increment_vcount(irqcnt);
            ppu.hdraw();

            for _ in 0..960 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }

            dma.request_hblank();
            ppu.hblank(irqcnt);

            for _ in 0..272 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }
        }

        dma.request_vblank();

        for _ in 0..68 {
            ppu.increment_vcount(irqcnt);
            ppu.vblank(irqcnt);

            for _ in 0..1272 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }
        }
    }

    pub fn step_dma_cpu_timer(
        dma: &mut DMA,
        cpu: &mut CPU,
        memory: &mut Memory,
        timers: &mut Timers,
        irqcnt: &mut IRQController,
    ) {
        let t = if dma.is_active() {
            dma.step(irqcnt, memory)
        } else {
            cpu.step(memory)
        };
        irqcnt.check(cpu);

        timers.run(t, irqcnt);
    }

    /// Single step CPU, for debugging purpose
    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
        // self.cpu.print(&mut self.memory);
    }
}
