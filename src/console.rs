use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::dma::DMA;
use crate::timer::Timers;
use crate::interrupt::IRQController;
use crate::interrupt::Interrupt::*;
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
        let dma    = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        for _ in 0..160
        {
            // self.vmatch_irq();
            cpu.run(960, memory);
            timers.run(960);

            ppu.render(&memory);
            memory.set_hblank_flag(true);
    
            dma.request(memory);
            irqcnt.request(HBlank, cpu);

            cpu.run(272, memory);
            timers.run(272);
    
            memory.inc_vcount();
            memory.set_hblank_flag(false); 
        }

        memory.set_vblank_flag(true);
        irqcnt.request(VBlank, cpu);

        for _ in 0..68
        {
            // self.vmatch_irq();
            cpu.run(1232, memory);
            timers.run(1232);

            memory.inc_vcount();
        }

        memory.clr_vcount();
        memory.set_vblank_flag(false);
        
        self.window.update_with_buffer(&ppu.buffer, 240, 160).unwrap();
    }

    pub fn vmatch_irq(&mut self)
    {
        let vmatch = self.memory.load16(0x04000004) >> 8;
        let vcount = self.memory.get_vcount();

        if vcount == vmatch
        {
            self.irqcnt.request(VCount, &mut self.cpu);
        }
    }

    /// Single step CPU, for debugging purpose
    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);

        if self.memory.get_vcount() > 228
        {
            self.memory.clr_vcount();
        }

        if self.cpu.counter % 1232 == 0
        {
            self.ppu.render(&mut self.memory);
            self.memory.inc_vcount();
            self.window.update_with_buffer(&self.ppu.buffer, 240, 160).unwrap();
        }
    }
}