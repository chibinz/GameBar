pub fn disassemble(opcode: u16) -> String {
    // decode the instructions using higher 5 bits
    let b15_11 = opcode >> 11;

    // use closures to inline calculation
    let offset5 = || opcode >> 6 & 0b00011111;
    let offset11 = || opcode & 0b0000011111111111;
    let offset8 = || opcode & 0b11111111;
    let offset7 = || opcode & 0b01111111;
    let sign7 = || opcode & 0b10000000;
    let rlist = || opcode & 0b11111111;
    let b11_8 = || opcode >> 8 & 0b00001111;
    let cond = || opcode >> 8 & 0b00001111;
    let b11_9 = || opcode >> 9 & 0b00000111;
    let b10_6 = || opcode >> 6 & 0b00011111;
    let b10_9 = || opcode >> 9 & 0b00000011;
    let rdb = || opcode >> 8 & 0b00000111;
    let rn = || opcode >> 6 & 0b00000111;
    let ro = || opcode >> 6 & 0b00000111;
    let rs = || opcode >> 3 & 0b00000111;
    let hs = || (opcode >> 3 & 0b00000111) + 8;
    let rb = || opcode >> 3 & 0b00000111;
    let rd = || opcode & 0b00000111;
    let hd = || (opcode & 0b00000111) + 8;

    // result of match statement is returned
    match b15_11 {
        0b00000 => format!("LSL R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00001 => format!("LSL R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00010 => format!("ASR R{}, R{}, #{}", rd(), rs(), offset5()),
        0b00011 => match b10_9() {
            0b00 => format!("ADD R{}, R{}, R{}", rd(), rs(), rn()),
            0b01 => format!("SUB R{}, R{}, R{}", rd(), rs(), rn()),
            0b10 => format!("ADD R{}, R{}, #{}", rd(), rs(), rn()),
            0b11 => format!("SUB R{}, R{}, #{}", rd(), rs(), rn()),
            _ => "undefined".to_string(),
        },
        0b00100 => format!("MOV R{}, #{}", rdb(), offset8()),
        0b00101 => format!("CMP R{}, #{}", rdb(), offset8()),
        0b00110 => format!("ADD R{}, #{}", rdb(), offset8()),
        0b00111 => format!("SUB R{}, #{}", rdb(), offset8()),
        0b01000 => match b10_6() {
            0b00000 => format!("AND R{}, R{}", rd(), rs()),
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
            0b10001 => format!("ADD R{}, R{}", rd(), hs()),
            0b10010 => format!("ADD R{}, R{}", hd(), rs()),
            0b10011 => format!("ADD R{}, R{}", hd(), hs()),
            0b10101 => format!("CMP R{}, R{}", rd(), hs()),
            0b10110 => format!("CMP R{}, R{}", hd(), rs()),
            0b10111 => format!("CMP R{}, R{}", hd(), hs()),
            0b11001 => format!("MOV R{}, R{}", rd(), hs()),
            0b11010 => format!("MOV R{}, R{}", hd(), rs()),
            0b11011 => format!("MOV R{}, R{}", hd(), hs()),
            0b11100 => format!("BX R{}", rs()),
            0b11101 => format!("BX R{}", hs()),
            _ => "undefined".to_string(),
        },
        0b01001 => format!("LDR R{}, [PC, #{}]", rdb(), offset8() << 2),
        0b01010 | 0b1011 => match b11_9() {
            0b000 => format!("STR R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b001 => format!("STRH R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b010 => format!("STRB R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b011 => format!("LDSB R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b100 => format!("LDR R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b101 => format!("LDRH R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b110 => format!("LDRB R{}, [R{}, R{}]", rd(), rb(), ro()),
            0b111 => format!("LDSH R{}, [R{}, R{}]", rd(), rb(), ro()),
            _ => "undefined".to_string(),
        },
        0b01100 => format!("STR R{}, [R{}, #{}]", rd(), rb(), offset5() << 2),
        0b01101 => format!("LDR R{}, [R{}, #{}]", rd(), rb(), offset5() << 2),
        0b01110 => format!("STRB R{}, [R{}, #{}]", rd(), rb(), offset5()),
        0b01111 => format!("LDRB R{}, [R{}, #{}]", rd(), rb(), offset5()),
        0b10000 => format!("STRH R{}, [R{}, #{}]", rd(), rb(), offset5() << 1),
        0b10001 => format!("LDRH R{}, [R{}, #{}]", rd(), rb(), offset5() << 1),
        0b10010 => format!("STR R{}, [SP, #{}]", rdb(), offset8() << 2),
        0b10011 => format!("LDR R{}, [SP, #{}]", rdb(), offset8() << 2),
        0b10100 => format!("ADD R{}, PC, #{}", rdb(), offset8() << 2),
        0b10101 => format!("ADD R{}, SP, #{}", rdb(), offset8() << 2),
        0b10110 | 0b10111 => {
            match b11_8() {
                // needs better implementation
                0b0000 => format!(
                    "{} SP #{}",
                    if sign7() == 0 { "ADD" } else { "SUB" },
                    offset7() << 2
                ),
                0b0100 => format!("PUSH R{{{:08b}}}", offset8()),
                0b0101 => format!("PUSH R{{{:08b}, LR}}", offset8()),
                0b1100 => format!("POP {{{:08b}}}", offset8()),
                0b1101 => format!("POP {{{:08b}, PC}}", offset8()),
                _ => "undefined".to_string(),
            }
        }
        0b11000 => format!("STMIA R{}!, {{{:08b}}}", rb(), rlist()),
        0b11001 => format!("LDMIA R{}!, {{{:08b}}}", rb(), rlist()),
        0b11010 | 0b11011 => {
            // TODO offset needs to be shifted
            match cond() {
                0b0000 => format!("BEQ #{}", (offset8() + 2) << 1),
                0b0001 => format!("BNE #{}", (offset8() + 2) << 1),
                0b0010 => format!("BCS #{}", (offset8() + 2) << 1),
                0b0011 => format!("BCC #{}", (offset8() + 2) << 1),
                0b0100 => format!("BMI #{}", (offset8() + 2) << 1),
                0b0101 => format!("BPL #{}", (offset8() + 2) << 1),
                0b0110 => format!("BVS #{}", (offset8() + 2) << 1),
                0b0111 => format!("BVC #{}", (offset8() + 2) << 1),
                0b1000 => format!("BHI #{}", (offset8() + 2) << 1),
                0b1001 => format!("BLS #{}", (offset8() + 2) << 1),
                0b1010 => format!("BGE #{}", (offset8() + 2) << 1),
                0b1011 => format!("BLT #{}", (offset8() + 2) << 1),
                0b1100 => format!("BGT #{}", (offset8() + 2) << 1),
                0b1101 => format!("BLE #{}", (offset8() + 2) << 1),
                0b1111 => format!("SWI #{}", offset8()),
                _ => "undefined".to_string(),
            }
        }
        0b11100 => format!("B #{}", offset11()),
        0b11110 => "BL-0".to_string(),
        0b11111 => "BL-1".to_string(),
        _ => "undefined".to_string(),
    }
}
