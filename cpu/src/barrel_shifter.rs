//! As x86 uses only the least significant 5 bits of shift amount
//! and Rust explicitly checks integer overflow in debug builds,
//! special handling is needed

use util::*;

/// Perform shift on a register, return shifted result.
/// Note that this function may change the C flag of CPSR.
#[inline]
pub fn shift_register(cpu: &crate::CPU, operand2: u32) -> (u32, bool) {
    let rm = operand2.bits(3, 0);
    let stype = operand2.bits(6, 5);
    // Register specified shift operates differently when shift amount
    // equals to zero, thus a flag is needed.
    let r = operand2.bit(4);
    let amount = if r {
        let rs = operand2.bits(11, 8);

        debug_assert_ne!(rs, 15);
        debug_assert_eq!(operand2.bit(7), false);

        cpu.r[rs as usize] & 0xff
    } else {
        operand2.bits(11, 7)
    };

    // One internal cycle for register specified shift

    shift(cpu.r[rm as usize], amount, stype, cpu.carry(), !r)
}

/// Perform rotate on an immediate, return rotated result.
/// While not listed on the data sheet, immediate operand rotates
/// do manipulate CPSR flags.
#[inline]
pub fn rotate_immediate(operand2: u32, carry: bool) -> (u32, bool) {
    let amount = operand2.bits(11, 8) * 2;
    let immediate = operand2.bits(7, 0);
    let carry = if amount == 0 {
        carry
    } else {
        immediate.bit(amount - 1)
    };

    (immediate.rotate_right(amount), carry)
}

/// Shift a value according to shift amount and type and return the shifted result.
/// Set carry bits of CPSR accordingly.
#[inline]
pub fn shift(operand: u32, amount: u32, stype: u32, carry: bool, i: bool) -> (u32, bool) {
    match stype {
        0b00 => logical_left(operand, amount, carry, i),
        0b01 => logical_right(operand, amount, carry, i),
        0b10 => arithmetic_right(operand, amount, carry, i),
        0b11 => rotate_right(operand, amount, carry, i),
        _ => unreachable!("Invalid shift type!"),
    }
}

/// LSL #0 maintains the old CPSR C flag
#[inline]
pub fn logical_left(operand: u32, amount: u32, carry: bool, _i: bool) -> (u32, bool) {
    if amount == 0 {
        (operand, carry)
    } else if amount < 32 {
        (operand << amount, operand.bit(32 - amount))
    } else if amount == 32 {
        (0, operand.bit(0))
    } else {
        (0, false)
    }
}

/// Note that LSR #0 for immediate shift is equivalent to LSR #32
#[inline]
pub fn logical_right(operand: u32, amount: u32, carry: bool, i: bool) -> (u32, bool) {
    if amount == 0 {
        if i {
            (0, operand.bit(31))
        } else {
            (operand, carry)
        }
    } else if amount < 32 {
        (operand >> amount, operand.bit(amount - 1))
    } else if amount == 32 {
        (0, operand.bit(31))
    } else {
        (0, false)
    }
}

/// Note that ASR #0 for immediate shift is equivalent to ASR #32
#[inline]
pub fn arithmetic_right(operand: u32, amount: u32, carry: bool, i: bool) -> (u32, bool) {
    if amount == 0 {
        if i {
            ((operand as i32 >> 31) as u32, operand.bit(31))
        } else {
            (operand, carry)
        }
    } else if amount >= 32 {
        ((operand as i32 >> 31) as u32, operand.bit(31))
    } else {
        ((operand as i32 >> amount) as u32, operand.bit(amount - 1))
    }
}

/// Note that ROR #0 for immediate shift is RRX, rotate right extended
#[inline]
pub fn rotate_right(operand: u32, amount: u32, carry: bool, i: bool) -> (u32, bool) {
    if amount == 0 {
        if i {
            (
                (carry as u32).rotate_right(1) | (operand >> 1),
                operand.bit(0),
            )
        } else {
            (operand, carry)
        }
    } else {
        // Rotate amount larger than 31 is same as their least significant 5 bits
        (
            operand.rotate_right(amount),
            operand.bit((amount - 1) & 0x1f),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_register() {
        let mut cpu = crate::CPU::new();

        let mut operand2;

        // ASR 32
        operand2 = 0b11111_10_0_0100;
        cpu.r[4] = 0x80000000;
        assert_eq!(shift_register(&cpu, operand2), (0xffffffff, false));

        // LSR 32
        operand2 = 0b0001_0_01_1_0100;
        cpu.r[1] = 32;
        cpu.r[4] = 0x80000000;
        assert_eq!(shift_register(&cpu, operand2), (0, true));
    }

    #[test]
    fn test_rotate_immediate() {
        let operand2 = 0b0001_00000010;

        assert_eq!(rotate_immediate(operand2, false), (0x80000000, true));
    }

    #[test]
    fn shift_logical_left() {
        assert_eq!(logical_left(0b11, 31, false, true), (0x80000000, true));
        assert_eq!(logical_left(0b11, 34, true, true), (0, false));
    }

    #[test]
    fn shift_logical_right() {
        assert_eq!(logical_right(0x80000000, 0, false, true), (0, true));
        assert_eq!(logical_right(0x80000000, 63, false, true), (0, false));
    }

    #[test]
    fn shift_arithmetic_right() {
        assert_eq!(
            arithmetic_right(0x80000000, 31, false, true),
            (0xffffffff, false)
        );
        assert_eq!(
            arithmetic_right(0x80000000, 0, false, true),
            (0xffffffff, true)
        );
        assert_eq!(
            arithmetic_right(0x80000000, 99, false, true),
            (0xffffffff, true)
        );
    }

    #[test]
    fn shift_rotate_right() {
        assert_eq!(rotate_right(1, 0, true, true), (0x80000000, true)); // RRX
        assert_eq!(
            rotate_right(0xf0f0f0f0, 4, false, true),
            (0x0f0f0f0f, false)
        );
        assert_eq!(
            rotate_right(0xf0f0f0f0, 64, false, true),
            (0xf0f0f0f0, true)
        );
    }
}
