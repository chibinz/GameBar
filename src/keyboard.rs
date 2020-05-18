use crate::util::*;
use crate::interrupt::IRQController;
use crate::interrupt::Interrupt::*;

use minifb::Key;
use minifb::Window;

pub struct Keypad
{
    pub keyinput: u16,
    pub keycnt  : u16,
}

impl Keypad
{
    pub fn new() -> Self
    {
        Self
        {
            keyinput: 0,
            keycnt  : 0,
        }
    }

    pub fn set_input(&mut self, value: u16, irqcnt: &mut IRQController)
    {
        self.keyinput = value;

        if self.keycnt.bit(14)
        {
            // Lower 10 bits
            let mask = self.keycnt & 0b0000001111111111;

            let irq = if self.keycnt.bit(15)
            {
                self.keyinput & mask == mask // AND
            }
            else
            {
                self.keyinput & mask > 0 // OR
            };

            if irq {irqcnt.request(Keypad)}
        }
    }
}

pub fn input(w: &Window) -> u16
{
    let mut input = 0x3ff;

    if w.is_key_down(Key::Z)        {input &= !(1 << 0);}
    if w.is_key_down(Key::X)        {input &= !(1 << 1);}
    if w.is_key_down(Key::Slash)    {input &= !(1 << 2);}
    if w.is_key_down(Key::Enter)    {input &= !(1 << 3);}
    if w.is_key_down(Key::Right)    {input &= !(1 << 4);}
    if w.is_key_down(Key::Left)     {input &= !(1 << 5);}
    if w.is_key_down(Key::Up)       {input &= !(1 << 6);}
    if w.is_key_down(Key::Down)     {input &= !(1 << 7);}
    if w.is_key_down(Key::A)        {input &= !(1 << 8);}
    if w.is_key_down(Key::S)        {input &= !(1 << 9);}

    input
}