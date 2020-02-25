use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[derive(Clone, Copy)]
pub enum Interrupt
{
    VBlank  = 0,
    HBlank  = 1,
    VCount  = 2,
    Timer0  = 3,
    Timer1  = 4,
    Timer2  = 5,
    Timer3  = 6,
    Serial  = 7,
    DMA0    = 8,
    DMA1    = 9,
    DMA2    = 10,
    DMA3    = 11,
    Keypad  = 12,
    GamePak = 13,
}

#[derive(Debug)]
pub struct IRQController
{
    pub ime: u16,   // Interrupt master enable flag
    pub ie : u16,   // Interrupt enable flag
    pub irf: u16,   // Interrupt request flag
}

impl IRQController
{
    pub fn new() -> Self
    {
        Self
        {
            ime: 0,
            ie : 0,
            irf: 0,
        }
    }

    pub fn request(&mut self, irq: Interrupt, cpu: &mut CPU, memory: &mut Memory)
    {
        if self.ime.bit(0)
        {
            if self.ie.bit(irq as u32)
            {
                self.acknowledge(irq, memory);
                cpu.hardware_interrupt();
            }
        }
    }

    pub fn acknowledge(&mut self, irq: Interrupt, memory: &mut Memory)
    {
        self.irf |= 1 << (irq as u32);

        memory.store16(0x04000202, self.irf);
    }
}
