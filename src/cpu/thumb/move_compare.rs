use crate::util::BitField;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u16)
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
    let op1 = cpu.register.r[rd as usize];
    let op2 = offset8;

    let mut add = |op1: u32, op2: u32| -> u32
    {
        let (r1, c1) = op1.overflowing_add(op2);
        let overflow = (op1 as i32).overflowing_add(op2 as i32).1;
        
        dbg!(op1, op2, op1 as i32, op2 as i32, (op1 as i32).overflowing_add(op2 as i32));

        cpu.register.set_cpsr_bit(V, overflow);
        cpu.register.set_cpsr_bit(C, c1);

        r1
    };

    let result = match op
    {
        0b00 => op2,
        0b01 => add(op1, !op2 + 1),
        0b10 => add(op1, op2),
        0b11 => add(op1, !op2 + 1),
        _    => unreachable!()
    };

    cpu.register.set_cpsr_bit(Z, result == 0);
    cpu.register.set_cpsr_bit(N, result.bit(31));

    if op != 0b01
    {
        cpu.register.r[rd as usize] = result;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::cpu::register::PSRBit::*;
    
    #[test]
    fn add()
    {
        let mut cpu = CPU::new();

        cpu.register.r[1] = 0xffffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.register.r[1], 0);
        assert_eq!(cpu.register.get_cpsr_bit(C), true);

        cpu.register.r[1] = 0x7fffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.register.r[1], 0x80000000);
        assert_eq!(cpu.register.get_cpsr_bit(V), true);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);
        assert_eq!(cpu.register.get_cpsr_bit(N), true);

        cpu.register.r[1] = 1;
        execute(&mut cpu, (0b01, 1, 2));
        assert_eq!(cpu.register.r[1], 1);
        assert_eq!(cpu.register.get_cpsr_bit(V), false);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);
        assert_eq!(cpu.register.get_cpsr_bit(N), true);
        assert_eq!(cpu.register.get_cpsr_bit(Z), false);
    }
}