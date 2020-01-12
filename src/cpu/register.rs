use crate::util::*;
use crate::cpu::CPU;

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

impl CPU
{
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

    /// Return true if condition is satified
    #[inline]
    pub fn check_condition(&self, condition: u32) -> bool
    {
        use PSRBit::*;

        match condition
        {
            0b0000 =>  self.get_cpsr_bit(Z),      // EQ
            0b0001 => !self.get_cpsr_bit(Z),      // NE
            0b0010 =>  self.get_cpsr_bit(C),      // CS
            0b0011 => !self.get_cpsr_bit(C),      // CC
            0b0100 =>  self.get_cpsr_bit(N),      // MI
            0b0101 => !self.get_cpsr_bit(N),      // PL
            0b0110 =>  self.get_cpsr_bit(V),      // VS
            0b0111 => !self.get_cpsr_bit(V),      // VC
            0b1000 =>  self.get_cpsr_bit(C) && !self.get_cpsr_bit(Z), // HI
            0b1001 => !self.get_cpsr_bit(C) ||  self.get_cpsr_bit(Z), // LS
            0b1010 =>  self.get_cpsr_bit(N) ==  self.get_cpsr_bit(V), // GE
            0b1011 =>  self.get_cpsr_bit(N) !=  self.get_cpsr_bit(V), // LT
            0b1100 => !self.get_cpsr_bit(Z) && (self.get_cpsr_bit(N)
                                                        ==  self.get_cpsr_bit(V)),// GT
            0b1101 =>  self.get_cpsr_bit(Z) || (self.get_cpsr_bit(N)
                                                        !=  self.get_cpsr_bit(V)),// LE
            0b1110 => true,
            _      => panic!("Invalid Condition Field!"),
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
        let mut cpu = CPU::new();
        cpu.cpsr = 0b100000;

        assert_eq!(cpu.get_cpsr_bit(PSRBit::T), true);
    }

    #[test]
    fn set_cpsr_bit()
    {
        let mut cpu = CPU::new();
        cpu.set_cpsr_bit(PSRBit::F, true);

        assert_eq!(cpu.cpsr.bit(6), true);
    }

    #[test]
    fn get_cpsr_mode()
    {
        let mut cpu = CPU::new();
        cpu.cpsr = 0b10000;

        assert!(match cpu.get_cpsr_mode() {PSRMode::User => true, _ => false});
    }

    #[test]
    fn set_cpsr_mode()
    {
        let mut cpu = CPU::new();
        cpu.set_cpsr_mode(PSRMode::System);

        assert_eq!(cpu.cpsr, 0b11111);
    }

    #[test]
    fn check_condition()
    {
        use PSRBit::*;

        let mut cpu = CPU::new();

        cpu.set_cpsr_bit(Z, true);
        assert!(cpu.check_condition(0b0000));

        cpu.set_cpsr_bit(C, true);
        assert!(cpu.check_condition(0b0010));

        cpu.set_cpsr_bit(N, true);
        assert!(cpu.check_condition(0b0100));

        cpu.set_cpsr_bit(V, true);
        assert!(cpu.check_condition(0b0110));

        assert!(cpu.check_condition(0b1010));

        cpu.set_cpsr_bit(Z, false);
        assert!(cpu.check_condition(0b1100));
    }
}