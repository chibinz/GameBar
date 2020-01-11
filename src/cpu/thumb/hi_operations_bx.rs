use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::alu;
use crate::cpu::register::PSRBit::T;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32)
{
    let op = instruction.bits(9, 8);

    // h1 and h2 are concatenated as sign bits to rd and rs.
    // rs = h2 || rs, rd = h1 || rd
    let rs = instruction.bits(6, 3);
    let rd = instruction.bits(2, 0) | (instruction as u32 >> 4) & 0b1000;

    (op, rs, rd)
}
 
#[inline]
fn execute(cpu: &mut CPU, (op, rs, rd): (u32, u32, u32))
{   
    let op1 = cpu.register.r[rd as usize];
    let op2 = cpu.register.r[rs as usize];

    match op
    {
        0b00 => cpu.register.r[rd as usize] = 
                alu::add(cpu, op1, op2, false),
        0b01 => 
            {
                alu::cmp(cpu, op1, op2);
            },
        0b10 => cpu.register.r[rd as usize] = 
                alu::mov(cpu, op1, op2, false),
        0b11 => 
            {
                cpu.register.set_cpsr_bit(T, cpu.register.r[rs as usize].bit(0));

                cpu.register.r[15] = cpu.register.r[rs as usize] & 0xfffffffe;
                cpu.flushed = true;
            },
        _    => unreachable!()
    };

}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::cpu::register::PSRBit::*;
    
    #[test]
    fn alu_operations()
    {
        let mut cpu = CPU::new();

        // All operations except CMP does not effect CPSR flags.
        // ADD 2, 0xffffffff
        cpu.register.r[8] = 0xffffffff;
        cpu.register.r[9] = 2;
        execute(&mut cpu, (0b00, 8, 9));
        assert_eq!(cpu.register.r[9], 1);
        assert_eq!(cpu.register.get_cpsr_bit(C), false);

        // CMP 1, 1
        cpu.register.r[8] = 1;
        execute(&mut cpu, (0b01, 8, 9));
        assert_eq!(cpu.register.r[9], 1);
        assert_eq!(cpu.register.get_cpsr_bit(Z), true);

        // BX 
        cpu.register.r[14] = 0xffffffff;
        execute(&mut cpu, (0b11, 14, 0));
        assert_eq!(cpu.register.r[15], 0xfffffffe);
    }
}