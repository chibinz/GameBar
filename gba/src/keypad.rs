use crate::interrupt::Irq::*;
use crate::interrupt::IrqController;
use util::*;

pub struct Keypad {
    pub keyinput: u16,
    pub keycnt: u16,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keyinput: 0,
            keycnt: 0,
        }
    }

    pub fn set_input(&mut self, value: u16, irqcnt: &mut IrqController) {
        self.keyinput = value;

        if self.keycnt.bit(14) {
            // Lower 10 bits
            let mask = self.keycnt & 0b0000001111111111;

            let irq = if self.keycnt.bit(15) {
                self.keyinput & mask == mask // AND
            } else {
                self.keyinput & mask != 0 // OR
            };

            if irq {
                irqcnt.request(Keypad)
            }
        }
    }
}
