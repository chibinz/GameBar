//! As x86 uses only the least significant 5 bits of shift amount
//! and Rust explicitly checks integer overflow in debug builds,
//! special handling is needed

use crate::cpu::CPU;
use crate::cpu::register::PSRBit::C;

/// Shift a value according to shift amount and type and return the shifted result.
/// Set carry bits of CPSR accordingly.
#[inline]
#[allow(exceeding_bitshifts)]
pub fn shift(cpu: &mut CPU, operand: u32, amount: u32, stype: u32) -> u32
{
    match stype
    {
        0b00 => logical_left(cpu, operand, amount),
        0b01 => logical_right(cpu, operand, amount),
        0b10 => arithmetic_right(cpu, operand, amount),
        0b11 => rotate_right(cpu, operand, amount),
        _    => panic!("Invalid shift type!"),
    }
}

/// Note that LSL #0 maintains the old CPSR C flag
#[inline]
#[allow(exceeding_bitshifts)]
pub fn logical_left(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{

    if amount == 0
    {
        operand
    }
    else if amount < 32
    {
        let carry = (operand >> (32 - amount)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        operand << amount
    }
    else if amount == 32
    {
        let carry = (operand >> 32) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        0
    }
    else
    {
        cpu.register.set_cpsr_bit(C, false);

        0
    }
}

/// Note that LSR #0 is equivalent to LSR #32
#[inline]
#[allow(exceeding_bitshifts)]
pub fn logical_right(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount == 0 || amount == 32
    {
        let carry = operand >> 31 & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        0
    }
    else if amount < 32
    {
        let carry = (operand >> (amount - 1)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        operand >> amount
    }
    else
    {
        cpu.register.set_cpsr_bit(C, false);

        0
    }
}

/// Note that ASR #0 is equivalent to ASR #32
#[inline]
#[allow(exceeding_bitshifts)]
pub fn arithmetic_right(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount == 0 || amount >= 32
    {
        let carry = operand >> 31 & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);
        
        (operand as i32 >> 31) as u32
    }
    else
    {
        let carry = (operand >> (amount - 1)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        (operand as i32 >> amount) as u32
    }
}

/// Note that ROR #0 is RRX, rotate right extended
#[inline]
#[allow(exceeding_bitshifts)]
pub fn rotate_right(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount == 0
    {
        let c = if cpu.register.get_cpsr_bit(C) {0x80000000} else {0};

        let carry = operand & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);
        
        c | (operand >> 1)
    }
    else
    {
        // Rotate amount larger than 32 is same as their least significant 5 bits
        let carry = (operand >> ((amount - 1) & 0b11111)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        operand.rotate_right(amount)
    }
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn shift_shift()
    {
        let mut cpu = CPU::new();

        assert_eq!(shift(&mut cpu, 3, 63, 0), 0);
    }

    #[test]
    fn shift_logical_left()
    {
        let mut cpu = CPU::new();
        assert_eq!(logical_left(&mut cpu, 3, 31), 0x80000000);
        assert!(cpu.register.get_cpsr_bit(C));

        cpu.register.set_cpsr_bit(C, true);
        assert_eq!(logical_left(&mut cpu, 3, 34), 0);
        assert!(!cpu.register.get_cpsr_bit(C));
    }

    #[test]
    fn shift_logical_right()
    {
        let mut cpu = CPU::new();
        assert_eq!(logical_right(&mut cpu, 0x80000000, 0), 0);
        assert!(cpu.register.get_cpsr_bit(C));

        cpu.register.set_cpsr_bit(C, false);
        assert_eq!(logical_right(&mut cpu, 0x80000000, 63), 0);
        assert!(!cpu.register.get_cpsr_bit(C));
    }

    #[test]
    fn shift_arithmetic_right()
    {
        let mut cpu = CPU::new();
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 31), 0xffffffff);
        assert!(!cpu.register.get_cpsr_bit(C));

        cpu.register.set_cpsr_bit(C, false);
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 0), 0xffffffff);
        assert!(cpu.register.get_cpsr_bit(C));

        cpu.register.set_cpsr_bit(C, false);
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 99), 0xffffffff);
        assert!(cpu.register.get_cpsr_bit(C));
    }

    #[test]
    fn shift_rotate_right()
    {
        let mut cpu = CPU::new();
        cpu.register.set_cpsr_bit(C, true);
        assert_eq!(rotate_right(&mut cpu, 1, 0), 0x80000000);
        assert!(cpu.register.get_cpsr_bit(C));
        
        cpu.register.set_cpsr_bit(C, false);
        assert_eq!(rotate_right(&mut cpu, 0xf0f0f0f0, 4), 0x0f0f0f0f);
        assert!(!cpu.register.get_cpsr_bit(C));

        cpu.register.set_cpsr_bit(C, false);
        assert_eq!(rotate_right(&mut cpu, 0xf0f0f0f0, 64), 0xf0f0f0f0);
        assert!(cpu.register.get_cpsr_bit(C));
    }
}