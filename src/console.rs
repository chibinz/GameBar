use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::dma::DMA;
use crate::timer::Timer;
use crate::memory::Memory;

use minifb::Window;
use minifb::WindowOptions;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub dma   : Vec<DMA>,
    pub timer : Vec<Timer>,
    pub memory: Memory,

    pub window: Window,
}

impl Console
{
    pub fn new() -> Console
    {
        let mut c = Self
        {
            cpu   : CPU::new(),
            ppu   : PPU::new(),
            dma   : vec![DMA::new(); 4],
            timer : vec![Timer::new(); 4],
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
        };

        for i in 0..4
        {
            c.dma[i].index = i;
        }

        for i in 0..4
        {
            c.timer[i].index = i;
            c.timer[i].data  = c.memory.get_timer_data(i);
        }

        c
    }

    /// Render a frame
    pub fn step_frame(&mut self)
    {
        for _ in 0..160
        {
            self.cpu.run(960, &mut self.memory);
            self.increment_timer(960);

            self.ppu.render(&self.memory);
            self.memory.set_hblank_flag(true);
    
            self.cpu.run(272, &mut self.memory);
            self.increment_timer(272);
    
            self.memory.inc_vcount();
            self.memory.set_hblank_flag(false); 

            self.dma_transfer();
        }

        self.memory.set_vblank_flag(true);

        // self.cpu.print(&self.memory);
        // dbg!(&self.dma[2]);
        
        // self.memory.store16(0x300310c, 7);

        for _ in 0..68
        {
            self.cpu.run(1232, &mut self.memory);
            self.increment_timer(1232);

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

    pub fn dma_transfer(&mut self)
    {
        for i in 0..4
        {
            self.memory.update_dma(&mut self.dma[i]);

            self.dma[i].transfer(&mut self.memory);
        }
    }

    pub fn increment_timer(&mut self, value: u32)
    {
        for i in 0..4
        {
            self.memory.update_tmcnt(&mut self.timer[i]);

            self.timer[i].increment_counter(value);
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_timer()
    {
        let mut console = Console::new();

        console.memory.store16(0x4000100, 0xff00);
        console.timer[0].increment_data(1);
        assert_eq!(console.memory.load16(0x4000100), 0xff01);
    }
}