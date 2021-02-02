use crate::alu;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instruction: u16) -> (bool, bool, u32, u32, u32) {
    let i = instruction.bit(10);
    let op = instruction.bit(9);
    let operand2 = instruction.bits(8, 6);
    let rs = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (i, op, operand2, rs, rd)
}

#[inline]
fn execute(cpu: &mut CPU, (i, op, operand2, rs, rd): (bool, bool, u32, u32, u32)) {
    let op1 = cpu.r[rs as usize];
    let op2 = if i {
        operand2
    } else {
        cpu.r[operand2 as usize]
    };

    let result = if op {
        alu::sub(cpu, op1, op2, true)
    } else {
        alu::add(cpu, op1, op2, true)
    };

    cpu.r[rd as usize] = result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::PSRBit::*;

    #[test]
    fn add_subtract() {
        let mut cpu = CPU::new();

        cpu.r[1] = 0xffffffff;
        execute(&mut cpu, (true, false, 0b111, 1, 1));
        assert_eq!(cpu.r[1], 0b110);
        assert_eq!(cpu.get_cpsr_bit(C), true);

        cpu.r[0] = 0x10000000;
        cpu.r[1] = 1;
        execute(&mut cpu, (false, true, 1, 0, 1));
        assert_eq!(cpu.r[1], 0x0fffffff);
        assert_eq!(cpu.get_cpsr_bit(V), false);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }
}
