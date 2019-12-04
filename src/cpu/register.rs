use crate::util::*;

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
            cpsr: 0b10011, // On reset, the CPSR is forced to supervisor mode
            spsr: [0; 5],
        }
    }

    #[inline]
    pub fn get_cpsr(&self) -> u32
    {
        self.cpsr
    }

    /// Set defined bits of CPSR.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of CPSR are kept intact.
    #[inline]
    pub fn set_cpsr(&mut self, r: u32, f: bool)
    {
        let mask = if f {0xf0000000} else {0xf00000ff};
        
        self.cpsr &= !mask;
        self.cpsr |= r & mask;
    }

    /// Copy SPSR of current mode to CPSR
    #[inline]
    pub fn restore_cpsr(&mut self)
    {
        self.cpsr = self.spsr[self.get_spsr_index()];
    }

    #[inline]
    pub fn get_cpsr_bit(&self, bit: PSRBit) -> bool
    {
        self.cpsr >> (bit as u32) & 1 == 1
    }

    #[inline]
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

    #[inline]
    pub fn get_cpsr_mode(&self) -> PSRMode
    {
        use PSRMode::*;

        match self.cpsr & 0b11111
        {
            0b10000 => User,
            0b10001 => FIQ,
            0b10010 => IRQ,
            0b10011 => Supervisor,
            0b10111 => Abort,
            0b11011 => Undefined,
            0b11111 => System, 
            _       => panic!("Invalid PSR Mode\n")
        }
    }

    #[inline]
    pub fn set_cpsr_mode(&mut self, m: PSRMode)
    {
        // Clear bits 4 - 0
        self.cpsr &= !0b11111;
        self.cpsr |= m as u32;
    }

    /// Get SPSR of current mode
    #[inline]
    pub fn get_spsr(&mut self) -> u32
    {
        self.spsr[self.get_spsr_index()]
    }

    /// Set defined bits of SPSR of current mode.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of SPSR are kept intact.
    #[inline]
    pub fn set_spsr(&mut self, r: u32, f: bool)
    {
        let mask = if f {0xf0000000} else {0xf00000ff};
        
        self.spsr[self.get_spsr_index()] &= !mask;
        self.spsr[self.get_spsr_index()] |= r & mask;
    }    
    
    /// Because SPSR are banked and stored as array,
    /// get the index of current SPSR.
    #[inline]
    fn get_spsr_index(&self) -> usize
    {
        use PSRMode::*;

        match self.get_cpsr_mode()
        {
            FIQ        => 0,
            Supervisor => 1,
            Abort      => 2,
            IRQ        => 3,
            Undefined  => 4,
            _          => panic!("Curent mode does not have a SPSR"),
        }
    }
}

#[cfg(test)]
mod tests
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

        assert_eq!(bit(reg.cpsr, 6), true);
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