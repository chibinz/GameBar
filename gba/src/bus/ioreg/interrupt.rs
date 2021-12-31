use crate::interrupt::IrqController;

impl IrqController {
    #[inline]
    pub fn get_ime(&self) -> u16 {
        self.ime
    }

    #[inline]
    pub fn set_ime(&mut self, value: u16) {
        self.ime = value;
    }

    #[inline]
    pub fn get_ie(&self) -> u16 {
        self.ie
    }

    #[inline]
    pub fn set_ie(&mut self, value: u16) {
        self.ie = value;
    }

    #[inline]
    pub fn get_irf(&self) -> u16 {
        self.irf
    }

    #[inline]
    pub fn ack_irf(&mut self, value: u16) {
        util::info!("{:b}", value);
        self.irf &= !value;
    }
}
