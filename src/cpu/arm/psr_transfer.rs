use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::rotate_immediate;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    let l = bit(instruction, 21);
    let pd = bit(instruction, 22);

    if l // MSR
    {
        let i = bit(instruction, 25);
        // Bit 16 of MSR instructions varies.
        // When it is clear, only PSR flag bits are
        // transeferred, otherwise all defined bits of source
        // register is transferred.
        let f = !bit(instruction, 16);
        let operand2 = bits(instruction, 11, 0);

        let op = 
        if i 
        {
            rotate_immediate(operand2)
        }
        else
        {
            debug_assert_eq!(bits(operand2, 11, 4), 0);

            let rm = bits(operand2, 3, 0);
            cpu.register.r[rm as usize]
        };

        if pd {cpu.register.set_spsr(op, f);} else {cpu.register.set_cpsr(op, f);}

    }
    else // MRS
    {
        debug_assert_eq!(bits(instruction, 21, 16), 0b001111);
        debug_assert_eq!(bits(instruction, 11, 0), 0);
        
        let rd = bits(instruction, 15, 12);
        let psr = if pd {cpu.register.get_spsr()} else {cpu.register.get_cpsr()};
        
        cpu.register.r[rd as usize] = psr;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_msr()
    {
        let mut cpu = CPU::new();

        // MSR CPSR
        cpu.register.r[0] = 0xfffffff1;
        decode_execute(&mut cpu, 0b0000_00010_0_10100_1_1111_00000000_0000);
        assert_eq!(cpu.register.get_cpsr(), 0xf00000f1);

        // MSR SPSR flag bits
        decode_execute(&mut cpu, 0b0000_00010_1_10100_0_1111_00000000_0000);
        assert_eq!(cpu.register.get_spsr(), 0xf0000000);
    }
}