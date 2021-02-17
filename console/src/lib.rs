mod cart;
mod dma;
// mod event;
mod bus;
mod interrupt;
mod keypad;
mod timer;

use bus::GBus;
use cart::Cart;
use cpu::CPU;
use dma::DMA;
use interrupt::IRQController;
use keypad::Keypad;
use ppu::PPU;
use timer::Timers;

pub struct Console {
    pub cpu: CPU,
    pub ppu: PPU,
    pub dma: DMA,
    pub timers: Timers,
    pub irqcnt: IRQController,
    pub keypad: Keypad,
    pub memory: GBus,
    pub cart: Cart,
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
            memory: GBus::new(),
            cart: Cart::with_rom(Vec::new()),
        }
    }

    pub fn init(&mut self) {
        self.memory.console = self as *mut Self;
    }

    /// Render a frame
    pub fn step_frame(&mut self) {
        use interrupt::Interrupt::*;
        // self.schedule();
        let cpu = &mut self.cpu;
        let ppu = &mut self.ppu;
        let memory = &mut self.memory;
        let timers = &mut self.timers;
        let dma = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        for _ in 0..160 {
            ppu.hdraw();

            for _ in 0..960 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }

            dma.request_hblank();
            if ppu.hblank() {
                irqcnt.request(HBlank);
            }

            for _ in 0..272 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }

            if ppu.increment_vcount() {
                irqcnt.request(VCount);
            }
        }

        dma.request_vblank();
        if ppu.vblank() {
            irqcnt.request(VBlank);
        }

        for _ in 0..68 {
            for _ in 0..1272 {
                Self::step_dma_cpu_timer(dma, cpu, memory, timers, irqcnt);
            }

            if ppu.increment_vcount() {
                irqcnt.request(VCount);
            }
        }

        ppu.rewind();
    }

    #[inline]
    pub fn step_dma_cpu_timer(
        dma: &mut DMA,
        cpu: &mut CPU,
        memory: &mut GBus,
        timers: &mut Timers,
        irqcnt: &mut IRQController,
    ) {
        let t = if dma.is_active() {
            dma.step(irqcnt, memory)
        } else {
            irqcnt.check(cpu);
            cpu.step(memory)
        };

        timers.run(t, irqcnt);
    }

    /// Single step CPU, for debugging purpose
    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
        // self.cpu.print(&mut self.memory);
    }
}
