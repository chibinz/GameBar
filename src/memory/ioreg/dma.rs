use crate::util::*;
use crate::dma::DMAChannel;

use super::Memory;

impl Memory
{
    pub fn update_dma(&mut self, dma: &mut DMAChannel)
    {
        self.update_dmasad(dma);
        self.update_dmadad(dma);
        self.update_dmacnt_l(dma);
        self.update_dmacnt_h(dma);
    }
    
    pub fn update_dmasad(&self, dma: &mut DMAChannel)
    {
        let mut addr = self.ioram32(0xb0 + dma.index * 0xc);

        if dma.index == 0
        {
            addr &= 0x07ffffff
        }
        else
        {
            addr &= 0x0fffffff
        }

        dma.src = addr;
    }

    pub fn update_dmadad(&self, dma: &mut DMAChannel)
    {
        let mut addr = self.ioram32(0xb4 + dma.index * 0xc);

        if dma.index == 0
        {
            addr &= 0x07ffffff
        }
        else
        {
            addr &= 0x0fffffff
        }

        dma.dst = addr;
    }

    pub fn update_dmacnt_l(&self, dma: &mut DMAChannel)
    {
        let cnt_l = self.ioram16(0xb8 + dma.index * 0xc);

        dma.count = cnt_l.bits(15, 0);
    }
    
    pub fn update_dmacnt_h(&mut self, dma: &mut DMAChannel)
    {
        let cnt_h = self.ioram16(0xba + dma.index * 0xc);

        dma.dstcnt   = cnt_h.bits(6, 5);
        dma.srccnt   = cnt_h.bits(8, 7);
        dma.repeat_f = cnt_h.bit(9);
        dma.word_f   = cnt_h.bit(10);
        dma.drq_f    = cnt_h.bit(11);
        dma.start    = cnt_h.bits(13, 12);
        dma.irq_f    = cnt_h.bit(14);
        dma.enable   = cnt_h.bit(15);

        // Initiate a DMA if start mode is immediate
        if dma.enable && dma.start == 0b00
        {
            dma.transfer(self);
        }
    }

    pub fn clr_dma(&mut self, index: usize)
    {
        let mut cnt = self.ioram16(0xba + index * 0xc);

        cnt &= !0x8000;

        self.ioram16_s(0xba + index * 0xc, cnt);
    }
}