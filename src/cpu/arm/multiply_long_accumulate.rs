use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, u32, u32, u32, u32)
{
    debug_assert_eq!(bits(instruction, 7, 4), 0b1001);

    let u = bit(instruction, 22);
    let a = bit(instruction, 21);
    let s = bit(instruction, 20);
    let rdhi = bits(instruction, 19, 16);
    let rdlo = bits(instruction, 15, 12);
    let rs = bits(instruction, 11, 8);
    let rm = bits(instruction, 3, 0);

    // `rdhi`, `rdlo`, and `rm` must all specify different registers.
    debug_assert_ne!(rdhi, rm);
    debug_assert_ne!(rdlo, rm);
    debug_assert_ne!(rdhi, rdlo);

    // `r15` must not be used as an operand or destination register
    debug_assert_ne!(rdhi, 15);
    debug_assert_ne!(rdlo, 15);
    debug_assert_ne!(rs, 15);
    debug_assert_ne!(rm, 15);

    (u, a, s, rdhi, rdlo, rs, rm)
}

#[inline]
pub fn execute(cpu: &mut CPU, (u, a, s, rdhi, rdlo, rs, rm): (bool, bool, bool, u32, u32, u32 ,u32))
{
    let mut result: u64;

    if u
    {
        let operand1 = cpu.register.r[rm as usize] as u64;
        let operand2 = cpu.register.r[rs as usize] as u64;

        result = operand1 * operand2;
    }
    else
    {
        // Operands are sign extended to 64 bits. `i32` is necessary for sign extension.
        let operand1 = cpu.register.r[rm as usize] as i32 as i64;
        let operand2 = cpu.register.r[rs as usize] as i32 as i64;
        
        println!("{:0x} {:0x}", operand1, operand2);
        
        result = (operand1 * operand2) as u64;
    }

    if a
    {
        let hi = (cpu.register.r[rdhi as usize] as u64) << 32;
        let lo = cpu.register.r[rdlo as usize] as u64;

        result = result.wrapping_add(hi + lo);
    }

    if s
    {
        if cpu.register.r[rdhi as usize] == 0 && cpu.register.r[rdlo as usize] == 0
        {
            cpu.register.set_cpsr_bit(Z, true)
        }

        if bit(cpu.register.r[rdhi as usize], 31)
        {
            cpu.register.set_cpsr_bit(N, true)
        }

        // Both the C and V flags are set to meaningless values
    }

    cpu.register.r[rdhi as usize] = (result >> 32) as u32;
    cpu.register.r[rdlo as usize] = result as u32;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn mul_long_execute()
    {
        let mut cpu = CPU::new();

        cpu.register.r[0] = 0x10000000;
        cpu.register.r[1] = 0x10000000;

        execute(&mut cpu, (true, false, false, 4, 3, 0, 1));

        assert_eq!(cpu.register.r[3], 0);
        assert_eq!(cpu.register.r[4], 0x01000000);

        cpu.register.r[0] = 0xffffffff;
        cpu.register.r[1] = 0xffffffff;

        execute(&mut cpu, (false, false, true, 4, 3, 0, 1));
        assert_eq!(cpu.register.r[3], 1);
        assert_eq!(cpu.register.r[4], 0);
        assert_eq!(cpu.register.get_cpsr_bit(N), false);
    }
}