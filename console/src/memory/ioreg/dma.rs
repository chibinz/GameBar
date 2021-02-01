use crate::dma::DMAChannel;
use crate::util::*;

impl DMAChannel {
    #[inline]
    pub fn get_src_l(&self) -> u16 {
        self.src as u16
    }

    #[inline]
    pub fn set_src_l(&mut self, value: u16) {
        self.src &= 0xffff0000;
        self.src |= value as u32;
    }

    #[inline]
    pub fn get_src_h(&self) -> u16 {
        (self.src >> 16) as u16
    }

    #[inline]
    pub fn set_src_h(&mut self, value: u16) {
        self.src &= 0x0000ffff;
        self.src |= (value as u32) << 16;
    }

    #[inline]
    pub fn get_dst_l(&self) -> u16 {
        self.dst as u16
    }

    #[inline]
    pub fn set_dst_l(&mut self, value: u16) {
        self.dst &= 0xffff0000;
        self.dst |= value as u32;
    }

    #[inline]
    pub fn get_dst_h(&self) -> u16 {
        (self.dst >> 16) as u16
    }

    #[inline]
    pub fn set_dst_h(&mut self, value: u16) {
        self.dst &= 0x0000ffff;
        self.dst |= (value as u32) << 16;
    }

    #[inline]
    pub fn get_count(&self) -> u16 {
        self.count
    }

    #[inline]
    pub fn set_count(&mut self, value: u16) {
        self.count = value;
    }

    #[inline]
    pub fn get_control(&self) -> u16 {
        self.control
    }

    #[inline]
    pub fn set_control(&mut self, value: u16) {
        self.control = value;

        // Initiate a DMA if start mode is immediate
        self.active = self.enable() && self.start() == 0;
    }

    #[inline]
    pub fn dstcnt(&self) -> u32 {
        self.control.bits(6, 5)
    }

    #[inline]
    pub fn srccnt(&self) -> u32 {
        self.control.bits(8, 7)
    }

    #[inline]
    pub fn repeat_f(&self) -> bool {
        self.control.bit(9)
    }

    #[inline]
    pub fn word_f(&self) -> bool {
        self.control.bit(10)
    }

    #[inline]
    pub fn start(&self) -> u32 {
        self.control.bits(13, 12)
    }

    #[inline]
    pub fn interrupt_f(&self) -> bool {
        self.control.bit(14)
    }

    #[inline]
    pub fn enable(&self) -> bool {
        self.control.bit(15)
    }
}
