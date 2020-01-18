pub mod disassemble;
pub mod move_shifted;
pub mod add_subtract;
pub mod move_compare;
pub mod alu_operations;
pub mod hi_operations_bx;
pub mod pc_relative_load;
pub mod load_address;

use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn step(cpu: &mut CPU, memory: &mut Memory)
{
    fetch(cpu, memory);

    println!("{}", cpu);
    print!("{:08x}: {:04x} | {:016b} ", cpu.r[15] - 4, cpu.instruction, cpu.instruction);
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
pub fn dispatch(cpu: &mut CPU, _memory: &mut Memory)
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
        0b01001 => pc_relative_load::decode_execute(cpu, instruction),
        // 0b01010 | 0b1011 => 
        // {
        //     match b11_9()
        //     {
        //         0b000 => format!("STR R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b001 => format!("STRB R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b010 => format!("STRH R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b011 => format!("LDSB R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b100 => format!("LDR R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b101 => format!("LDRB R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b110 => format!("LDRH R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         0b111 => format!("LDSH R{}, [R{}, R{}]", rd(), rb(), ro()),
        //         _    => format!("undefined"),
        //     }
        // },
        // 0b01100 => format!("STR R{}, [R{}, #{}]",rd(), rb(), offset5() << 2),
        // 0b01101 => format!("LDR R{}, [R{}, #{}]",rd(), rb(), offset5() << 2),
        // 0b01110 => format!("STRB R{}, [R{}, #{}]",rd(), rb(), offset5()),
        // 0b01111 => format!("LDRB R{}, [R{}, #{}]",rd(), rb(), offset5()),
        // 0b10000 => format!("STRH R{}, [R{}, #{}]",rd(), rb(), offset5() << 1),
        // 0b10001 => format!("LDRH R{}, [R{}, #{}]",rd(), rb(), offset5() << 1),
        // 0b10010 => format!("STR R{}, [SP, #{}]", rdb(), offset8() << 2),
        // 0b10011 => format!("LDR R{}, [SP, #{}]", rdb(), offset8() << 2),
        0b10100 ..=
        0b10101 => load_address::decode_execute(cpu, instruction),
        // 0b10110 | 0b10111 => 
        // {
        //     match b11_8()
        //     {
        //         // needs better implementation
        //         0b0000 => format!("ADD SP #{}", if sign7() > 0 {offset7() as i16 * 2} else {-(offset7() as i16 * 2)}),
        //         0b0100 => format!("PUSH R{{{:08b}}}", offset8()),
        //         0b0101 => format!("PUSH R{{{:08b}, LR}}", offset8()),
        //         0b1100 => format!("POP {{{:08b}}}", offset8()),
        //         0b1101 => format!("POP {{{:08b}, PC}}", offset8()),
        //         _      => format!("undefined"),
        //     }
        // },
        // 0b11000 => format!("STMIA R{}!, {{{:08b}}}", rb(), rlist()),
        // 0b11001 => format!("LDMIA R{}!, {{{:08b}}}", rb(), rlist()),
        // 0b11010 | 0b11011 => 
        // {
        //     // TODO offset needs to be shifted
        //     match cond()
        //     {
        //         0b0000 => format!("BEQ #{}", (offset8() + 2) << 1),
        //         0b0001 => format!("BNE #{}", (offset8() + 2) << 1),
        //         0b0010 => format!("BCS #{}", (offset8() + 2) << 1),
        //         0b0011 => format!("BCC #{}", (offset8() + 2) << 1),
        //         0b0100 => format!("BMI #{}", (offset8() + 2) << 1),
        //         0b0101 => format!("BPL #{}", (offset8() + 2) << 1),
        //         0b0110 => format!("BVS #{}", (offset8() + 2) << 1),
        //         0b0111 => format!("BVC #{}", (offset8() + 2) << 1),
        //         0b1000 => format!("BHI #{}", (offset8() + 2) << 1),
        //         0b1001 => format!("BLS #{}", (offset8() + 2) << 1),
        //         0b1010 => format!("BGE #{}", (offset8() + 2) << 1),
        //         0b1011 => format!("BLT #{}", (offset8() + 2) << 1),
        //         0b1100 => format!("BGT #{}", (offset8() + 2) << 1),
        //         0b1101 => format!("BLE #{}", (offset8() + 2) << 1),
        //         0b1111 => format!("SWI #{}", offset8()),
        //         _      => format!("undefined"),
        //     }
        // },
        // 0b11100 => format!("B #{}", offset11()),
        // 0b11110 => format!("BL-0"),
        // 0b11111 => format!("BL-1"),
        _       => unimplemented!(),
    };
}