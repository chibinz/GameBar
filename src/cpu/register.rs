/// Structure containing 16 general pupose registers,
/// 1 Current Program Status Register,
/// and 5 Saved Program Status Register
pub struct Register
{
    pub r: [u32; 16],

    cpsr : u32,
    spsr : [u32; 5],
}

/// Bits 31 - 28, 7 - 5 of Current Program Status Register
pub enum PSRBit
{
    N = 31,     // Sign Flag
    Z = 30,     // Zero Flag
    C = 29,     // Carry Flag
    V = 28,     // Overflow Flag
    // Bits 27 - 8 are reserved

    I = 7,      // IRQ Disable
    F = 6,      // FIQ Disable
    T = 5,      // State Bit, Thumb/Arm
    // Bits 4 - 0 are mode bits
}

/// Operating Mode
pub enum PSRMode
{
    User       = 0b10000,
    FIQ        = 0b10001,
    IRQ        = 0b10010,
    Supervisor = 0b10011,
    Abort      = 0b10111,
    Undefined  = 0b11011,
    System     = 0b11111,
}

impl Register
{
    pub fn new() -> Self
    {
        Self
        {
            r   : [0; 16],
            cpsr: 0,
            spsr: [0; 5],
        }
    }

    #[inline(always)]
    pub fn get_cpsr_bit(&self, bit: PSRBit) -> bool
    {
        self.cpsr >> (bit as u32) & 1 == 1
    }

    #[inline(always)]
    pub fn set_cpsr_bit(&mut self, bit: PSRBit, t: bool)
    {
        if t
        {
            self.cpsr |= 1 << (bit as u32)
        }
        else
        {
            self.cpsr &= !(1 << (bit as u32))
        }
    }

    #[inline(always)]
    pub fn get_cpsr_mode(&self) -> PSRMode
    {
        match self.cpsr
        {
            0b10000 => PSRMode::User,
            0b10001 => PSRMode::FIQ,
            0b10010 => PSRMode::IRQ,
            0b10011 => PSRMode::Supervisor,
            0b10111 => PSRMode::Abort,
            0b11011 => PSRMode::Undefined,
            0b11111 => PSRMode::System, 
            _       => panic!("Invalid PSR Mode\n")
        }
    }

    #[inline(always)]
    pub fn set_cpsr_mode(&mut self, m: PSRMode)
    {
        // Clear bits 4 - 0
        self.cpsr &= !0b11111;
        self.cpsr |= m as u32;
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn get_cpsr_bit()
    {
        let mut reg = Register::new();
        reg.cpsr = 0b100000;

        assert_eq!(reg.get_cpsr_bit(PSRBit::T), true);
    }

    #[test]
    fn set_cpsr_bit()
    {
        let mut reg = Register::new();
        reg.set_cpsr_bit(PSRBit::F, true);

        assert_eq!(reg.cpsr, 0b1000000);
    }

    #[test]
    fn get_cpsr_mode()
    {
        let mut reg = Register::new();
        reg.cpsr = 0b10000;

        assert!(match reg.get_cpsr_mode() {PSRMode::User => true, _ => false});
    }

    #[test]
    fn set_cpsr_mode()
    {
        let mut reg = Register::new();
        reg.set_cpsr_mode(PSRMode::System);

        assert_eq!(reg.cpsr, 0b11111);
    }
}