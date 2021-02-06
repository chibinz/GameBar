//! The reason to separate ALU implementations is because many
//! instructions uses it, in both arm and thumb mode.
//! ALU operations may change CPSR flags but not GPR contents.
//! The result of the operation is passed back as return values.

use crate::barrel_shifter;
use crate::register::PSRBit::*;
use util::*;

// CPSR flag manipulation

#[inline]
pub fn zero(result: u32) -> bool {
    result == 0
}

#[inline]
pub fn negative(result: u32) -> bool {
    result.bit(31)
}

#[inline]
pub fn with_flags(result: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    (result, (negative(result), zero(result), c, v))
}

#[inline]
fn add_carry(op1: u32, op2: u32) -> bool {
    op1.overflowing_add(op2).1
}

#[inline]
fn add_overflow(op1: u32, op2: u32) -> bool {
    (op1 as i32).overflowing_add(op2 as i32).1
}

#[inline]
fn sub_carry(op1: u32, op2: u32) -> bool {
    op1 >= op2
}

#[inline]
fn sub_overflow(op1: u32, op2: u32) -> bool {
    (op1 as i32).overflowing_sub(op2 as i32).1
}

// Logical operations

#[inline]
pub fn and(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(op1 & op2, c, v)
}

#[inline]
pub fn eor(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(op1 ^ op2, c, v)
}

#[inline]
pub fn orr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(op1 | op2, c, v)
}

#[inline]
pub fn bic(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(op1 & !op2, c, v)
}

#[inline]
pub fn mov(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(op2, c, v)
}

#[inline]
pub fn mvn(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(!op2, c, v)
}

#[inline]
pub fn tst(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    and(op1, op2, c, v)
}

#[inline]
pub fn teq(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    eor(op1, op2, c, v)
}

// Arithemetic operations

#[inline]
pub fn add(op1: u32, op2: u32, _c: bool, _v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(
        op1.wrapping_add(op2),
        add_carry(op1, op2),
        add_overflow(op1, op2),
    )
}

#[inline]
pub fn adc(op1: u32, op2: u32, carry: bool, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    let (opc, carry2) = op2.overflowing_add(carry as u32);
    let result = op1.wrapping_add(opc);

    if s {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, add_carry(op1, opc) || carry2);
        cpu.set_cpsr_bit(V, add_overflow(op1, opc));
    }

    result
}

#[inline]
pub fn sub(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    with_flags(
        op1.wrapping_sub(op2),
        sub_carry(op1, op2),
        sub_overflow(op1, op2),
    )
}

#[inline]
pub fn rsb(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    let result = op2.wrapping_sub(op1);

    if s {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, sub_carry(op2, op1));
        cpu.set_cpsr_bit(V, sub_overflow(op2, op1));
    }

    result
}

#[inline]
pub fn sbc(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    let opc = (op2 as u64) - (carry as u64) + 1;
    let result = op1.wrapping_sub(opc as u32);

    if s {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, op1 as u64 >= opc);
        cpu.set_cpsr_bit(V, sub_overflow(op1, opc as u32));
    }

    result
}

#[inline]
pub fn rsc(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    sbc(op2, op1, c, v)
}

#[inline]
pub fn neg(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    rsb(_op1, op2, c, v)
}

#[inline]
pub fn cmp(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    sub(cpu, op1, op2, true)
}

#[inline]
pub fn cmn(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    add(cpu, op1, op2, true)
}

// Multiplication

pub fn mul(op1: u32, op2: u32, c: bool, v: bool) -> (u32, (bool, bool, bool, bool)) {
    let result = op2.wrapping_mul(op1);

    if s {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

// Shift operations (for thumb)

#[inline]
pub fn lsl(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    let result = barrel_shifter::logical_left(op1, op2, false, false).0;

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn lsr(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    let result = barrel_shifter::logical_right(op1, op2, false, false).0;

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn asr(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    let result = barrel_shifter::arithmetic_right(op1, op2, false, false).0;

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn ror(op1: u32, op2: u32) -> (u32, (bool, bool, bool, bool)) {
    let result = barrel_shifter::rotate_right(op1, op2, false, false).0;

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}
