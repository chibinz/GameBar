mod add_sp;
mod add_subtract;
mod alu_operations;
mod conditional_branch;
mod data_transfer_reg;
mod disassemble;
mod halfword_transfer_imm;
mod hi_operations_bx;
mod load_address;
mod long_branch;
mod move_compare;
mod move_shifted;
mod multiple_transfer;
mod pc_relative_load;
mod push_pop;
mod single_transfer_imm;
mod sp_relative_load;
mod unconditional_branch;

pub use disassemble::disassemble;

use crate::CPU;
use crate::Bus;
use util::*;

#[inline]
pub fn step(cpu: &mut CPU, bus: &mut impl Bus) {
    fetch(cpu, bus);

    util::trace!("{:?}", cpu);

    // crate::push_cpu(cpu.clone());

    increment_pc(cpu);

    interpret(cpu, bus);
}

#[inline]
pub fn fetch(cpu: &mut CPU, bus: &mut impl Bus) {
    cpu.ir = CPU::ldrh(cpu.r(15) - 2, bus);
}

#[inline]
pub fn increment_pc(cpu: &mut CPU) {
    cpu.r[15] += 2;
}

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus) {
    dispatch(cpu, bus);
}

#[inline]
pub fn dispatch(cpu: &mut CPU, bus: &mut impl Bus) {
    let instr = cpu.ir as u16;

    match instr.bits(15, 11) {
        0b00000..=0b00010 => move_shifted::interpret(cpu, instr),
        0b00011 => add_subtract::interpret(cpu, instr),
        0b00100..=0b00111 => move_compare::interpret(cpu, instr),
        0b01000 => match instr.bits(10, 6) {
            0b00000..=0b01111 => alu_operations::interpret(cpu, instr),

            0b10001..=0b11101 => hi_operations_bx::interpret(cpu, instr),
            _ => unreachable!(),
        },
        0b01001 => pc_relative_load::interpret(cpu, bus, instr),
        0b01010 | 0b01011 => data_transfer_reg::interpret(cpu, bus, instr),
        0b01100..=0b01111 => single_transfer_imm::interpret(cpu, bus, instr),
        0b10000 | 0b10001 => halfword_transfer_imm::interpret(cpu, bus, instr),
        0b10010 | 0b10011 => sp_relative_load::interpret(cpu, bus, instr),
        0b10100 | 0b10101 => load_address::interpret(cpu, instr),
        0b10110 | 0b10111 => match instr.bits(11, 8) {
            0b0000 => add_sp::interpret(cpu, instr),
            0b0100..=0b1101 => push_pop::interpret(cpu, bus, instr),
            _ => unreachable!(),
        },
        0b11000 | 0b11001 => multiple_transfer::interpret(cpu, bus, instr),
        0b11010 | 0b11011 => match instr.bits(11, 8) {
            0b0000..=0b1101 => conditional_branch::interpret(cpu, instr),
            0b1111 => cpu.software_interrupt(),
            _ => unreachable!(),
        },
        0b11100 => unconditional_branch::interpret(cpu, instr),
        0b11110 | 0b11111 => long_branch::interpret(cpu, instr),
        _ => unimplemented!(),
    };
}
