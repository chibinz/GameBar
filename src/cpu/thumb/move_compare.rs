use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::alu;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32)
{
    let op = instruction.bits(12, 11);
    let rd = instruction.bits(10, 8);
    let offset8 = instruction.bits(7, 0);

    (op, rd, offset8)
}

#[inline]
fn execute(cpu: &mut CPU, (op, rd, offset8): (u32, u32, u32))
{
    let op1 = cpu.r[rd as usize];
    let op2 = offset8;

    let result = match op
    {
        0b00 => alu::mov(cpu, op1, op2, true),
        0b01 => alu::cmp(cpu, op1, op2),
        0b10 => alu::add(cpu, op1, op2, true),
        0b11 => alu::sub(cpu, op1, op2, true),
        _    => unreachable!()
    };

    if op != 0b01
    {
        cpu.r[rd as usize] = result;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::cpu::register::PSRBit::*;

    #[test]
    fn move_compare()
    {
        let mut cpu = CPU::new();

        cpu.r[1] = 0xffffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r[1], 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);

        cpu.r[1] = 0x7fffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r[1], 0x80000000);
        assert_eq!(cpu.get_cpsr_bit(V), true);
        assert_eq!(cpu.get_cpsr_bit(C), false);
        assert_eq!(cpu.get_cpsr_bit(N), true);

        cpu.r[1] = 1;
        execute(&mut cpu, (0b01, 1, 2));
        assert_eq!(cpu.r[1], 1);
        assert_eq!(cpu.get_cpsr_bit(V), false);
        assert_eq!(cpu.get_cpsr_bit(C), false);
        assert_eq!(cpu.get_cpsr_bit(N), true);
        assert_eq!(cpu.get_cpsr_bit(Z), false);
    }
}