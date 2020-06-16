//! The reason to separate ALU implementations is because many
//! instructions uses it, in both arm and thumb mode.
//! ALU operations may change CPSR flags but not GPR contents.
//! The result of the operation is passed back as return values.

use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter;
use crate::cpu::register::PSRBit::*;

// CPSR flag manipulation

#[inline]
pub fn zero(result: u32) -> bool
{
    result == 0
}

#[inline]
pub fn negative(result: u32) -> bool
{
    result.bit(31)
}

#[inline]
fn add_carry(op1: u32, op2: u32) -> bool
{
    op1.overflowing_add(op2).1
}

#[inline]
fn add_overflow(op1: u32, op2: u32) -> bool
{
    (op1 as i32).overflowing_add(op2 as i32).1
}

#[inline]
fn sub_carry(op1: u32, op2: u32) -> bool
{
    op1 >= op2
}

#[inline]
fn sub_overflow(op1: u32, op2: u32) -> bool
{
    (op1 as i32).overflowing_sub(op2 as i32).1
}

// Logical operations

#[inline]
pub fn and(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1 & op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn eor(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1 ^ op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn orr(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1 | op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn bic(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1 & !op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn mov(cpu: &mut CPU, _op1: u32, op2: u32, s: bool) -> u32
{
    let result = op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn mvn(cpu: &mut CPU, _op1: u32, op2: u32, s: bool) -> u32
{
    let result = !op2;

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

#[inline]
pub fn tst(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    and(cpu, op1, op2, true)
}

#[inline]
pub fn teq(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    eor(cpu, op1, op2, true)
}

// Arithemetic operations

#[inline]
pub fn add(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1.wrapping_add(op2);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, add_carry(op1, op2));
        cpu.set_cpsr_bit(V, add_overflow(op1, op2));
    }

    result
}

#[inline]
pub fn adc(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let carry = cpu.get_cpsr_bit(C) as u32;
    let result = op1.wrapping_add(op2.wrapping_add(carry));

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, add_carry(op1, op2.wrapping_add(carry)));
        cpu.set_cpsr_bit(V, add_overflow(op1, op2.wrapping_add(carry)));
    }

    result
}

#[inline]
pub fn sub(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op1.wrapping_sub(op2);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, sub_carry(op1, op2));
        cpu.set_cpsr_bit(V, sub_overflow(op1, op2));
    }

    result
}

#[inline]
pub fn rsb(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op2.wrapping_sub(op1);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, sub_carry(op2, op1));
        cpu.set_cpsr_bit(V, sub_overflow(op2, op1));
    }

    result
}

#[inline]
pub fn sbc(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let carry: u32 = cpu.get_cpsr_bit(C) as u32;
    let opc = op2.wrapping_sub(carry).wrapping_add(1);
    let result = op1.wrapping_sub(opc);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, sub_carry(op1, opc));
        cpu.set_cpsr_bit(V, sub_overflow(op1, opc));
    }

    result
}

#[inline]
pub fn rsc(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let carry: u32 = cpu.get_cpsr_bit(C) as u32;
    let opc = op1.wrapping_sub(carry).wrapping_add(1);
    let result = op2.wrapping_sub(opc);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
        cpu.set_cpsr_bit(C, sub_carry(op2, opc));
        cpu.set_cpsr_bit(V, sub_overflow(op2, opc));
    }

    result
}

#[inline]
pub fn neg(cpu: &mut CPU, _op1: u32, op2: u32) -> u32
{
    rsb(cpu, op2, 0, true)
}

#[inline]
pub fn cmp(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    sub(cpu, op1, op2, true)
}

#[inline]
pub fn cmn(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    add(cpu, op1, op2, true)
}

// Multiplication

pub fn mul(cpu: &mut CPU, op1: u32, op2: u32, s: bool) -> u32
{
    let result = op2.wrapping_mul(op1);

    if s
    {
        cpu.set_cpsr_bit(N, negative(result));
        cpu.set_cpsr_bit(Z, zero(result));
    }

    result
}

// Shift operations (for thumb)

#[inline]
pub fn lsl(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    let result = barrel_shifter::logical_left(cpu, op1, op2, false);

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn lsr(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    let result = barrel_shifter::logical_right(cpu, op1, op2, false);

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn asr(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    let result = barrel_shifter::arithmetic_right(cpu, op1, op2, false);

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}

#[inline]
pub fn ror(cpu: &mut CPU, op1: u32, op2: u32) -> u32
{
    let result = barrel_shifter::rotate_right(cpu, op1, op2, false);

    cpu.set_cpsr_bit(N, negative(result));
    cpu.set_cpsr_bit(Z, zero(result));

    result
}