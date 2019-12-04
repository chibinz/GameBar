pub mod register;
pub mod arm;
pub mod thumb;
pub mod barrel_shifter;

use std::fmt;
use register::Register;

pub struct CPU
{
    pub register: Register,
}

impl CPU
{
    pub fn new() -> Self
    {
        Self
        {
            register: Register::new(),
        }
    }

    /// Return true if condition is satified
    #[inline(always)]
    fn check_condition(&self, condition: u32) -> bool
    {
        use register::PSRBit::*;

        match condition
        {
            0b0000 =>  self.register.get_cpsr_bit(Z),      // EQ
            0b0001 => !self.register.get_cpsr_bit(Z),      // NE
            0b0010 =>  self.register.get_cpsr_bit(C),      // CS
            0b0011 => !self.register.get_cpsr_bit(C),      // CC
            0b0100 =>  self.register.get_cpsr_bit(N),      // MI
            0b0101 => !self.register.get_cpsr_bit(N),      // PL
            0b0110 =>  self.register.get_cpsr_bit(V),      // VS
            0b0111 => !self.register.get_cpsr_bit(V),      // VC
            0b1000 =>  self.register.get_cpsr_bit(C) && !self.register.get_cpsr_bit(Z), // HI
            0b1001 => !self.register.get_cpsr_bit(C) ||  self.register.get_cpsr_bit(Z), // LS
            0b1010 =>  self.register.get_cpsr_bit(N) ==  self.register.get_cpsr_bit(V), // GE
            0b1011 =>  self.register.get_cpsr_bit(N) !=  self.register.get_cpsr_bit(V), // LT
            0b1100 => !self.register.get_cpsr_bit(Z) && (self.register.get_cpsr_bit(N)
                                                     ==  self.register.get_cpsr_bit(V)),// GT
            0b1101 =>  self.register.get_cpsr_bit(Z) || (self.register.get_cpsr_bit(N)
                                                     !=  self.register.get_cpsr_bit(V)),// LE
            0b1110 => true,
            _      => panic!("Invalid Condition Field!"),
        }
    }
}

impl fmt::Display for CPU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for i in 0..16
        {
            if i % 4 == 0 && i > 0 {print!("\n");}

            print!("R{:<2} = {:08x} ", i, self.register.r[i as usize]);
        }

        write!(f, "\nCPSR = {:032b}", self.register.get_cpsr())
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn check_condition()
    {
        use register::PSRBit::*;

        let mut cpu = CPU::new();

        cpu.register.set_cpsr_bit(Z, true);
        assert!(cpu.check_condition(0b0000));

        cpu.register.set_cpsr_bit(C, true);
        assert!(cpu.check_condition(0b0010));

        cpu.register.set_cpsr_bit(N, true);
        assert!(cpu.check_condition(0b0100));

        cpu.register.set_cpsr_bit(V, true);
        assert!(cpu.check_condition(0b0110));

        assert!(cpu.check_condition(0b1010));

        cpu.register.set_cpsr_bit(Z, false);
        assert!(cpu.check_condition(0b1100));
    }
}