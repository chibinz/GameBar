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
    let op1 = cpu.r(rs);
    let op2 = if i { operand2 } else { cpu.r(operand2) };

    let (result, flags) = if op {
        alu::sub(op1, op2, cpu.cpsr.c, cpu.cpsr.v)
    } else {
        alu::add(op1, op2, cpu.cpsr.c, cpu.cpsr.v)
    };

    cpu.set_flags(flags);
    cpu.set_r(rd, result);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn add_subtract() {
        let mut cpu = CPU::new();

        cpu.set_r(1, 0xffffffff);
        execute(&mut cpu, (true, false, 0b111, 1, 1));
        assert_eq!(cpu.r(1), 0b110);
        assert_eq!(cpu.cpsr.c, true);

        cpu.set_r(0, 0x10000000);
        cpu.set_r(1, 1);
        execute(&mut cpu, (false, true, 1, 0, 1));
        assert_eq!(cpu.r(1), 0x0fffffff);
        assert_eq!(cpu.cpsr.v, false);
        assert_eq!(cpu.cpsr.c, true);
    }
}
