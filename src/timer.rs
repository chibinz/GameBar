use crate::interrupt::IRQController;
use crate::interrupt::Interrupt;
use crate::interrupt::Interrupt::*;

pub static PRESCALER: [u32; 4] =
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
    pub reload   : u16,     // Initial value on reload
    pub counter  : u16,     // Timer data
    pub modulo   : u32,     // Leftover (< prescaler)
    pub prescaler: u32,     // 1, 64, 256, 1024
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

    pub fn run(&mut self, value: u32, irqcnt: &mut IRQController)
    {
        let mut overflow = false;

        for (i, timer) in self.timer.iter_mut().enumerate()
        {
            let increment = if timer.cascade_f {overflow as u32} else {value};

            overflow = timer.increment_counter(increment);

            if overflow && timer.irq_f {irqcnt.request(IRQ[i])}
        }
    }
}

impl Timer
{
    pub fn new() -> Self
    {
        Self
        {
            reload   : 0,
            counter  : 0,
            prescaler: 0,
            modulo   : 0,
            irq_f    : false,
            cascade_f: false,
            enable   : false,
        }
    }

    pub fn increment_counter(&mut self, ticks: u32) -> bool
    {
        if self.enable
        {
            self.modulo += ticks;

            let increment = (self.modulo / self.prescaler) as u16;
            let (value, overflow) = self.counter.overflowing_add(increment);

            // Reload initial value on overflow
            self.counter = if overflow {self.reload} else {value};
            self.modulo %= self.prescaler;

            overflow
        }
        else
        {
            false
        }
    }
}