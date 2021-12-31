use crate::shifter::rotate_immediate;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u32) {
    let l = instr.bit(21);
    let pd = instr.bit(22);

    if l
    // MSR
    {
        let i = instr.bit(25);
        // Bit 16 of MSR instructions varies.
        // When it is clear, only PSR flag bits are
        // transeferred, otherwise all defined bits of source
        // register is transferred.
        let f = !instr.bit(16);
        let operand2 = instr.bits(11, 0);

        let op = if i {
            rotate_immediate(operand2, cpu.cpsr.c).0
        } else {
            debug_assert_eq!(operand2.bits(11, 4), 0);
            cpu.r(operand2.bits(3, 0))
        };

        if pd {
            cpu.set_spsr(op, f);
        } else {
            cpu.set_cpsr(op, f);
        }
    } else
    // MRS
    {
        debug_assert_eq!(instr.bits(21, 16), 0b001111);
        debug_assert_eq!(instr.bits(11, 0), 0);

        let rd = instr.bits(15, 12);
        let psr = if pd { cpu.get_spsr() } else { cpu.get_cpsr() };

        cpu.set_r(rd, psr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msr() {
        let mut cpu = Cpu::new();

        // MSR CPSR
        cpu.set_r(0, 0xf00000f1);
        interpret(&mut cpu, 0b0000_0001_0010_1001_1111_0000_0000_0000);
        assert_eq!(cpu.get_cpsr(), 0xf00000f1);

        // MSR SPSR flag bits
        interpret(&mut cpu, 0b0000_0001_0110_1000_1111_0000_0000_0000);
        assert_eq!(cpu.get_spsr(), 0xf0000000);
    }
}
