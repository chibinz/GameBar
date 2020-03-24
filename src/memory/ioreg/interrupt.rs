use crate::interrupt::IRQController;
use crate::cpu::CPU;

impl IRQController
{
    #[inline]
    pub fn get_ime(&self) -> u16
    {
        self.ime
    }

    #[inline]
    pub fn set_ime(&mut self, value: u16, cpu: &mut CPU)
    {
        self.ime = value;

        self.check(cpu);
    }

    #[inline]
    pub fn get_ie(&self) -> u16
    {
        self.ie
    }

    #[inline]
    pub fn set_ie(&mut self, value: u16, cpu: &mut CPU)
    {
        self.ie = value;

        self.check(cpu);
    }

    #[inline]
    pub fn get_irf(&self) -> u16
    {
        self.irf
    }

    #[inline]
    pub fn ack_irf(&mut self, value: u16)
    {
        self.irf &= !value;
    }
}