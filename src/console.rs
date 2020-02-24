use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::dma::DMA;
use crate::timer::Timers;
use crate::memory::Memory;

use minifb::Window;
use minifb::WindowOptions;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub dma   : DMA,
    pub timers: Timers,
    pub memory: Memory,

    pub window: Window,
}

impl Console
{
    pub fn new() -> Console
    {
        let mut m = Memory::new();

        Self
        {
            cpu   : CPU::new(),
            ppu   : PPU::new(),
            dma   : DMA::new(),
            timers: Timers::new(&mut m),
            
            memory: m,

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
        }
    }

    /// Render a frame
    pub fn step_frame(&mut self)
    {
        for _ in 0..160
        {
            self.cpu.run(960, &mut self.memory);
            self.timers.update(960, &mut self.memory);

            self.ppu.render(&self.memory);
            self.memory.set_hblank_flag(true);
    
            self.dma.request(&mut self.memory);

            self.cpu.run(272, &mut self.memory);
            self.timers.update(272, &mut self.memory);
    
            self.memory.inc_vcount();
            self.memory.set_hblank_flag(false); 
        }

        self.memory.set_vblank_flag(true);

        for _ in 0..68
        {
            self.cpu.run(1232, &mut self.memory);
            self.timers.update(1232, &mut self.memory);

            self.memory.inc_vcount();
        }

        self.memory.clr_vcount();
        self.memory.set_vblank_flag(false);
        
        self.window.update_with_buffer(&self.ppu.buffer, 240, 160).unwrap();
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