use crate::memory::Memory;

#[derive(Clone, Debug)]
pub struct DMA
{
    pub index: usize,
    pub src  : u32,
    pub dst  : u32,
    pub count: u32,
    
    pub dstcnt  : u32,
    pub srccnt  : u32,
    pub repeat_f: bool,
    pub word_f  : bool,
    pub drq_f   : bool,
    pub start   : u32,
    pub irq_f   : bool,
    pub enable  : bool,
}

impl DMA
{
    pub fn new() -> Self
    {
        Self
        {
            index   : 0,
            src     : 0,
            dst     : 0,
            count   : 0,
            dstcnt  : 0,
            srccnt  : 0,
            repeat_f: false,
            word_f  : false,
            drq_f   : false,
            start   : 0,
            irq_f   : false,
            enable  : false,
        }
    }

    pub fn transfer(&mut self, memory: &mut Memory)
    {
        if self.enable
        {
            // dbg!(&self);

            let srcinc: u32 = Self::get_increment(self.word_f, self.srccnt);
            let dstinc: u32 = Self::get_increment(self.word_f, self.dstcnt);

            for _ in 0..self.count
            {
                if self.word_f
                {
                    let value = memory.load32(self.src);
                    memory.store32(self.dst, value);
                }
                else
                {
                    let value = memory.load16(self.src);
                    memory.store16(self.dst, value);
                }

                self.src = self.src.wrapping_add(srcinc);
                self.dst = self.dst.wrapping_add(dstinc);
            }

            if !self.repeat_f
            {
                self.enable = false;
                memory.clr_dma(self.index);
            }
        }
    }

    pub fn get_increment(word_f: bool, cnt: u32) -> u32
    {
        let inc: u32 = if word_f {4} else {2};
        match cnt
        {
            0b00 => inc,
            0b01 => inc.wrapping_neg(),
            0b11 => 0,
            _    => unreachable!(),
        }
    }
}