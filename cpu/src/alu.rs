//! The reason to separate ALU implementations is because many
//! instructions uses it, in both arm and thumb mode.
//! ALU operations may change CPSR flags but not GPR contents.
//! The result of the operation is passed back as return values.

use crate::barrel_shifter;
use crate::register::PSRBit::*;
use util::*;

type Flags = (bool, bool, bool, bool);

// CPSR flag manipulation
pub fn set_flags(cpu: &mut crate::CPU, (n, z, c, v): Flags) {
    cpu.set_cpsr_bit(N, n);
    cpu.set_cpsr_bit(Z, z);
    cpu.set_cpsr_bit(C, c);
    cpu.set_cpsr_bit(V, v);
}

pub fn get_cv(cpu: &crate::CPU) -> (bool, bool) {
    (cpu.get_cpsr_bit(C), cpu.get_cpsr_bit(V))
}

#[inline]
pub fn with_flags(result: u32, c: bool, v: bool) -> (u32, Flags) {
    (result, (result.bit(31), result == 0, c, v))
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
fn sub_carry<T: PartialOrd>(op1: T, op2: T) -> bool {
    op1 >= op2
}

#[inline]
fn sub_overflow(op1: u32, op2: u32) -> bool {
    (op1 as i32).overflowing_sub(op2 as i32).1
}

// Logical operations

#[inline]
pub fn and(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op1 & op2, c, v)
}

#[inline]
pub fn eor(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op1 ^ op2, c, v)
}

#[inline]
pub fn orr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op1 | op2, c, v)
}

#[inline]
pub fn bic(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op1 & !op2, c, v)
}

#[inline]
pub fn mov(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op2, c, v)
}

#[inline]
pub fn mvn(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(!op2, c, v)
}

#[inline]
pub fn tst(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    and(op1, op2, c, v)
}

#[inline]
pub fn teq(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    eor(op1, op2, c, v)
}

// Arithemetic operations

#[inline]
pub fn add(op1: u32, op2: u32, _c: bool, _v: bool) -> (u32, Flags) {
    with_flags(
        op1.wrapping_add(op2),
        add_carry(op1, op2),
        add_overflow(op1, op2),
    )
}

#[inline]
pub fn adc(op1: u32, op2: u32, c: bool, _v: bool) -> (u32, Flags) {
    let (opc, carry2) = op2.overflowing_add(c as u32);
    let result = op1.wrapping_add(opc);

    with_flags(
        result,
        add_carry(op1, opc) || carry2,
        ((!op1 ^ op2) & (op1 ^ result)).bit(31),
    )
}

#[inline]
pub fn sub(op1: u32, op2: u32, _c: bool, _v: bool) -> (u32, Flags) {
    with_flags(
        op1.wrapping_sub(op2),
        sub_carry(op1, op2),
        sub_overflow(op1, op2),
    )
}

#[inline]
pub fn rsb(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    sub(op2, op1, c, v)
}

#[inline]
pub fn sbc(op1: u32, op2: u32, c: bool, _v: bool) -> (u32, Flags) {
    let opc = (op2 as u64).wrapping_sub(c as u64).wrapping_add(1);
    let result = (op1 as u64).wrapping_sub(opc) as u32;

    with_flags(
        result,
        op1 as u64 >= opc,
        ((op1 ^ op2) & (op1 ^ result)).bit(31),
    )
}

#[inline]
pub fn rsc(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    sbc(op2, op1, c, v)
}

#[inline]
pub fn neg(_op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op2.wrapping_neg(), c, v)
}

#[inline]
pub fn cmp(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    sub(op1, op2, c, v)
}

#[inline]
pub fn cmn(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    add(op1, op2, c, v)
}

// Multiplication

pub fn mul(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    with_flags(op2.wrapping_mul(op1), c, v)
}

// Shift operations (for thumb)

#[inline]
pub fn lsl(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = barrel_shifter::logical_left(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn lsr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = barrel_shifter::logical_right(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn asr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = barrel_shifter::arithmetic_right(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn ror(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = barrel_shifter::rotate_right(op1, op2, c, false);

    with_flags(result, c, v)
}
