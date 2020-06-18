//! As x86 uses only the least significant 5 bits of shift amount
//! and Rust explicitly checks integer overflow in debug builds,
//! special handling is needed

use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::C;

/// Perform shift on a register, return shifted result.
/// Note that this function may change the C flag of CPSR.
#[inline]
pub fn shift_register(cpu: &mut CPU, operand2: u32) -> u32
{
    let rm = operand2.bits(3, 0);
    let stype = operand2.bits(6, 5);
    // Register specified shift operates differently when shift amount
    // equals to zero, thus a flag is needed.
    let r = operand2.bit(4);
    let amount =
    if r
    {
        let rs = operand2.bits(11, 8);

        debug_assert_ne!(rs, 15);
        debug_assert_eq!(operand2.bit(7), false);

        cpu.r[rs as usize] & 0xff
    }
    else
    {
        operand2.bits(11, 7)
    };

    // One internal cycle for register specified shift
    cpu.cycles += 1;

    shift(cpu, cpu.r[rm as usize], amount, stype, !r)
}

/// Perform rotate on an immediate, return rotated result.
/// While not listed on the data sheet, immediate operand rotates
/// do manipulate CPSR flags.
#[inline]
pub fn rotate_immediate(cpu: &mut CPU, operand2: u32) -> u32
{
    let amount = operand2.bits(11, 8) * 2;
    let immediate = operand2.bits(7, 0);

    if amount == 0
    {
        immediate
    }
    else
    {
        let carry = (immediate >> ((amount - 1) & 0b11111)) & 1 == 1;
        cpu.set_cpsr_bit(C, carry);

        immediate.rotate_right(amount)
    }
}

/// Shift a value according to shift amount and type and return the shifted result.
/// Set carry bits of CPSR accordingly.
#[inline]
pub fn shift(cpu: &mut CPU, operand: u32, amount: u32, stype: u32, i: bool) -> u32
{
    match stype
    {
        0b00 => logical_left(cpu, operand, amount, i),
        0b01 => logical_right(cpu, operand, amount, i),
        0b10 => arithmetic_right(cpu, operand, amount, i),
        0b11 => rotate_right(cpu, operand, amount, i),
        _    => unreachable!("Invalid shift type!"),
    }
}

/// LSL #0 maintains the old CPSR C flag
#[inline]
pub fn logical_left(cpu: &mut CPU, operand: u32, amount: u32, _i: bool) -> u32
{

    if amount == 0
    {
        operand
    }
    else if amount < 32
    {
        let carry = operand.bit(32 - amount);
        cpu.set_cpsr_bit(C, carry);

        operand << amount
    }
    else if amount == 32
    {
        let carry = operand & 1 == 1;
        cpu.set_cpsr_bit(C, carry);

        0
    }
    else
    {
        cpu.set_cpsr_bit(C, false);

        0
    }
}

/// Note that LSR #0 for immediate shift is equivalent to LSR #32
#[inline]
pub fn logical_right(cpu: &mut CPU, operand: u32, amount: u32, i: bool) -> u32
{
    if amount == 0
    {
        if i
        {
            let carry = operand.bit(31);
            cpu.set_cpsr_bit(C, carry);

            0
        }
        else
        {
            operand
        }
    }
    else if amount < 32
    {
        let carry = operand.bit(amount - 1);
        cpu.set_cpsr_bit(C, carry);

        operand >> amount
    }
    else if amount == 32
    {
        let carry = operand.bit(31);
        cpu.set_cpsr_bit(C, carry);

        0
    }
    else
    {
        cpu.set_cpsr_bit(C, false);

        0
    }
}

