use crate::alu;
use crate::barrel_shifter::{rotate_immediate, shift_register};
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u32) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, u32, bool, u32, u32, u32) {
    let i = instr.bit(25);
    let opcode = instr.bits(24, 21);
    let s = instr.bit(20);
    let rn = instr.bits(19, 16);
    let rd = instr.bits(15, 12);
    let operand2 = instr.bits(11, 0);

    (i, opcode, s, rn, rd, operand2)
}

#[inline]
pub fn execute(cpu: &mut CPU, (i, opcode, s, rn, rd, operand2): (bool, u32, bool, u32, u32, u32)) {
    let v = cpu.get_cpsr_bit(crate::register::PSRBit::V);
    let oldc = cpu.carry(); // Old carry
    let mut op1 = cpu.r(rn);
    let (mut op2, c) = if i {
        rotate_immediate(operand2, cpu.carry())
    } else {
        shift_register(cpu, operand2)
    };

    // If a register is used to specify shift amount, the value of pc
    // will be 12 head of the address of the currently executed instruction.
    if !i && operand2.bit(4) {
        let rm = operand2.bits(3, 0);

        if rn == 15 {
            op1 += 4
        };
        if rm == 15 {
            op2 += 4
        };
    }

    // adc, sbc, rsc use old carry flag
    let (result, flags) = match opcode {
        0b0000 => alu::and(op1, op2, c, v),
        0b0001 => alu::eor(op1, op2, c, v),
        0b0010 => alu::sub(op1, op2, c, v),
        0b0011 => alu::rsb(op1, op2, c, v),
        0b0100 => alu::add(op1, op2, c, v),
        0b0101 => alu::adc(op1, op2, oldc, v),
        0b0110 => alu::sbc(op1, op2, oldc, v),
        0b0111 => alu::rsc(op1, op2, oldc, v),
        0b1000 => alu::tst(op1, op2, c, v),
        0b1001 => alu::teq(op1, op2, c, v),
        0b1010 => alu::cmp(op1, op2, c, v),
        0b1011 => alu::cmn(op1, op2, c, v),
        0b1100 => alu::orr(op1, op2, c, v),
        0b1101 => alu::mov(op1, op2, c, v),
        0b1110 => alu::bic(op1, op2, c, v),
        0b1111 => alu::mvn(op1, op2, c, v),
        _ => unreachable!(),
    };

    if s && rd != 15 {
        cpu.set_carry(c);
        alu::set_flags(cpu, flags);
    }

    // Write result to register, if needed
    if opcode < 0b1000 || opcode > 0b1011 {
        cpu.set_r(rd, result);

        if rd == 15 && s {
            // Direct manipulation of pc will result in a pipeline flush.
            // The next instruction will be fetched from memory address
            // at pc. pc is further incremented by 4 to maintain offset 8
            // from the currently executed instruction.
            cpu.restore_cpsr();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_execute() {
        let mut cpu = CPU::new();

        // AND R1, R2, R4 LSL R1
        cpu.set_r(1, 1);
        cpu.set_r(2, 2);
        cpu.set_r(3, 1);
        cpu.set_r(4, 0xffffffff);
        execute(&mut cpu, (false, 0b0000, true, 2, 1, 0b0011_0_00_1_0100));
        assert_eq!(cpu.r(1), 2);
        assert_eq!(cpu.carry(), true);
    }

    #[test]
    fn fuzzarm_adc() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 0x1fffffff);
        cpu.set_r(1, 0xe8888888);
        cpu.set_r(2, 0x0000001f);
        cpu.set_cpsr(0xf0000000, true);

        execute(&mut cpu, (false, 0b0101, true, 0, 4, 0b0010_0_10_1_0001));

        assert_eq!(cpu.r(4), cpu.r(0));
        assert_eq!(cpu.carry(), true); // Carry bit should be set
    }

    #[test]
    fn fuzzarm_sbc() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 0x61111111);
        cpu.set_r(1, 0xb3333333);
        cpu.set_r(2, 0x00000020);
        cpu.set_cpsr(0x10000000, true);

        execute(&mut cpu, (false, 0b0110, false, 0, 4, 0b0010_0_10_1_0001));

        assert_eq!(cpu.r(4), cpu.r(0));
        assert_eq!(cpu.carry(), false); // Carry bit should be clear
    }
}
