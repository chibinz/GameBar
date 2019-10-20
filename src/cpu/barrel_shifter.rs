use crate::cpu::CPU;
use crate::cpu::register::PSRBit::C;

/// Shift a value according to shift amount and type and return the carry bit.
/// Set carry bits of CPSR accordingly
#[inline]
pub fn shift(cpu: &mut CPU, operand: u32, amount: u32, stype: u32) -> u32
{
    let samount = amount % 64;

    match stype
    {
        0b00 => logical_left(cpu, operand, samount),
        0b01 => logical_right(cpu, operand, samount),
        0b10 => arithmetic_right(cpu, operand, samount),
        0b11 => rotate_right(cpu, operand, samount),
        _    => panic!("Invalid shift type!"),
    }
}

/// Note that LSL #0 maintains the old CPSR C flag
#[inline]
#[allow(exceeding_bitshifts)]
pub fn logical_left(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount > 0
    {
        let carry = (operand >> (32 - amount)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);
    }

    // `operand << amount` strangely compiles to rotate left
    ((operand as u64) << amount) as u32
}

/// Note that LSR #0 is equivalent to LSR #32
#[inline]
#[allow(exceeding_bitshifts)]
pub fn logical_right(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount == 0
    {
        let carry = operand >> 31 & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        0
    }
    else
    {
        let carry = (operand >> (amount - 1)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        // `operand >> amount` strangely compiles to rotate right
        ((((operand as u64) << amount) & 0xffffffff00000000) >> 32) as u32
    }
}

/// Note that ASR #0 is equivalent to ASR #32
#[inline]
#[allow(exceeding_bitshifts)]
pub fn arithmetic_right(cpu: &mut CPU, operand: u32, amount: u32) -> u32
{
    if amount == 0
    {
        let carry = operand >> 31 & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);
        
        // assert_eq!(0x80000000u32 as i32 >> 32, -1i32);
        // Rust perform arithemetic shift right on signed integers
        ((operand as i32) / (32 << 1)) as u32
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
        let carry = (operand >> (amount - 1)) & 1 == 1;
        cpu.register.set_cpsr_bit(C, carry);

        operand.rotate_right(amount)
    }
}


#[cfg(test)]
mod test
{
    use super::*;
    // Part of the test are commented out due to Rust explicitly
    // checks for integer overflow in debug builds.

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
        assert_eq!(rotate_right(&mut cpu, 0xf0f0f0f0, 8), 0xf0f0f0f0);
        assert!(cpu.register.get_cpsr_bit(C));
    }
}