/// Note that ASR #0 for immediate shift is equivalent to ASR #32
#[inline]
pub fn arithmetic_right(cpu: &mut CPU, operand: u32, amount: u32, i: bool) -> u32
{
    if amount == 0
    {
        if i
        {
            let carry = operand.bit(31);
            cpu.set_cpsr_bit(C, carry);

            (operand as i32 >> 31) as u32
        }
        else
        {
            operand
        }
    }
    else if amount >= 32
    {
        let carry = operand.bit(31);
        cpu.set_cpsr_bit(C, carry);

        (operand as i32 >> 31) as u32
    }
    else
    {
        let carry = operand.bit(amount - 1);
        cpu.set_cpsr_bit(C, carry);

        (operand as i32 >> amount) as u32
    }
}

/// Note that ROR #0 for immediate shift is RRX, rotate right extended
#[inline]
pub fn rotate_right(cpu: &mut CPU, operand: u32, amount: u32, i: bool) -> u32
{
    if amount == 0
    {
        if i
        {
            let c = (cpu.get_cpsr_bit(C) as u32) << 31;

            let carry = operand.bit(0);
            cpu.set_cpsr_bit(C, carry);

            c | (operand >> 1)
        }
        else
        {
            operand
        }
    }
    else
    {
        // Rotate amount larger than 32 is same as their least significant 5 bits
        let carry = (operand >> ((amount - 1) & 0b11111)) & 1 == 1;
        cpu.set_cpsr_bit(C, carry);

        operand.rotate_right(amount)
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_shift_register()
    {
        let mut cpu = CPU::new();

        let mut operand2;

        // ASR 32
        operand2 = 0b11111_10_0_0100;
        cpu.r[4] = 0x80000000;
        assert_eq!(shift_register(&mut cpu, operand2), 0xffffffff);
        assert_eq!(cpu.get_cpsr_bit(C), false);

        // LSR 32
        operand2 = 0b0001_0_01_1_0100;
        cpu.r[1] = 32;
        cpu.r[4] = 0x80000000;
        assert_eq!(shift_register(&mut cpu, operand2), 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }

    #[test]
    fn test_rotate_immediate()
    {
        let mut cpu = CPU::new();
        let operand2 = 0b0001_00000010;

        assert_eq!(rotate_immediate(&mut cpu, operand2), 0x80000000);
    }

    #[test]
    fn shift_logical_left()
    {
        let mut cpu = CPU::new();
        assert_eq!(logical_left(&mut cpu, 3, 31, true), 0x80000000);
        assert!(cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, true);
        assert_eq!(logical_left(&mut cpu, 3, 34, true), 0);
        assert!(!cpu.get_cpsr_bit(C));
    }

    #[test]
    fn shift_logical_right()
    {
        let mut cpu = CPU::new();
        assert_eq!(logical_right(&mut cpu, 0x80000000, 0, true), 0);
        assert!(cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, false);
        assert_eq!(logical_right(&mut cpu, 0x80000000, 63, true), 0);
        assert!(!cpu.get_cpsr_bit(C));
    }

    #[test]
    fn shift_arithmetic_right()
    {
        let mut cpu = CPU::new();
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 31, true), 0xffffffff);
        assert!(!cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, false);
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 0, true), 0xffffffff);
        assert!(cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, false);
        assert_eq!(arithmetic_right(&mut cpu, 0x80000000, 99, true), 0xffffffff);
        assert!(cpu.get_cpsr_bit(C));
    }

    #[test]
    fn shift_rotate_right()
    {
        let mut cpu = CPU::new();
        cpu.set_cpsr_bit(C, true);
        assert_eq!(rotate_right(&mut cpu, 1, 0, true), 0x80000000);
        assert!(cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, false);
        assert_eq!(rotate_right(&mut cpu, 0xf0f0f0f0, 4, true), 0x0f0f0f0f);
        assert!(!cpu.get_cpsr_bit(C));

        cpu.set_cpsr_bit(C, false);
        assert_eq!(rotate_right(&mut cpu, 0xf0f0f0f0, 64, true), 0xf0f0f0f0);
        assert!(cpu.get_cpsr_bit(C));
    }
}