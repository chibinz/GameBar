use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;
use crate::cpu::barrel_shifter::{shift};

#[inline]
pub fn decode(instruction: u32) -> (bool, u32, bool, u32, u32, u32)
{
    let i = instruction >> 25 & 1 == 1;
    let opcode = instruction >> 21 & 0b1111;
    let s = instruction >> 20 & 1 == 1;
    let rn = instruction >> 16 & 0b1111;
    let rd = instruction >> 12 & 0b1111;
    let operand2 = instruction & 0b11111111111;

    (i, opcode, s, rn, rd, operand2)
}

#[inline]
pub fn execute(cpu: &mut CPU, (i, opcode, s, rn, rd, operand2): (bool, u32, bool, u32, u32, u32))
{
    let op1 = cpu.register.r[rn as usize];
    let op2 = 
    if i 
    {
        let rotate = operand2 >> 8 & 0b1111;
        let immediate = operand2 & 0b11111111;
        immediate.rotate_right(rotate * 2)
    }
    else
    {    
        let rm = (operand2 & 0b1111) as usize;
        let rs = (operand2 >> 8 & 0b1111) as usize;
        let stype = operand2 >> 5 & 0b11;
        let amount = if operand2 >> 4 & 1 == 1 {cpu.register.r[rs]} 
                     else {(operand2 >> 3) & 0b11111};
        shift(cpu, cpu.register.r[rm], amount, stype)
    };
     
    let carry = if cpu.register.get_cpsr_bit(C) {1} else {0};

    // Copy & pasted from 'felixzhuologist'
    // Return the sum, carry, and overflow of the two operands
    let mut add = |op1: u32, op2: u32, carry: u32| -> u32
    {
        let (r1, c1) = op1.overflowing_add(op2);
        let (r2, c2) = r1.overflowing_add(carry);

        // There's an overflow for addition when both operands are positive and the
        // result is negative, or both operands are negative and the result is positive.
        if s
        {
            let overflow = ((!(op1 ^ op2)) & (op1 ^ r2)) >> 31 & 1 == 1;
            cpu.register.set_cpsr_bit(V, overflow);
            cpu.register.set_cpsr_bit(C, c1 || c2);
        }

        r2
    };

    let result = match opcode
    {
        0b0000 => op1 & op2,
        0b0001 => op1 ^ op2,
        0b0010 => add(op1, !op2, 0),
        0b0011 => add(op2, !op1, 0),
        0b0101 => add(op1, op2, carry),
        0b0110 => add(op1, !op2, carry),
        0b0111 => add(op2, !op1, carry),
        0b1000 => op1 & op2,
        0b1001 => op1 ^ op2,
        0b1010 => add(op1, !op2, 0),
        0b1011 => add(op1, op2, 0),
        0b1100 => op1 | op2,
        0b1101 => op2,
        0b1110 => op1 & !op2,
        0b1111 => !op2,
        _      => panic!("Invalid opcode!") 
    };

    // If bit S is set, set CPSR condition flags accordingly
    if s
    {
        if result == 0
        {
            cpu.register.set_cpsr_bit(Z, true)
        }

        if result >> 31 & 1 == 1
        {
            cpu.register.set_cpsr_bit(N, true)
        }
    }

    // Write result to register, if needed
    if opcode < 0b1000 || opcode > 0b1011
    {
        cpu.register.r[rd as usize] = result;
    }
}