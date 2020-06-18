use crate::interrupt::IRQController;
use crate::interrupt::Interrupt;
use crate::interrupt::Interrupt::*;

pub static PRESCALER: [u16; 4] =
[
    1,
    64,
    256,
    1024,
];

pub static IRQ: [Interrupt; 4] =
[
    Timer0,
    Timer1,
    Timer2,
    Timer3,
];

pub struct Timers
{
    pub timer: Vec<Timer>
}

#[derive(Clone, Debug)]
pub struct Timer
{
    pub control  : u16,     // Raw control bits
    pub reload   : u16,     // Initial value on reload
    pub counter  : u16,     // Timer data
    pub modulo   : u16,     // Leftover (< prescaler)
    pub prescaler: u16,     // 1, 64, 256, 1024
    pub irq_f    : bool,    // Interrupt on overflow
    pub cascade_f: bool,    // Cascade flag
    pub enable   : bool,    // Enable flag
}

impl Timers
{
    pub fn new() -> Self
    {
        Self
        {
            timer: vec![Timer::new(); 4],
        }
    }

    /// There might be some issues when ticks > u16::max_value()
    pub fn run(&mut self, ticks: i32, irqcnt: &mut IRQController)
    {
        let mut times_overflowed = 0;

        for (i, timer) in self.timer.iter_mut().enumerate()
        {
            if timer.enable
            {
                let increment = timer.calculate_increment(ticks as u16, times_overflowed);

                times_overflowed = timer.increment_counter(increment);

                if times_overflowed > 0 && timer.irq_f {irqcnt.request(IRQ[i])}
            }
            else
            {
                times_overflowed = 0;
            }
        }
    }
}

impl Timer
{
    pub fn new() -> Self
    {
        Self
        {
            control  : 0,
            reload   : 0,
            counter  : 0,
            prescaler: 0,
            modulo   : 0,
            irq_f    : false,
            cascade_f: false,
            enable   : false,
        }
    }

    pub fn calculate_increment(&mut self, ticks_past: u16, times_overflowed: u16) -> u16
    {
        if self.cascade_f
        {
            times_overflowed
        }
        else
        {
            self.modulo += ticks_past;

            let quotient = self.modulo / self.prescaler;
            let remainder = self.modulo % self.prescaler;

            self.modulo = remainder;

            quotient
        }
    }

    /// Add increment to the current counter value, return number of times overflowed
    pub fn increment_counter(&mut self, increment: u16) -> u16
    {
        let mut remaining = increment;
        let mut times_overflowed = 0;

        while remaining > 0
        {
            let (value, overflow) = self.counter.overflowing_add(remaining);

            if overflow
            {
                remaining -= u16::max_value() - self.counter;
                times_overflowed += 1;
                self.counter = self.reload;
            }
            else
            {
                remaining = 0;
                self.counter = value;
            }
        }

        times_overflowed
    }
}