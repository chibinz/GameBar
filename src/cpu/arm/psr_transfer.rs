use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::rotate_immediate;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    let l = instruction.bit(21);
    let pd = instruction.bit(22);

    if l // MSR
    {
        let i = instruction.bit(25);
        // Bit 16 of MSR instructions varies.
        // When it is clear, only PSR flag bits are
        // transeferred, otherwise all defined bits of source
        // register is transferred.
        let f = !instruction.bit(16);
        let operand2 = instruction.bits(11, 0);

        let op = 
        if i 
        {
            rotate_immediate(cpu, operand2)
        }
        else
        {
            debug_assert_eq!(operand2.bits(11, 4), 0);

            let rm = operand2.bits(3, 0);
            cpu.r[rm as usize]
        };

        if pd {cpu.set_spsr(op, f);} else {cpu.set_cpsr(op, f);}

    }
    else // MRS
    {
        debug_assert_eq!(instruction.bits(21, 16), 0b001111);
        debug_assert_eq!(instruction.bits(11, 0), 0);
        
        let rd = instruction.bits(15, 12);
        let psr = if pd {cpu.get_spsr()} else {cpu.get_cpsr()};
        
        cpu.r[rd as usize] = psr;
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
        cpu.r[0] = 0xf00000f1;
        decode_execute(&mut cpu, 0b0000_00010_0_10100_1_1111_00000000_0000);
        assert_eq!(cpu.get_cpsr(), 0xf00000f1);

        // MSR SPSR flag bits
        decode_execute(&mut cpu, 0b0000_00010_1_10100_0_1111_00000000_0000);
        assert_eq!(cpu.get_spsr(), 0xf0000000);
    }
}