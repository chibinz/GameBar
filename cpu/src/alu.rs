//! The reason to separate ALU implementations is because many
//! instructions uses it, in both arm and thumb mode.
//! ALU operations may change CPSR flags but not GPR contents.
//! The result of the operation is passed back as return values.

use crate::shifter::*;
use util::*;

type Flags = (bool, bool, bool, bool);

// CPSR flag manipulation
impl crate::Cpu {
    pub fn set_flags(&mut self, (n, z, c, v): Flags) {
        self.cpsr.n = n;
        self.cpsr.z = z;
        self.cpsr.c = c;
        self.cpsr.v = v;
    }
}

#[inline]
pub fn with_flags(result: u32, c: bool, v: bool) -> (u32, Flags) {
    (result, (result.bit(31), result == 0, c, v))
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
        op1.overflowing_add(op2).1,
        (op1 as i32).overflowing_add(op2 as i32).1,
    )
}

#[inline]
pub fn adc(op1: u32, op2: u32, c: bool, _v: bool) -> (u32, Flags) {
    let (opc, c1) = op2.overflowing_add(c as u32);
    let (result, c2) = op1.overflowing_add(opc);

    with_flags(result, c1 || c2, ((!op1 ^ op2) & (op1 ^ result)).bit(31))
}

#[inline]
pub fn sub(op1: u32, op2: u32, _c: bool, _v: bool) -> (u32, Flags) {
    with_flags(
        op1.wrapping_sub(op2),
        op1 >= op2,
        (op1 as i32).overflowing_sub(op2 as i32).1,
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
    let (result, c) = logical_left(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn lsr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = logical_right(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn asr(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = arithmetic_right(op1, op2, c, false);

    with_flags(result, c, v)
}

#[inline]
pub fn ror(op1: u32, op2: u32, c: bool, v: bool) -> (u32, Flags) {
    let (result, c) = rotate_right(op1, op2, c, false);

    with_flags(result, c, v)
}
