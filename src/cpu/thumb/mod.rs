pub mod disassemble;
pub mod move_shifted;
pub mod add_subtract;
pub mod move_compare;
pub mod alu_operations;
pub mod hi_operations_bx;
pub mod pc_relative_load;
pub mod data_transfer_reg;
pub mod single_transfer_imm;
pub mod halfword_transfer_imm;
pub mod sp_relative_load;
pub mod load_address;
pub mod add_sp;
pub mod push_pop;
pub mod multiple_transfer;
pub mod conditional_branch;
pub mod unconditional_branch;
pub mod long_branch;

use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

use super::arm::software_interrupt;

#[inline]
pub fn step(cpu: &mut CPU, memory: &mut Memory)
{
    fetch(cpu, memory);

    println!("{}", cpu);
    print!("{:08x}: {:04x} | {:016b} ", cpu.r[15] - 2, cpu.instruction, cpu.instruction);
    println!("{}", disassemble::disassemble(cpu.instruction as u16));

    execute(cpu, memory);
}

#[inline]
pub fn fetch(cpu: &mut CPU, memory: &mut Memory)
{
    cpu.instruction = memory.load16(cpu.r[15] - 2) as u32;
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory) -> u32
{
    cpu.r[15] += 2;

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
        0b00010 => move_shifted::decode_execute(cpu, instruction),
        0b00011 => add_subtract::decode_execute(cpu, instruction),
        0b00100 ..=
        0b00111 => move_compare::decode_execute(cpu, instruction),
        0b01000 =>
        {
            match instruction.bits(10, 6)
            {
                0b00000 ..=
                0b01111 => alu_operations::decode_execute(cpu, instruction),

                0b10001 ..=
                0b11101 => hi_operations_bx::decode_execute(cpu, instruction), 
                _       => unreachable!(),
            }
        },
        0b01001 => pc_relative_load::decode_execute(cpu, memory, instruction),
        0b01010 |
        0b01011 => data_transfer_reg::decode_execute(cpu, memory, instruction),
        0b01100 ..=
        0b01111 => single_transfer_imm::decode_execute(cpu, memory, instruction),
        0b10000 |
        0b10001 => halfword_transfer_imm::decode_execute(cpu, memory, instruction),
        0b10010 |
        0b10011 => sp_relative_load::decode_execute(cpu, memory, instruction),
        0b10100 |
        0b10101 => load_address::decode_execute(cpu, instruction),
        0b10110 | 0b10111 => 
        {
            match instruction.bits(11, 8)
            {
                0b0000 => add_sp::decode_execute(cpu, instruction),
                0b0100 ..=
                0b1101 => push_pop::decode_execute(cpu, memory, instruction),
                _      => unreachable!(),
            }
        },
        0b11000 |
        0b11001 => multiple_transfer::decode_execute(cpu, memory, instruction),
        0b11010 | 0b11011 => 
        {
            match instruction.bits(11, 8)
            {
                0b0000 ..=
                0b1101 => conditional_branch::decode_execute(cpu, instruction),
                0b1111 => software_interrupt::decode_execute(cpu),
                _      => unreachable!(),
            }
        },
        0b11100 => unconditional_branch::decode_execute(cpu, instruction),
        0b11110 |
        0b11111 => long_branch::decode_execute(cpu, instruction),
        _       => unimplemented!(),
    };
}