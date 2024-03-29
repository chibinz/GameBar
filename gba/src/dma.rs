use crate::bus::GbaBus;
use crate::interrupt::Irq::*;
use crate::interrupt::IrqController;
use util::Bus;

pub struct Dma {
    pub channel: Vec<DmaChannel>,
}

#[derive(Clone)]
pub struct DmaChannel {
    pub index: usize, // DMA index 0 - 3
    pub src: u32,     // Source address when read from the bus
    pub dst: u32,     // Destination address
    pub count: u16,   // Number of word / halfword to be copied
    pub control: u16, // DMA control bits

    pub in_src: u32,   // Internal source register
    pub in_dst: u32,   // Internal destinatino register
    pub srcinc: u32,   // Added to in_src after every copy
    pub dstinc: u32,   // Added to dst_src after every copy
    pub in_count: u16, // Keep track of how many words transferred

    transfer: fn(&mut Self, &mut GbaBus),
    pub state: DMAState,
    pub cycles: i32, // Number of cycles consumed for transfer

    pub active: bool,
}

#[derive(Clone, Debug)]
pub enum DMAState {
    Unintialized,
    Transferring,
    Finished,
}

impl Dma {
    pub fn new() -> Self {
        let mut d = Self {
            channel: vec![DmaChannel::new(); 4],
        };

        for i in 0..4 {
            d.channel[i].index = i;
        }

        d
    }

    pub fn step(&mut self, irqcnt: &mut IrqController, bus: &mut GbaBus) -> i32 {
        for c in self.channel.iter_mut() {
            if c.active {
                return c.step(irqcnt, bus);
            }
        }

        // At least one dma channel should be active!
        unreachable!()
    }

    pub fn request_hblank(&mut self) {
        for c in self.channel.iter_mut() {
            if c.start() == 0b10 {
                c.active = true;
            }
        }
    }

    pub fn request_vblank(&mut self) {
        for c in self.channel.iter_mut() {
            if c.start() == 0b01 {
                c.active = true;
            }
        }
    }

    /// Check if any dma channel is ready but being held
    pub fn is_active(&self) -> bool {
        for c in self.channel.iter() {
            if c.active {
                return true;
            }
        }

        false
    }
}

impl DmaChannel {
    pub fn new() -> Self {
        Self {
            index: 0,
            src: 0,
            dst: 0,
            count: 0,
            control: 0,

            in_count: 0,
            in_src: 0,
            in_dst: 0,
            srcinc: 0,
            dstinc: 0,

            transfer: Self::transfer16,
            state: DMAState::Unintialized,
            cycles: 0,

            active: false,
        }
    }

    /// Things to be done before transfer initiates,
    /// e.g. Copy into internal register, calculate increment...
    pub fn setup(&mut self) {
        if !self.active {
            dbg!(self.index);
            return;
        }

        // Start mode 3 not handled
        assert_ne!(self.start(), 3);
        assert!(self.active);
        assert!(self.enable());

        // Copy into internal register
        self.in_src = self.src;
        self.in_dst = self.dst;

        self.srcinc = self.get_increment(self.srccnt());
        self.dstinc = self.get_increment(self.dstcnt());

        self.transfer = if self.word_f() {
            Self::transfer32
        } else {
            Self::transfer16
        };

        self.state = DMAState::Transferring;
    }

    /// Things to be done after transfer completes,
    /// e.g. Source register write back, interrupt...
    pub fn finish(&mut self, irqcnt: &mut IrqController) {
        self.src = self.in_src;
        if self.dstcnt() != 3 {
            self.dst = self.in_dst
        }

        // Clear enable bit if repeat flag not set
        if !self.repeat_f() {
            self.control &= !0x8000
        }

        let irq = [DMA0, DMA1, DMA2, DMA3];

        if self.interrupt_f() {
            util::info!("DMA {} generated interrupt", self.index);
            irqcnt.request(irq[self.index])
        }

        self.in_count = 0;
        self.active = false;

        self.state = DMAState::Unintialized;
    }

    pub fn step(&mut self, irqcnt: &mut IrqController, bus: &mut GbaBus) -> i32 {
        use DMAState::*;

        self.cycles = 0;

        match self.state {
            Unintialized => self.setup(),
            Transferring => (self.transfer)(self, bus),
            Finished => self.finish(irqcnt),
        }

        self.cycles
    }

    pub fn transfer16(&mut self, bus: &mut GbaBus) {
        if self.in_count < self.count {
            bus.store16(self.in_dst as usize, bus.load16(self.in_src as usize));

            // Incrment internal register
            self.in_src = self.in_src.wrapping_add(self.srcinc);
            self.in_dst = self.in_dst.wrapping_add(self.dstinc);

            self.in_count += 1;

            self.cycles += GbaBus::access_timing(self.in_dst as usize, 1);
            self.cycles += GbaBus::access_timing(self.in_src as usize, 1);
        } else {
            self.state = DMAState::Finished;
        }
    }

    pub fn transfer32(&mut self, bus: &mut GbaBus) {
        if self.in_count < self.count {
            bus.store32(self.in_dst as usize, bus.load32(self.in_src as usize));

            // Incrment internal register
            self.in_src = self.in_src.wrapping_add(self.srcinc);
            self.in_dst = self.in_dst.wrapping_add(self.dstinc);

            self.in_count += 1;

            self.cycles += GbaBus::access_timing(self.in_dst as usize, 2);
            self.cycles += GbaBus::access_timing(self.in_src as usize, 2);
        } else {
            self.state = DMAState::Finished;
        }
    }

    pub fn get_increment(&self, cnt: u32) -> u32 {
        let inc: u32 = if self.word_f() { 4 } else { 2 };
        match cnt {
            0b00 => inc,
            0b01 => inc.wrapping_neg(),
            0b10 => 0,
            0b11 => inc,
            _ => unreachable!(),
        }
    }
}
