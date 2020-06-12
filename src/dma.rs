use crate::memory::Memory;

pub struct DMA
{
    pub channel: Vec<DMAChannel>
}

#[derive(Clone, Debug)]
pub struct DMAChannel
{
    pub index  : usize,   // DMA index 0 - 3
    pub src    : u32,     // Internal register of current source address
    pub dst    : u32,     // Internal register of current destination address
    pub count  : u16,     // Number of word / halfword to be copied
    pub control: u16,     // DMA control bits

    pub active : bool,
}

impl DMA
{
    pub fn new() -> Self
    {
        let mut d = Self
        {
            channel: vec![DMAChannel::new(); 4],
        };

        for i in 0..4
        {
            d.channel[i].index = i;
        }

        d
    }

    pub fn run(&mut self, cycles: &mut i32, memory: &mut Memory)
    {
        for c in self.channel.iter_mut()
        {
            c.transfer(cycles, memory);
        }
    }

    // pub fn request(&mut self, memory: &mut Memory)
    // {
    //     let mut n = 100000;
    //     for c in self.channel.iter_mut()
    //     {
    //         c.transfer(&mut n, memory);
    //     }
    // }

    /// Check if any dma channel is ready but being held
    pub fn is_active(&self) -> bool
    {
        for c in self.channel.iter()
        {
            if c.active
            {
                return true;
            }
        }

        return false;
    }
}

impl DMAChannel
{
    pub fn new() -> Self
    {
        Self
        {
            index   : 0,
            src     : 0,
            dst     : 0,
            count   : 0,
            control : 0,

            active  : false,
        }
    }

    /// Transfer data and return the number of cycles used
    pub fn transfer(&mut self, cycles: &mut i32, memory: &mut Memory)
    {
        // TODO special start mode
        if self.start() == 3 {return}
        if !self.enable() {return}

        if self.word_f()
        {
            self.transfer32(cycles, memory);
        }
        else
        {
            self.transfer16(cycles, memory);
        };

        // Premature exit
        if *cycles <= 0 {return}

        if !self.repeat_f()
        {
            self.control &= !0x8000;
        }

        self.active = false;
    }

    pub fn transfer16(&mut self, cycles: &mut i32, memory: &mut Memory)
    {
        // Copy into internal register
        let mut src = self.src;
        let mut dst = self.dst;

        let srcinc = Self::get_increment(self.word_f(), self.srccnt());
        let dstinc = Self::get_increment(self.word_f(), self.dstcnt());

        for _ in 0..self.count
        {
            // If allocated time is used up, return early.
            // dma registers will not be written back
            if *cycles <= 0 {return}

            memory.store16(dst, memory.load16(src));

            // Incrment internal register
            src = src.wrapping_add(srcinc);
            dst = dst.wrapping_add(dstinc);

            // TODO improve timing accuracy
            *cycles -= 2;
        }

        // Write back
        self.src = src;
        if self.dstcnt() != 3 {self.dst = dst}
    }

    pub fn transfer32(&mut self, cycles: &mut i32, memory: &mut Memory)
    {
        let mut src = self.src;
        let mut dst = self.dst;

        let srcinc = Self::get_increment(self.word_f(), self.srccnt());
        let dstinc = Self::get_increment(self.word_f(), self.dstcnt());

        for _ in 0..self.count
        {
            if *cycles <= 0 {return}

            memory.store32(dst, memory.load32(src));

            src = src.wrapping_add(srcinc);
            dst = dst.wrapping_add(dstinc);

            *cycles -= 2;
        }

        self.src = src;
        if self.dstcnt() != 3 {self.dst = dst}
    }

    pub fn get_increment(word_f: bool, cnt: u32) -> u32
    {
        let inc: u32 = if word_f {4} else {2};
        match cnt
        {
            0b00 => inc,
            0b01 => inc.wrapping_neg(),
            0b10 => 0,
            0b11 => inc,
            _    => unreachable!(),
        }
    }
}