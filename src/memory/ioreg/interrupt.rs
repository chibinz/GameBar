use crate::interrupt::IRQController;

use super::Memory;

impl Memory
{
    pub fn update_ime(&mut self, irqcnt: &mut IRQController)
    {
        irqcnt.ime = self.ioram16(0x208);
    }

    pub fn update_ie(&mut self, irqcnt: &mut IRQController)
    {
        irqcnt.ie = self.ioram16(0x200);
    }

    pub fn update_irf(&mut self, irqcnt: &mut IRQController)
    {
        irqcnt.irf = self.ioram16(0x202);
    }
}