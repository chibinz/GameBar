//! Prototypical event scheduler
// use std::collections::VecDeque;
use crate::Console;

// A Rough picture for how events are scheduled

// PPU runs for 960 ticks
// /------ HDraw ------/
// DMA, CPU, and Timers run for another 960 ticks
// /------- 960 -------/
// PPU idles for 272 ticks
// / HBlank /
// DMA, CPU, and Timers consumes 272 ticks with decreasing priority
// /-- 272 -/
// /.................../
// /......../
// Repeat until VCount = 160

// DMA, CPU, Timers still work similarly
// /------ VBlank -----/
// /------- 960 -------/
// / HBlank /
// /-- 272 -/

enum Event {

}

struct Scheduler {
    queue: std::collections::VecDeque<(Event, u64)>,
}

impl Console {
    pub fn schedule(&mut self) {
        use super::ppu::PPUState::*;

        let cpu = &mut self.cpu;
        let ppu = &mut self.ppu;
        let memory = &mut self.memory;
        let timers = &mut self.timers;
        let dma = &mut self.dma;
        let irqcnt = &mut self.irqcnt;

        while ppu.vcount < 226 {
            let ppu_ticks = match ppu.state {
                HDraw => {
                    ppu.hdraw();
                    960
                }
                HBlankStart => {
                    ppu.hblank(irqcnt);
                    dma.request_hblank();
                    0
                }
                HBlank => {
                    ppu.state = EndOfLine;
                    272
                }
                VBlankStart => {
                    ppu.vblank(irqcnt);
                    dma.request_vblank();
                    0
                }
                VBlank => {
                    ppu.state = HBlank;
                    960
                }
                EndOfLine => {
                    ppu.increment_vcount(irqcnt);
                    0
                }
            };

            let mut remaining_ticks = ppu_ticks;
            while remaining_ticks > 0 {
                remaining_ticks -= if dma.is_active() {
                    dma.step(irqcnt, memory)
                } else {
                    cpu.step(memory)
                }
            }

            timers.run(ppu_ticks, irqcnt);
        }

        ppu.increment_vcount(irqcnt);
    }
}
