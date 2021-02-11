pub mod block_data_transfer;

mod branch_exchange;
mod branch_long;
mod data_processing;
mod disassemble;
mod halfword_data_transfer;
mod multiply_accumulate;
mod multiply_long_accumulate;
mod psr_transfer;
mod single_data_swap;
mod single_data_transfer;

pub use disassemble::disassemble;

use crate::CPU;
use crate::{push_cpu, Bus};
use util::*;

#[inline]
pub fn step(cpu: &mut CPU, bus: &mut impl Bus) {
    fetch(cpu, bus);

    util::trace!("{:?}", cpu);

    push_cpu(cpu.clone());

    increment_pc(cpu);

    interpret(cpu, bus);
}

#[inline]
pub fn fetch(cpu: &mut CPU, bus: &mut impl Bus) {
    cpu.ir = CPU::ldr(cpu.r(15) - 4, bus);
}

#[inline]
pub fn increment_pc(cpu: &mut CPU) {
    cpu.r[15] += 4;
}

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus) {
    let cond = cpu.ir.bits(31, 28);
    if cpu.check_condition(cond) {
        dispatch(cpu, bus);
    }
}

#[inline]
pub fn dispatch(cpu: &mut CPU, bus: &mut impl Bus) {
    let instr = cpu.ir;

    let b74 = || instr >> 6 & 0b10 | instr >> 4 & 0b01;

    // Data Processing / PSR Transfer / branch and exchange
    let data_process_psr_bx = |cpu: &mut CPU| {
        match instr.bits(24, 20) {
            0b10000 | 0b10100 | 0b10110 => psr_transfer::interpret(cpu, instr),
            0b10010 => {
                if b74() == 0 {
                    psr_transfer::interpret(cpu, instr)
                } else {
                    branch_exchange::interpret(cpu, instr)
                }
            }
            _ => data_processing::interpret(cpu, instr),
        };
    };

    match instr.bits(27, 25) {
        0b000 => {
            if b74() < 0b11 {
                data_process_psr_bx(cpu)
            } else {
                if instr.bits(6, 5) > 0 {
                    halfword_data_transfer::interpret(cpu, bus, instr)
                } else {
                    multiply_swap(cpu, bus, instr)
                }
            }
        }
        0b001 => match instr.bits(24, 20) {
            0b10110 | 0b10010 => psr_transfer::interpret(cpu, instr),
            _ => data_processing::interpret(cpu, instr),
        },
        0b010 | 0b011 => single_data_transfer::interpret(cpu, bus, instr),
        0b100 => block_data_transfer::interpret(cpu, bus, instr),
        0b101 => branch_long::interpret(cpu, instr),
        0b111 => cpu.software_interrupt(),
        _ => unimplemented!(),
    };
}

/// Multiply / Multiply Long / Single Data Swap
#[inline]
fn multiply_swap(cpu: &mut CPU, bus: &mut impl Bus, instr: u32) {
    match instr.bits(24, 20) {
        0b00000 | 0b00001 | 0b00010 | 0b00011 => multiply_accumulate::interpret(cpu, instr),
        0b01000 | 0b01001 | 0b01010 | 0b01011 | 0b01100 | 0b01101 | 0b01110 | 0b01111 => {
            multiply_long_accumulate::interpret(cpu, instr)
        }

        // Single data swap
        0b10000 | 0b10100 => single_data_swap::interpret(cpu, bus, instr),

        _ => unreachable!(),
    };
}
