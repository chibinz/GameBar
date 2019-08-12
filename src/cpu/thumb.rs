pub fn disassemble(opcode: u16) -> String
{
    // decode the instructions using higher 5 bits
    let b15_11: u8 = (opcode >> 11) as u8;


    // use closures to inline calculation
    let offset5 = || -> u8 {(opcode >> 6 & 0b00011111) as u8};
    let offset8 = || -> u8 {opcode as u8};
    let b10_6 = || -> u8 {(opcode >> 6 & 0b00011111) as u8};
    let b10_9 = || -> u8 {(opcode >> 9 & 0b00000011) as u8};
    let rdb = || -> u8 {(opcode >> 8 & 0b00000111) as u8};
    let rn = || -> u8 {(opcode >> 6 & 0b00000111) as u8};
    let rs = || -> u8 {(opcode >> 3 & 0b00000111) as u8};
    let rd = || -> u8 {(opcode & 0b00000111) as u8};

    // result of match statement is returned
    match b15_11 
    {
        0b00000 => format!("LSL R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00001 => format!("LSL R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00010 => format!("ASR R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00011 =>
        {
            match b10_9()
            {
                0b00 => format!("ADD R{}, R{}, R{}", rd(), rs(), rn()),
                0b01 => format!("SUB R{}, R{}, R{}", rd(), rs(), rn()),
                0b10 => format!("ADD R{}, R{}, #{}", rd(), rs(), rn()),
                0b11 => format!("SUB R{}, R{}, #{}", rd(), rs(), rn()),
                _    => format!("undefined"),
            }
        },
        0b00100 => format!("MOV R{}, #{}", rdb(), offset8()),
        0b00101 => format!("CMP R{}, #{}", rdb(), offset8()),
        0b00110 => format!("ADD R{}, #{}", rdb(), offset8()),
        0b00111 => format!("SUB R{}, #{}", rdb(), offset8()),
        0b01000 =>
        {
            match b10_6()
            {
                0b00000 => format!("ADD R{}, R{}", rd(), rs()),
                0b00001 => format!("EOR R{}, R{}", rd(), rs()),
                0b00010 => format!("LSL R{}, R{}", rd(), rs()),
                0b00011 => format!("LSR R{}, R{}", rd(), rs()),
                0b00100 => format!("ASR R{}, R{}", rd(), rs()),
                0b00101 => format!("ADC R{}, R{}", rd(), rs()),
                0b00110 => format!("SBC R{}, R{}", rd(), rs()),
                0b00111 => format!("ROR R{}, R{}", rd(), rs()),
                0b01000 => format!("TST R{}, R{}", rd(), rs()),
                0b01001 => format!("NEG R{}, R{}", rd(), rs()),
                0b01010 => format!("CMP R{}, R{}", rd(), rs()),
                0b01011 => format!("CMN R{}, R{}", rd(), rs()),
                0b01100 => format!("ORR R{}, R{}", rd(), rs()),
                0b01101 => format!("MUL R{}, R{}", rd(), rs()),
                0b01110 => format!("BIC R{}, R{}", rd(), rs()),
                0b01111 => format!("MVN R{}, R{}", rd(), rs()),
                0b10001 => format!("ADD R{}, R{}", rd(), rs()), 
                0b10010 => format!("ADD R{}, R{}", rd(), rs()), 
                0b10011 => format!("ADD R{}, R{}", rd(), rs()), 
                0b10101 => format!("CMP R{}, R{}", rd(), rs()), 
                0b10110 => format!("CMP R{}, R{}", rd(), rs()), 
                0b10111 => format!("CMP R{}, R{}", rd(), rs()), 
                0b11001 => format!("MOV R{}, R{}", rd(), rs()), 
                0b11010 => format!("MOV R{}, R{}", rd(), rs()), 
                0b11011 => format!("MOV R{}, R{}", rd(), rs()), 
                0b11100 => format!("BX R{}", rs()), 
                0b11101 => format!("BX R{}", rs()), 
                _       => format!("undefined"),
            }
        },
        0b01001 => format!("LDR PC"),
        0b01010 => format!("STR/STRB/STRH/LDSB"),
        0b01011 => format!("LDR/LDRB/LDRH/LDSH"),
        0b01100 => format!("STR imm"),
        0b01101 => format!("STRB imm"),
        0b01110 => format!("LDR imm"),
        0b01111 => format!("LDRB imm"),
        0b10000 => format!("STRH imm"),
        0b10001 => format!("LDRH imm"),
        0b10010 => format!("STR SP"),
        0b10011 => format!("LDR SP"),
        0b10100 => format!("ADD Rd, PC"),
        0b10101 => format!("ADD Rd, SP"),
        0b10110 => format!("ADD SP imm/PUSH"),
        0b10111 => format!("POP"),
        0b11000 => format!("STMIA"),
        0b11001 => format!("LDMIA"),
        0b11010 => format!("BEQ-BVC"),
        0b11011 => format!("BHI-BLE/SWI"),
        0b11100 => format!("B"),
        0b11110 => format!("BL-0"),
        0b11111 => format!("BL-1"),
        _       => format!("undefined"),
    }
}