pub mod disassemble;
mod move_shifted;
mod add_subtract;
mod move_compare;
mod alu_operations;
mod hi_operations_bx;
mod pc_relative_load;
mod data_transfer_reg;
mod single_transfer_imm;
mod halfword_transfer_imm;
mod sp_relative_load;
mod load_address;
mod add_sp;
mod push_pop;
mod multiple_transfer;
mod conditional_branch;
mod unconditional_branch;
mod long_branch;

use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn step(cpu: &mut CPU, memory: &mut Memory)
{
    fetch(cpu, memory);

    increment_pc(cpu);

    interpret(cpu, memory);
}

#[inline]
pub fn fetch(cpu: &mut CPU, memory: &mut Memory)
{
    cpu.instruction = memory.load16(cpu.r[15] - 2) as u32;
    cpu.prefetched = memory.load16(cpu.r[15]) as u32;
}

#[inline]
pub fn increment_pc(cpu: &mut CPU)
{
    cpu.r[15] += 2;
}

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory) -> u32
{
    dispatch(cpu, memory);

    0
}

#[inline]
pub fn dispatch(cpu: &mut CPU, memory: &mut Memory)
{
    let instruction = cpu.instruction as u16;

    match instruction.bits(15, 11)
    {
        0b00000 ..=
        0b00010 => move_shifted::interpret(cpu, instruction),
        0b00011 => add_subtract::interpret(cpu, instruction),
        0b00100 ..=
        0b00111 => move_compare::interpret(cpu, instruction),
        0b01000 =>
        {
            match instruction.bits(10, 6)
            {
                0b00000 ..=
                0b01111 => alu_operations::interpret(cpu, instruction),

                0b10001 ..=
                0b11101 => hi_operations_bx::interpret(cpu, instruction),
                _       => unreachable!(),
            }
        },
        0b01001 => pc_relative_load::interpret(cpu, memory, instruction),
        0b01010 |
        0b01011 => data_transfer_reg::interpret(cpu, memory, instruction),
        0b01100 ..=
        0b01111 => single_transfer_imm::interpret(cpu, memory, instruction),
        0b10000 |
        0b10001 => halfword_transfer_imm::interpret(cpu, memory, instruction),
        0b10010 |
        0b10011 => sp_relative_load::interpret(cpu, memory, instruction),
        0b10100 |
        0b10101 => load_address::interpret(cpu, instruction),
        0b10110 | 0b10111 =>
        {
            match instruction.bits(11, 8)
            {
                0b0000 => add_sp::interpret(cpu, instruction),
                0b0100 ..=
                0b1101 => push_pop::interpret(cpu, memory, instruction),
                _      => unreachable!(),
            }
        },
        0b11000 |
        0b11001 => multiple_transfer::interpret(cpu, memory, instruction),
        0b11010 | 0b11011 =>
        {
            match instruction.bits(11, 8)
            {
                0b0000 ..=
                0b1101 => conditional_branch::interpret(cpu, instruction),
                0b1111 => cpu.software_interrupt(),
                _      => unreachable!(),
            }
        },
        0b11100 => unconditional_branch::interpret(cpu, instruction),
        0b11110 |
        0b11111 => long_branch::interpret(cpu, instruction),
        _       => unimplemented!(),
    };
}