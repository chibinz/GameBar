use util::*;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Interrupt {
    VBlank = 1 << 0,
    HBlank = 1 << 1,
    VCount = 1 << 2,
    Timer0 = 1 << 3,
    Timer1 = 1 << 4,
    Timer2 = 1 << 5,
    Timer3 = 1 << 6,
    Serial = 1 << 7,
    DMA0 = 1 << 8,
    DMA1 = 1 << 9,
    DMA2 = 1 << 10,
    DMA3 = 1 << 11,
    Keypad = 1 << 12,
    GamePak = 1 << 13,
}

#[derive(Debug)]
pub struct IrqController {
    pub ime: u16, // Interrupt master enable flag
    pub ie: u16,  // Interrupt enable flag
    pub irf: u16, // Interrupt request flag
}

impl IrqController {
    pub fn new() -> Self {
        Self {
            ime: 0,
            ie: 0,
            irf: 0,
        }
    }

    pub fn pending(&self) -> bool {
        self.ime.bit(0) && self.ie & self.irf != 0
    }

    pub fn request(&mut self, irq: Interrupt) {
        self.irf |= irq as u16;
    }

    pub fn check(&mut self, cpu: &mut cpu::Cpu) {
        if self.pending() {
            util::info!("Hardware interrupt triggered by irqcnt");
            util::info!("{:?}", &self);
            cpu.hardware_interrupt();
        }
    }
}
