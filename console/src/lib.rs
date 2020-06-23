mod cpu;
mod ppu;
mod dma;
mod timer;
mod interrupt;
mod memory;
mod keypad;
mod event;

pub mod util;

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::dma::DMA;
use crate::timer::Timers;
use crate::interrupt::IRQController;
use crate::keypad::Keypad;
use crate::memory::Memory;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub dma   : DMA,
    pub timers: Timers,
    pub irqcnt: IRQController,
    pub keypad: Keypad,
    pub memory: Memory,

    pub magic : u32,
}

impl Console
{
    pub fn new() -> Console
    {
        Self
        {
            cpu   : CPU::new(),
            ppu   : PPU::new(),
            dma   : DMA::new(),
            irqcnt: IRQController::new(),
            timers: Timers::new(),
            keypad: Keypad::new(),
            memory: Memory::new(),

            magic: 0xdeadbeef,
        }
    }

    pub fn init(&mut self)
    {
        self.memory.console = self as *mut Self;
        self.irqcnt.cpu = &mut self.cpu as *mut cpu::CPU;
    }

    /// Render a frame
    pub fn step_frame(&mut self)
    {
        // self.schedule();
        let cpu    = &mut self.cpu;
        let ppu    = &mut self.ppu;
        let memory = &mut self.memory;
        let timers = &mut self.timers;
        let dma    = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        for _ in 0..160
        {
            ppu.increment_vcount(irqcnt);
            ppu.hdraw();
            cpu.run(960, dma, irqcnt, memory);
            timers.run(960, irqcnt);

            dma.request_hblank();

            ppu.hblank(irqcnt);
            cpu.run(272, dma, irqcnt, memory);
            timers.run(272, irqcnt);
        }

        dma.request_vblank();

        for _ in 0..68
        {
            ppu.increment_vcount(irqcnt);
            ppu.vblank(irqcnt);
            cpu.run(1232, dma, irqcnt, memory);
            timers.run(1232, irqcnt);
        }
    }

    /// Single step CPU, for debugging purpose
    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);
        self.cpu.print(&mut self.memory);
    }
}