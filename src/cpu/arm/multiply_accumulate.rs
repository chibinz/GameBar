use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, u32, u32, u32, u32)
{
    debug_assert_eq!(bits(instruction, 7, 4), 0b1001);

    let a = bit(instruction, 21);
    let s = bit(instruction, 20);
    let rd = bits(instruction, 19, 16);
    let rn = bits(instruction, 15, 12);
    let rs = bits(instruction, 11, 8);
    let rm = bits(instruction, 3, 0);

    // The destination register rd must not be the same as the operand register rm
    debug_assert_ne!(rd, rm);

    // `r15` must not be used as an operand or destination register
    debug_assert_ne!(rd, 15);
    debug_assert_ne!(rn, 15);
    debug_assert_ne!(rs, 15);
    debug_assert_ne!(rm, 15);

    (a, s, rd, rn, rs, rm)
}

#[inline]
pub fn execute(cpu: &mut CPU, (a, s, rd, rn, rs, rm): (bool, bool, u32, u32, u32, u32))
{
    let mut result = cpu.register.r[rm as usize].wrapping_mul(cpu.register.r[rs as usize]);

    // If accumulate bit is set, add rn to result
    if a
    {
        result = result.wrapping_add(cpu.register.r[rn as usize]);
    }
    else
    {
        debug_assert_eq!(cpu.register.r[rn as usize], 0);
    }

    if s
    {
        if result == 0
        {
            cpu.register.set_cpsr_bit(Z, true)
        }

        if bit(result, 31)
        {
            cpu.register.set_cpsr_bit(N, true)
        }

        // The C (Carry) flag is set to a meaningless value.
        // And the V (Overflow) flag is unaffected.
    }

    cpu.register.r[rd as usize] = result;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn mul_execute()
    {
        let mut cpu = CPU::new();

        cpu.register.r[0] = 0xfffffff6;
        cpu.register.r[1] = 0x00000014;

        execute(&mut cpu, (false, false, 3, 4, 0, 1));

        assert_eq!(cpu.register.r[3], 0xffffff38);

        cpu.register.r[0] = 0x10;
        cpu.register.r[1] = 0x10000000;

        execute(&mut cpu, (false, true, 3, 4, 0, 1));
        assert_eq!(cpu.register.r[3], 0);
        assert_eq!(cpu.register.get_cpsr_bit(Z), true);
        assert_eq!(cpu.register.get_cpsr_bit(N), false);
    }
}