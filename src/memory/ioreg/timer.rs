use crate::util::*;
use crate::timer::Timer;
use crate::timer::PRESCALER;

impl Timer
{
    #[inline]
    pub fn get_counter(&self) -> u16
    {
        self.counter
    }

    #[inline]
    pub fn set_reload(&mut self, value: u16)
    {
        self.reload = value;
    }

    #[inline]
    pub fn set_control(&mut self, value: u16)
    {
        if !self.enable && value.bit(7) {self.counter = self.reload}

        self.prescaler = PRESCALER[value.bits(1, 0) as usize];
        self.cascade_f = value.bit(2);
        self.irq_f     = value.bit(6);
        self.enable    = value.bit(7); // Reload on switching on timer
    }
}