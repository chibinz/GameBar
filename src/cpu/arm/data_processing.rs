use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;
use crate::cpu::barrel_shifter::{shift_register, rotate_immediate};

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, u32, bool, u32, u32, u32)
{
    let i = instruction.bit(25);
    let opcode = instruction.bits(24, 21);
    let s = instruction.bit(20);
    let rn = instruction.bits(19, 16);
    let rd = instruction.bits(15, 12);
    let operand2 = instruction.bits(11, 0);

    (i, opcode, s, rn, rd, operand2)
}

#[inline]
pub fn execute(cpu: &mut CPU, (i, opcode, s, rn, rd, operand2): (bool, u32, bool, u32, u32, u32))
{
    let op1 = cpu.register.r[rn as usize];
    
    // CPSR C flag may be changed in the barrel shifter
    let op2 = if i {rotate_immediate(operand2)} else {shift_register(cpu, operand2)};
     
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
            let overflow = (op1 as i32).overflowing_add(op2 as i32).1;
            cpu.register.set_cpsr_bit(V, overflow);
            cpu.register.set_cpsr_bit(C, c1 || c2);
        }

        r2
    };

    // Subtracting an operand is adding its 2's complement.
    // An operand's 2's complement is its negation plus one.
    let result = match opcode
    {
        0b0000 => op1 & op2,
        0b0001 => op1 ^ op2,
        0b0010 => add(op1, !op2 + 1, 0),
        0b0011 => add(op2, !op1 + 1, 0),
        0b0100 => add(op1, op2, 0),
        0b0101 => add(op1, op2, carry),
        0b0110 => add(op1, !op2 + 1, carry.wrapping_sub(1)),
        0b0111 => add(op2, !op1 + 1, carry.wrapping_sub(1)),
        0b1000 => op1 & op2,
        0b1001 => op1 ^ op2,
        0b1010 => add(op1, !op2 + 1, 0),
        0b1011 => add(op1, op2, 0),
        0b1100 => op1 | op2,
        0b1101 => op2,
        0b1110 => op1 & !op2,
        0b1111 => !op2,
        _      => unreachable!("Invalid opcode!") 
    };

    // If S bit is set, set CPSR condition flags accordingly
    if s
    {
        cpu.register.set_cpsr_bit(Z, result == 0);
        cpu.register.set_cpsr_bit(N, result.bit(31));

        // If S bit is set and `rd` is pc, move the SPSR corresponding to the 
        // current mode to the CPSR
        if rd == 15
        {
            cpu.register.restore_cpsr();
        }
    }

    // Write result to register, if needed
    if opcode < 0b1000 || opcode > 0b1011
    {
        cpu.register.r[rd as usize] = result;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn data_execute()
    {
        let mut cpu = CPU::new();

        // AND R1, R2, R4 LSL R1
        cpu.register.r[1] = 1;
        cpu.register.r[2] = 2;
        cpu.register.r[3] = 1;
        cpu.register.r[4] = 0xffffffff;
        execute(&mut cpu, (false, 0b0000, true, 2, 1, 0b0011_0_00_1_0100));
        assert_eq!(cpu.register.r[1], 2);
        assert_eq!(cpu.register.get_cpsr_bit(C), true);
    }
}