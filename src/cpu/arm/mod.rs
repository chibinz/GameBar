pub mod disassemble;
pub mod data_processing;
pub mod psr_transfer;
pub mod branch_long;
pub mod branch_exchange;
pub mod multiply_accumulate;
pub mod multiply_long_accumulate;
pub mod single_data_transfer;
pub mod single_data_swap;
pub mod halfword_data_transfer;
pub mod block_data_transfer;
pub mod software_interrupt;

use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn step(cpu: &mut CPU, memory: &mut Memory)
{
    fetch(cpu, memory);

    println!("{}", cpu);
    print!("{:08x}: {:08x} | {:032b} ", cpu.r[15] - 4, cpu.instruction, cpu.instruction);
    println!("{}", disassemble::disassemble(cpu.instruction));

    execute(cpu, memory);
}

#[inline]
pub fn fetch(cpu: &mut CPU, memory: &mut Memory)
{
    cpu.instruction = memory.load32(cpu.r[15] - 4);
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory) -> u32
{
    cpu.r[15] += 4;
    
    let cond = cpu.instruction.bits(31, 28);
    if cpu.check_condition(cond)
    {
        dispatch(cpu, memory);
    }

    return 0;
}

#[inline]
pub fn dispatch(cpu: &mut CPU, memory: &mut Memory)
{
    let instruction = cpu.instruction;

    let b74 = || instruction >> 6 & 0b10 | instruction >> 4 & 0b01;
    let b65 = || instruction >> 5 & 0b11;

    // Data Processing / PSR Transfer / branch and exchange
    let data_process_psr_bx = |cpu: &mut CPU|
    {
        match bits(instruction, 24, 20)
        {
            0b10000 | 0b10100 | 0b10110  => psr_transfer::decode_execute(cpu, instruction),
            0b10010           => if b74() == 0 
                                {psr_transfer::decode_execute(cpu, instruction)} else
                                {branch_exchange::decode_execute(cpu, instruction)},
            _                 => data_processing::decode_execute(cpu, instruction)
        };
    };

    // Multiply / Multiply Long / Single Data Swap
    let multiply_swap = |cpu: &mut CPU, memory: &mut Memory|
    {
        match bits(instruction, 24, 20)
        {
            0b00000 | 0b00001 |
            0b00010 | 0b00011 => multiply_accumulate::decode_execute(cpu, instruction),
            0b01000 | 0b01001 |
            0b01010 | 0b01011 |
            0b01100 | 0b01101 |
            0b01110 | 0b01111 => multiply_long_accumulate::decode_execute(cpu, instruction),

            // Single data swap
            0b10000 | 0b10100 => single_data_swap::decode_execute(cpu, memory, instruction),

            _                 => unreachable!(),
        };
    };
    
    match bits(instruction, 27, 25)
    {
        0b000 =>
        {
            if b74() < 0b11
            {
                data_process_psr_bx(cpu)
            }
            else
            {
                if b65() > 0
                {
                    halfword_data_transfer::decode_execute(cpu, memory, instruction)
                }
                else
                {
                    multiply_swap(cpu, memory)
                }
            }
        },
        0b001 => match bits(instruction, 24, 20)
                {
                    0b10110 | 0b10010 => psr_transfer::decode_execute(cpu, instruction),
                    _                 => data_processing::decode_execute(cpu, instruction)
                },
        0b010 |
        0b011 => single_data_transfer::decode_execute(cpu, memory, instruction),
        0b100 => block_data_transfer::decode_execute(cpu, memory, instruction),
        0b101 => branch_long::decode_execute(cpu, instruction), 
        0b111 => software_interrupt::decode_execute(cpu),
        _     => unimplemented!(),
    };
}