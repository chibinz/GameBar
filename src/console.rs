use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::dma::DMA;
use crate::timer::Timers;
use crate::interrupt::IRQController;
use crate::keyboard::Keypad;
use crate::memory::Memory;

use minifb::Window;
use minifb::WindowOptions;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub dma   : DMA,
    pub timers: Timers,
    pub irqcnt: IRQController,
    pub keypad: Keypad,
    pub memory: Memory,

    pub window: Window,
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

            window: Window::new
            (
                "GameBar",
                240,
                160,
                WindowOptions
                {
                    scale: minifb::Scale::X2,
                    ..WindowOptions::default()
                }
            ).unwrap(),

            magic: 0xdeadbeef,
        }
    }

    /// Render a frame
    pub fn step_frame(&mut self)
    {
        let cpu    = &mut self.cpu;
        let ppu    = &mut self.ppu;
        let memory = &mut self.memory;
        let timers = &mut self.timers;
        // let dma    = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        for _ in 0..160
        {
            ppu.hdraw(irqcnt);
            cpu.run(960, memory);
            timers.run(960, irqcnt);


            ppu.hblank(irqcnt);
            cpu.run(272, memory);
            timers.run(272, irqcnt);
        }

        for _ in 0..68
        {
            ppu.vblank(irqcnt);
            cpu.run(1232, memory);
            timers.run(1232, irqcnt);
        }

        self.window.update_with_buffer(&ppu.buffer, 240, 160).unwrap();
    }

    /// Single step CPU, for debugging purpose
    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);
        self.cpu.print();
    }
}