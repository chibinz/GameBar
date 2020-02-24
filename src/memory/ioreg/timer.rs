use crate::util::*;
use crate::timer::Timer;
use crate::timer::PRESCALER;

use super::Memory;

impl Memory
{
    pub fn update_tmcnt(&self, timer: &mut Timer)
    {
        let cnt = self.ioram16(0x102 + timer.index * 4);

        timer.prescaler = PRESCALER[cnt.bits(1, 0) as usize];
        timer.cascade_f = cnt.bit(2);
        timer.irq_f     = cnt.bit(6);
        timer.enable    = cnt.bit(7);
    }

    pub fn get_timer_data(&mut self, index: usize) -> *mut u16
    {
        (&mut self.ioram[0x100 + index * 4]) as *mut u8 as *mut u16
    }
}