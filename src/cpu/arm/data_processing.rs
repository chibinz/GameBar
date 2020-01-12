use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::alu;
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
    let op1 = cpu.r[rn as usize];
    let op2 = if i {rotate_immediate(cpu, operand2)} else {shift_register(cpu, operand2)};
     
    let result = match opcode
    {
        0b0000 => alu::and(cpu, op1, op2, s),
        0b0001 => alu::eor(cpu, op1, op2, s),
        0b0010 => alu::sub(cpu, op1, op2, s),
        0b0011 => alu::rsb(cpu, op1, op2, s),
        0b0100 => alu::add(cpu, op1, op2, s),
        0b0101 => alu::adc(cpu, op1, op2, s),
        0b0110 => alu::sbc(cpu, op1, op2, s),
        0b0111 => alu::rsc(cpu, op1, op2, s),
        0b1000 => alu::tst(cpu, op1, op2),
        0b1001 => alu::teq(cpu, op1, op2),
        0b1010 => alu::cmp(cpu, op1, op2),
        0b1011 => alu::cmn(cpu, op1, op2),
        0b1100 => alu::orr(cpu, op1, op2, s),
        0b1101 => alu::mov(cpu, op1, op2, s),
        0b1110 => alu::bic(cpu, op1, op2, s),
        0b1111 => alu::mvn(cpu, op1, op2, s),
        _      => unreachable!() 
    };

    // Write result to register, if needed
    if opcode < 0b1000 || opcode > 0b1011
    {
        cpu.r[rd as usize] = result;

        if rd == 15
        {
            cpu.flushed = true;
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::cpu::register::PSRBit::*;
    
    #[test]
    fn data_execute()
    {
        let mut cpu = CPU::new();

        // AND R1, R2, R4 LSL R1
        cpu.r[1] = 1;
        cpu.r[2] = 2;
        cpu.r[3] = 1;
        cpu.r[4] = 0xffffffff;
        execute(&mut cpu, (false, 0b0000, true, 2, 1, 0b0011_0_00_1_0100));
        assert_eq!(cpu.r[1], 2);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }
}