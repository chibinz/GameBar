use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::shift;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32, u32)
{
    let op = instruction.bits(12, 11);
    let offset5 = instruction.bits(10, 6);
    let rs = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (op, offset5, rs, rd)
}

#[inline]
fn execute(cpu: &mut CPU, (op, offset5, rs, rd): (u32, u32, u32, u32))
{
    let shifted = shift(cpu, cpu.register.r[rs as usize], offset5, op);

    cpu.register.r[rd as usize] = shifted;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::cpu::register::PSRBit::*;
    
    #[test]
    fn logical_left()
    {
        let mut cpu = CPU::new();

        cpu.register.r[0] = 1;
        execute(&mut cpu, (0b00, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 0x80000000);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);

        cpu.register.r[0] = 0b10;
        execute(&mut cpu, (0b00, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 0);
        assert_eq!(cpu.register.get_cpsr_bit(C), true);
    }

    #[test]
    fn logical_right()
    {
        let mut cpu = CPU::new();

        cpu.register.r[0] = 0x80000000;
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 1);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);

        cpu.register.r[0] = 0x40000000;
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 0);
        assert_eq!(cpu.register.get_cpsr_bit(C), true);
    }

    #[test]
    fn arithmetic_right()
    {
        let mut cpu = CPU::new();

        cpu.register.r[0] = 0x80000000;
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 0xffffffff);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);

        cpu.register.r[0] = 0x40000000;
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.register.r[1], 0);
        assert_eq!(cpu.register.get_cpsr_bit(C), true);
    }
}