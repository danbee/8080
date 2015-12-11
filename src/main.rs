use std::io::{self, Read};

fn print_mnemonic(mnemonic: &str, bytes: &[u8]) {
    match bytes.len() {
        1 => println!("{:02x}         {}", bytes[0], mnemonic),
        2 => println!("{:02x} {:02x}      {:5} #0x{:02x}", bytes[0], bytes[1], mnemonic, bytes[1]),
        3 => println!("{:02x} {:02x} {:02x}   {:5} ${:02x}{:02x}", bytes[0], bytes[1], bytes[2], mnemonic, bytes[2], bytes[1]),

        _ => {},
    }
}

fn opcode(input: &Vec<u8>, counter: usize) -> usize {
    let size: usize;

    let opcode = input[counter];

    match opcode {
        // 00
        0x00 => { print_mnemonic("NOP", &input[counter..counter + 1]); size = 1 },
        0x01 => { print_mnemonic("LXI  B", &input[counter..counter + 3]); size = 3 },
        0x02 => { print_mnemonic("STAX B", &input[counter..counter + 1]); size = 1 },
        0x03 => { print_mnemonic("INX  B", &input[counter..counter + 1]); size = 1 },
        0x04 => { print_mnemonic("INR  B", &input[counter..counter + 1]); size = 1 },
        0x05 => { print_mnemonic("DCR  B", &input[counter..counter + 1]); size = 1 },
        0x06 => { print_mnemonic("MVI B,", &input[counter..counter + 2]); size = 2; },
        0x07 => { print_mnemonic("RLC", &input[counter..counter + 1]); size = 1 },

        // 08
        0x08 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x09 => { print_mnemonic("DAD  B", &input[counter..counter + 1]); size = 1 },
        0x0a => { print_mnemonic("LDAX B", &input[counter..counter + 1]); size = 1 },
        0x0b => { print_mnemonic("DCX  B", &input[counter..counter + 1]); size = 1 },
        0x0c => { print_mnemonic("INR  C", &input[counter..counter + 1]); size = 1 },
        0x0d => { print_mnemonic("DCR  C", &input[counter..counter + 1]); size = 1 },
        0x0e => { print_mnemonic("MVI  C", &input[counter..counter + 2]); size = 2 },
        0x0f => { print_mnemonic("RRC", &input[counter..counter + 1]); size = 1 },

        // 10
        0x10 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x11 => { print_mnemonic("LXI  D", &input[counter..counter + 3]); size = 3 },
        0x12 => { print_mnemonic("STAX D", &input[counter..counter + 1]); size = 1 },
        0x13 => { print_mnemonic("INX  D", &input[counter..counter + 1]); size = 1 },
        0x14 => { print_mnemonic("INR  D", &input[counter..counter + 1]); size = 1 },
        0x15 => { print_mnemonic("DCR  D", &input[counter..counter + 1]); size = 1 },
        0x16 => { print_mnemonic("MVI  D", &input[counter..counter + 2]); size = 2 },
        0x17 => { print_mnemonic("RAL", &input[counter..counter + 1]); size = 1 },

        // 18
        0x18 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x19 => { print_mnemonic("DAD  D", &input[counter..counter + 1]); size = 1 },
        0x1a => { print_mnemonic("LDAX D", &input[counter..counter + 1]); size = 1 },
        0x1b => { print_mnemonic("DCX  D", &input[counter..counter + 1]); size = 1 },
        0x1c => { print_mnemonic("INR  E", &input[counter..counter + 1]); size = 1 },
        0x1d => { print_mnemonic("DCR  E", &input[counter..counter + 1]); size = 1 },
        0x1e => { print_mnemonic("MVI  E", &input[counter..counter + 2]); size = 2 },
        0x1f => { print_mnemonic("RAR", &input[counter..counter + 1]); size = 1 },

        // 20
        0x20 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x21 => { print_mnemonic("LXI  H", &input[counter..counter + 3]); size = 3 },
        0x22 => { print_mnemonic("SHLD", &input[counter..counter + 3]); size = 3 },
        0x23 => { print_mnemonic("INX  H", &input[counter..counter + 1]); size = 1 },
        0x24 => { print_mnemonic("INR  H", &input[counter..counter + 1]); size = 1 },
        0x25 => { print_mnemonic("DCR  H", &input[counter..counter + 1]); size = 1 },
        0x26 => { print_mnemonic("MVI  H", &input[counter..counter + 2]); size = 2 },
        0x27 => { print_mnemonic("DDA", &input[counter..counter + 1]); size = 1 },

        // 28
        0x28 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x29 => { print_mnemonic("DAD  H", &input[counter..counter + 1]); size = 1 },
        0x2a => { print_mnemonic("LHLD", &input[counter..counter + 3]); size = 3 },
        0x2b => { print_mnemonic("DCX  H", &input[counter..counter + 1]); size = 1 },
        0x2c => { print_mnemonic("INR  L", &input[counter..counter + 1]); size = 1 },
        0x2d => { print_mnemonic("DCR  L", &input[counter..counter + 1]); size = 1 },
        0x2e => { print_mnemonic("MVI  L", &input[counter..counter + 2]); size = 2 },
        0x2f => { print_mnemonic("CMA", &input[counter..counter + 1]); size = 1 },

        // 30
        0x30 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x31 => {
            println!("LXI  SP,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x32 => {
            println!("STA  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x33 => { print_mnemonic("INX  SP", &input[counter..counter + 1]); size = 1 },
        0x34 => { print_mnemonic("INR  M", &input[counter..counter + 1]); size = 1 },
        0x35 => { print_mnemonic("DCR  M", &input[counter..counter + 1]); size = 1 },
        0x36 => { print_mnemonic("MVI  M,", &input[counter..counter + 2]); size = 2 },
        0x37 => { print_mnemonic("STC", &input[counter..counter + 1]); size = 1 },

        // 38
        0x38 => { print_mnemonic("*NOP", &input[counter..counter + 1]); size = 1 },
        0x39 => { print_mnemonic("DAD  SP", &input[counter..counter + 1]); size = 1 },
        0x3a => {
            println!("LDA  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x3b => { print_mnemonic("DCX  SP", &input[counter..counter + 1]); size = 1 },
        0x3c => { print_mnemonic("INR  A", &input[counter..counter + 1]); size = 1 },
        0x3d => { print_mnemonic("DCR  A", &input[counter..counter + 1]); size = 1 },
        0x3e => { print_mnemonic("MVI  A,", &input[counter..counter + 2]); size = 2 },
        0x3f => { print_mnemonic("CMC", &input[counter..counter + 1]); size = 1 },

        // 40
        0x40 => { print_mnemonic("MOV  B,B", &input[counter..counter + 1]); size = 1 },
        0x41 => { print_mnemonic("MOV  B,C", &input[counter..counter + 1]); size = 1 },
        0x42 => { print_mnemonic("MOV  B,D", &input[counter..counter + 1]); size = 1 },
        0x43 => { print_mnemonic("MOV  B,E", &input[counter..counter + 1]); size = 1 },
        0x44 => { print_mnemonic("MOV  B,H", &input[counter..counter + 1]); size = 1 },
        0x45 => { print_mnemonic("MOV  B,L", &input[counter..counter + 1]); size = 1 },
        0x46 => { print_mnemonic("MOV  B,M", &input[counter..counter + 1]); size = 1 },
        0x47 => { print_mnemonic("MOV  B,A", &input[counter..counter + 1]); size = 1 },

        // 48
        0x48 => { print_mnemonic("MOV  C,B", &input[counter..counter + 1]); size = 1 },
        0x49 => { print_mnemonic("MOV  C,C", &input[counter..counter + 1]); size = 1 },
        0x4a => { print_mnemonic("MOV  C,D", &input[counter..counter + 1]); size = 1 },
        0x4b => { print_mnemonic("MOV  C,E", &input[counter..counter + 1]); size = 1 },
        0x4c => { print_mnemonic("MOV  C,H", &input[counter..counter + 1]); size = 1 },
        0x4d => { print_mnemonic("MOV  C,L", &input[counter..counter + 1]); size = 1 },
        0x4e => { print_mnemonic("MOV  C,M", &input[counter..counter + 1]); size = 1 },
        0x4f => { print_mnemonic("MOV  C,A", &input[counter..counter + 1]); size = 1 },

        // 50
        0x50 => { print_mnemonic("MOV  D,B", &input[counter..counter + 1]); size = 1 },
        0x51 => { print_mnemonic("MOV  D,C", &input[counter..counter + 1]); size = 1 },
        0x52 => { print_mnemonic("MOV  D,D", &input[counter..counter + 1]); size = 1 },
        0x53 => { print_mnemonic("MOV  D,E", &input[counter..counter + 1]); size = 1 },
        0x54 => { print_mnemonic("MOV  D,H", &input[counter..counter + 1]); size = 1 },
        0x55 => { print_mnemonic("MOV  D,L", &input[counter..counter + 1]); size = 1 },
        0x56 => { print_mnemonic("MOV  D,M", &input[counter..counter + 1]); size = 1 },
        0x57 => { print_mnemonic("MOV  D,A", &input[counter..counter + 1]); size = 1 },

        // 58
        0x58 => { print_mnemonic("MOV  E,B", &input[counter..counter + 1]); size = 1 },
        0x59 => { print_mnemonic("MOV  E,C", &input[counter..counter + 1]); size = 1 },
        0x5a => { print_mnemonic("MOV  E,D", &input[counter..counter + 1]); size = 1 },
        0x5b => { print_mnemonic("MOV  E,E", &input[counter..counter + 1]); size = 1 },
        0x5c => { print_mnemonic("MOV  E,H", &input[counter..counter + 1]); size = 1 },
        0x5d => { print_mnemonic("MOV  E,L", &input[counter..counter + 1]); size = 1 },
        0x5e => { print_mnemonic("MOV  E,M", &input[counter..counter + 1]); size = 1 },
        0x5f => { print_mnemonic("MOV  E,A", &input[counter..counter + 1]); size = 1 },

        // 60
        0x60 => { print_mnemonic("MOV  H,B", &input[counter..counter + 1]); size = 1 },
        0x61 => { print_mnemonic("MOV  H,C", &input[counter..counter + 1]); size = 1 },
        0x62 => { print_mnemonic("MOV  H,D", &input[counter..counter + 1]); size = 1 },
        0x63 => { print_mnemonic("MOV  H,E", &input[counter..counter + 1]); size = 1 },
        0x64 => { print_mnemonic("MOV  H,H", &input[counter..counter + 1]); size = 1 },
        0x65 => { print_mnemonic("MOV  H,L", &input[counter..counter + 1]); size = 1 },
        0x66 => { print_mnemonic("MOV  H,M", &input[counter..counter + 1]); size = 1 },
        0x67 => { print_mnemonic("MOV  H,A", &input[counter..counter + 1]); size = 1 },

        // 68
        0x68 => { print_mnemonic("MOV  L,B", &input[counter..counter + 1]); size = 1 },
        0x69 => { print_mnemonic("MOV  L,C", &input[counter..counter + 1]); size = 1 },
        0x6a => { print_mnemonic("MOV  L,D", &input[counter..counter + 1]); size = 1 },
        0x6b => { print_mnemonic("MOV  L,E", &input[counter..counter + 1]); size = 1 },
        0x6c => { print_mnemonic("MOV  L,H", &input[counter..counter + 1]); size = 1 },
        0x6d => { print_mnemonic("MOV  L,L", &input[counter..counter + 1]); size = 1 },
        0x6e => { print_mnemonic("MOV  L,M", &input[counter..counter + 1]); size = 1 },
        0x6f => { print_mnemonic("MOV  L,A", &input[counter..counter + 1]); size = 1 },

        // 70
        0x70 => { print_mnemonic("MOV  M,B", &input[counter..counter + 1]); size = 1 },
        0x71 => { print_mnemonic("MOV  M,C", &input[counter..counter + 1]); size = 1 },
        0x72 => { print_mnemonic("MOV  M,D", &input[counter..counter + 1]); size = 1 },
        0x73 => { print_mnemonic("MOV  M,E", &input[counter..counter + 1]); size = 1 },
        0x74 => { print_mnemonic("MOV  M,H", &input[counter..counter + 1]); size = 1 },
        0x75 => { print_mnemonic("MOV  M,L", &input[counter..counter + 1]); size = 1 },
        0x76 => { print_mnemonic("HLT", &input[counter..counter + 1]); size = 1 },
        0x77 => { print_mnemonic("MOV  M,A", &input[counter..counter + 1]); size = 1 },

        // 78
        0x78 => { print_mnemonic("MOV  A,B", &input[counter..counter + 1]); size = 1 },
        0x79 => { print_mnemonic("MOV  A,C", &input[counter..counter + 1]); size = 1 },
        0x7a => { print_mnemonic("MOV  A,D", &input[counter..counter + 1]); size = 1 },
        0x7b => { print_mnemonic("MOV  A,E", &input[counter..counter + 1]); size = 1 },
        0x7c => { print_mnemonic("MOV  A,H", &input[counter..counter + 1]); size = 1 },
        0x7d => { print_mnemonic("MOV  A,L", &input[counter..counter + 1]); size = 1 },
        0x7e => { print_mnemonic("MOV  A,M", &input[counter..counter + 1]); size = 1 },
        0x7f => { print_mnemonic("MOV  A,A", &input[counter..counter + 1]); size = 1 },

        // 80
        0x80 => { print_mnemonic("ADD  B", &input[counter..counter + 1]); size = 1 },
        0x81 => { print_mnemonic("ADD  C", &input[counter..counter + 1]); size = 1 },
        0x82 => { print_mnemonic("ADD  D", &input[counter..counter + 1]); size = 1 },
        0x83 => { print_mnemonic("ADD  E", &input[counter..counter + 1]); size = 1 },
        0x84 => { print_mnemonic("ADD  H", &input[counter..counter + 1]); size = 1 },
        0x85 => { print_mnemonic("ADD  L", &input[counter..counter + 1]); size = 1 },
        0x86 => { print_mnemonic("ADD  M", &input[counter..counter + 1]); size = 1 },
        0x87 => { print_mnemonic("ADD  A", &input[counter..counter + 1]); size = 1 },

        // 88
        0x88 => { print_mnemonic("ADC  B", &input[counter..counter + 1]); size = 1 },
        0x89 => { print_mnemonic("ADC  C", &input[counter..counter + 1]); size = 1 },
        0x8a => { print_mnemonic("ADC  D", &input[counter..counter + 1]); size = 1 },
        0x8b => { print_mnemonic("ADC  E", &input[counter..counter + 1]); size = 1 },
        0x8c => { print_mnemonic("ADC  H", &input[counter..counter + 1]); size = 1 },
        0x8d => { print_mnemonic("ADC  L", &input[counter..counter + 1]); size = 1 },
        0x8e => { print_mnemonic("ADC  M", &input[counter..counter + 1]); size = 1 },
        0x8f => { print_mnemonic("ADC  A", &input[counter..counter + 1]); size = 1 },

        // 90
        0x90 => { print_mnemonic("SUB  B", &input[counter..counter + 1]); size = 1 },
        0x91 => { print_mnemonic("SUB  C", &input[counter..counter + 1]); size = 1 },
        0x92 => { print_mnemonic("SUB  D", &input[counter..counter + 1]); size = 1 },
        0x93 => { print_mnemonic("SUB  E", &input[counter..counter + 1]); size = 1 },
        0x94 => { print_mnemonic("SUB  H", &input[counter..counter + 1]); size = 1 },
        0x95 => { print_mnemonic("SUB  L", &input[counter..counter + 1]); size = 1 },
        0x96 => { print_mnemonic("SUB  M", &input[counter..counter + 1]); size = 1 },
        0x97 => { print_mnemonic("SUB  A", &input[counter..counter + 1]); size = 1 },

        // 98
        0x98 => { print_mnemonic("SBB  B", &input[counter..counter + 1]); size = 1 },
        0x99 => { print_mnemonic("SBB  C", &input[counter..counter + 1]); size = 1 },
        0x9a => { print_mnemonic("SBB  D", &input[counter..counter + 1]); size = 1 },
        0x9b => { print_mnemonic("SBB  E", &input[counter..counter + 1]); size = 1 },
        0x9c => { print_mnemonic("SBB  H", &input[counter..counter + 1]); size = 1 },
        0x9d => { print_mnemonic("SBB  L", &input[counter..counter + 1]); size = 1 },
        0x9e => { print_mnemonic("SBB  M", &input[counter..counter + 1]); size = 1 },
        0x9f => { print_mnemonic("SBB  A", &input[counter..counter + 1]); size = 1 },

        // a0
        0xa0 => { print_mnemonic("ANA  B", &input[counter..counter + 1]); size = 1 },
        0xa1 => { print_mnemonic("ANA  C", &input[counter..counter + 1]); size = 1 },
        0xa2 => { print_mnemonic("ANA  D", &input[counter..counter + 1]); size = 1 },
        0xa3 => { print_mnemonic("ANA  E", &input[counter..counter + 1]); size = 1 },
        0xa4 => { print_mnemonic("ANA  H", &input[counter..counter + 1]); size = 1 },
        0xa5 => { print_mnemonic("ANA  L", &input[counter..counter + 1]); size = 1 },
        0xa6 => { print_mnemonic("ANA  M", &input[counter..counter + 1]); size = 1 },
        0xa7 => { print_mnemonic("ANA  A", &input[counter..counter + 1]); size = 1 },

        // a8
        0xa8 => { print_mnemonic("XRA  B", &input[counter..counter + 1]); size = 1 },
        0xa9 => { print_mnemonic("XRA  C", &input[counter..counter + 1]); size = 1 },
        0xaa => { print_mnemonic("XRA  D", &input[counter..counter + 1]); size = 1 },
        0xab => { print_mnemonic("XRA  E", &input[counter..counter + 1]); size = 1 },
        0xac => { print_mnemonic("XRA  H", &input[counter..counter + 1]); size = 1 },
        0xad => { print_mnemonic("XRA  L", &input[counter..counter + 1]); size = 1 },
        0xae => { print_mnemonic("XRA  M", &input[counter..counter + 1]); size = 1 },
        0xaf => { print_mnemonic("XRA  A", &input[counter..counter + 1]); size = 1 },

        // b0
        0xb0 => { print_mnemonic("ORA  B", &input[counter..counter + 1]); size = 1 },
        0xb1 => { print_mnemonic("ORA  C", &input[counter..counter + 1]); size = 1 },
        0xb2 => { print_mnemonic("ORA  D", &input[counter..counter + 1]); size = 1 },
        0xb3 => { print_mnemonic("ORA  E", &input[counter..counter + 1]); size = 1 },
        0xb4 => { print_mnemonic("ORA  H", &input[counter..counter + 1]); size = 1 },
        0xb5 => { print_mnemonic("ORA  L", &input[counter..counter + 1]); size = 1 },
        0xb6 => { print_mnemonic("ORA  M", &input[counter..counter + 1]); size = 1 },
        0xb7 => { print_mnemonic("ORA  A", &input[counter..counter + 1]); size = 1 },

        // b8
        0xb8 => { print_mnemonic("CMP  B", &input[counter..counter + 1]); size = 1 },
        0xb9 => { print_mnemonic("CMP  C", &input[counter..counter + 1]); size = 1 },
        0xba => { print_mnemonic("CMP  D", &input[counter..counter + 1]); size = 1 },
        0xbb => { print_mnemonic("CMP  E", &input[counter..counter + 1]); size = 1 },
        0xbc => { print_mnemonic("CMP  H", &input[counter..counter + 1]); size = 1 },
        0xbd => { print_mnemonic("CMP  L", &input[counter..counter + 1]); size = 1 },
        0xbe => { print_mnemonic("CMP  M", &input[counter..counter + 1]); size = 1 },
        0xbf => { print_mnemonic("CMP  A", &input[counter..counter + 1]); size = 1 },

        // c0
        0xc0 => { print_mnemonic("RNZ", &input[counter..counter + 1]); size = 1 },
        0xc1 => { print_mnemonic("POP  B", &input[counter..counter + 1]); size = 1 },
        0xc2 => {
            println!("JNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc3 => { print_mnemonic("JMP", &input[counter..counter + 3]); size = 3 },
        0xc4 => {
            println!("CNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc5 => { print_mnemonic("PUSH B", &input[counter..counter + 1]); size = 1 },
        0xc6 => { print_mnemonic("ADI", &input[counter..counter + 2]); size = 2 },
        0xc7 => { print_mnemonic("RST  0", &input[counter..counter + 1]); size = 1 },

        // c8
        0xc8 => { print_mnemonic("RZ", &input[counter..counter + 1]); size = 1 },
        0xc9 => { print_mnemonic("RET", &input[counter..counter + 1]); size = 1 },
        0xca => {
            println!("JZ   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xcb => {
            println!("*JMP ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xcc => {
            println!("CZ   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xcd => {
            println!("CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xce => { print_mnemonic("ACI", &input[counter..counter + 2]); size = 2 },
        0xcf => { print_mnemonic("RST  1", &input[counter..counter + 1]); size = 1 },

        // d0
        0xd0 => { print_mnemonic("RNC", &input[counter..counter + 1]); size = 1 },
        0xd1 => { print_mnemonic("POP  D", &input[counter..counter + 1]); size = 1 },
        0xd2 => {
            println!("JNC  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xd3 => { print_mnemonic("OUT", &input[counter..counter + 2]); size = 2 },
        0xd4 => {
            println!("CNC  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xd5 => { print_mnemonic("PUSH D", &input[counter..counter + 1]); size = 1 },
        0xd6 => { print_mnemonic("SUI", &input[counter..counter + 2]); size = 2 },
        0xd7 => { print_mnemonic("RST  2", &input[counter..counter + 1]); size = 1 },

        // d8
        0xd8 => { print_mnemonic("RC", &input[counter..counter + 1]); size = 1 },
        0xd9 => { print_mnemonic("*RET", &input[counter..counter + 1]); size = 1 },
        0xda => {
            println!("JC   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xdb => { print_mnemonic("IN", &input[counter..counter + 2]); size = 2 },
        0xdc => {
            println!("CC   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xdd => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xde => { print_mnemonic("SBI", &input[counter..counter + 2]); size = 2 },
        0xdf => { print_mnemonic("RST  3", &input[counter..counter + 1]); size = 1 },

        // e0
        0xe0 => { print_mnemonic("RPO", &input[counter..counter + 1]); size = 1 },
        0xe1 => { print_mnemonic("POP  H", &input[counter..counter + 1]); size = 1 },
        0xe2 => {
            println!("JPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe3 => { print_mnemonic("XTHL", &input[counter..counter + 1]); size = 1 },
        0xe4 => {
            println!("CPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe5 => { print_mnemonic("PUSH H", &input[counter..counter + 1]); size = 1 },
        0xe6 => { print_mnemonic("ANI", &input[counter..counter + 2]); size = 2 },
        0xe7 => { print_mnemonic("RST  4", &input[counter..counter + 1]); size = 1 },

        // e8
        0xe8 => { print_mnemonic("RPE", &input[counter..counter + 1]); size = 1 },
        0xe9 => { print_mnemonic("PCHL", &input[counter..counter + 1]); size = 1 },
        0xea => {
            println!("JPE  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xeb => { print_mnemonic("XCHG", &input[counter..counter + 1]); size = 1 },
        0xec => {
            println!("CPE  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xed => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xee => { print_mnemonic("XRI", &input[counter..counter + 2]); size = 2 },
        0xef => { print_mnemonic("RST  5", &input[counter..counter + 1]); size = 1 },

        // f0
        0xf0 => { print_mnemonic("RP", &input[counter..counter + 1]); size = 1 },
        0xf1 => { print_mnemonic("POP  PSW", &input[counter..counter + 1]); size = 1 },
        0xf2 => {
            println!("JP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf3 => { print_mnemonic("DI", &input[counter..counter + 1]); size = 1 },
        0xf4 => {
            println!("CP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf5 => { print_mnemonic("PUSH PSW", &input[counter..counter + 1]); size = 1 },
        0xf6 => { print_mnemonic("ORI", &input[counter..counter + 2]); size = 2 },
        0xf7 => { print_mnemonic("RST  6", &input[counter..counter + 1]); size = 1 },

        // f8
        0xf8 => { print_mnemonic("RM", &input[counter..counter + 1]); size = 1 },
        0xf9 => { print_mnemonic("SPHL", &input[counter..counter + 1]); size = 1 },
        0xfa => {
            println!("JM   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfb => { print_mnemonic("EI", &input[counter..counter + 1]); size = 1 },
        0xfc => {
            println!("CM   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfd => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfe => { print_mnemonic("CPI ", &input[counter..counter + 2]); size = 2 },
        0xff => { print_mnemonic("RST  7", &input[counter..counter + 1]); size = 1 },

        _  => { println!("-    {:02x}", opcode); size = 1 },
    }

    size
}

fn main() {
    let mut input: Vec<u8> = vec![];

    let mut counter: usize = 0;

    let mut oplength: usize;

    io::stdin()
        .read_to_end(&mut input)
        .unwrap();

    // for byte in input.iter() {
        // println!("{}", byte);
    // };

    while counter < input.len() {
        // get the instruction
        print!("{:04x}   ", counter);
        oplength = opcode(&input, counter);
        counter += oplength;
    }
}
