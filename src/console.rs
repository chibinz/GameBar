use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::memory::Memory;

use minifb::Window;
use minifb::WindowOptions;

pub struct Console
{
    pub cpu   : CPU,
    pub ppu   : PPU,
    pub memory: Memory,

    pub window: Window,
}

impl Console
{
    pub fn new() -> Console
    {
        Self
        {
            cpu   : CPU::new(),
            ppu   : PPU::new(),
            memory: Memory::new(),

            window: Window::new
            (
                "ESC to exit",
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

    // Render a frame
    pub fn run(&mut self)
    {
        for _ in 0..160
        {
            self.cpu.run(960, &mut self.memory);

            self.ppu.render(&mut self.memory);
            self.memory.set_hblank_flag(true);
    
            self.cpu.run(272, &mut self.memory);
    
            self.memory.inc_vcount();
            self.memory.set_hblank_flag(false); 
        }
        
        self.memory.set_vblank_flag(true);

        for _ in 0..68
        {
            self.cpu.run(1232, &mut self.memory);

            self.memory.inc_vcount();
        }

        self.memory.clr_vcount();
        self.memory.set_vblank_flag(false);
        
        self.window.update_with_buffer(&self.ppu.buffer, 240, 160).unwrap();
    }

    // Single step CPU, for debugging purpose
    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);
        self.ppu.render(&mut self.memory);
        self.memory.inc_vcount();

        if self.memory.get_vcount() > 228
        {
            self.memory.clr_vcount();
        }

        let f = self.cpu.counter % 280896; // Frame
        let l = f % 1232;                  // Scanline

        if f <= 197120
        {
            if l <= 960
            {
                self.memory.set_hblank_flag(false);
            }
            else
            {
                self.memory.set_hblank_flag(true);
            }
        }
        else
        {
            self.memory.set_hblank_flag(false);
            self.memory.set_vblank_flag(true);
        }

        self.window.update_with_buffer(&self.ppu.buffer, 240, 160).unwrap();
        // self.run();
    }
}