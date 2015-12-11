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
        0x01 => {
            println!("LXI  B,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x02 => { print_mneomonic("STAX B", $input[counter..counter + 1]); size = 1 }, },
        0x03 => { print_mneomonic("INX  B", $input[counter..counter + 1]); size = 1 }, },
        0x04 => { print_mneomonic("INR  B", $input[counter..counter + 1]); size = 1 }, },
        0x05 => { print_mneomonic("DCR  B", $input[counter..counter + 1]); size = 1 }, },
        0x06 => { print_mnemonic("MVI B,", &input[counter..counter + 2]); size = 2; },
        0x07 => { print_mneomonic("RLC", $input[counter..counter + 1]); size = 1 }, },

        // 08
        0x08 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x09 => { print_mneomonic("DAD  B", $input[counter..counter + 1]); size = 1 }, },
        0x0a => { print_mneomonic("LDAX B", $input[counter..counter + 1]); size = 1 }, },
        0x0b => { print_mneomonic("DCX  B", $input[counter..counter + 1]); size = 1 }, },
        0x0c => { print_mneomonic("INR  C", $input[counter..counter + 1]); size = 1 }, },
        0x0d => { print_mneomonic("DCR  C", $input[counter..counter + 1]); size = 1 }, },
        0x0e => {
            println!("MVI  C,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x0f => { print_mneomonic("RRC", $input[counter..counter + 1]); size = 1 }, },

        // 10
        0x10 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x11 => {
            println!("LXI  D,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x12 => { print_mneomonic("STAX D", $input[counter..counter + 1]); size = 1 }, },
        0x13 => { print_mneomonic("INX  D", $input[counter..counter + 1]); size = 1 }, },
        0x14 => { print_mneomonic("INR  D", $input[counter..counter + 1]); size = 1 }, },
        0x15 => { print_mneomonic("DCR  D", $input[counter..counter + 1]); size = 1 }, },
        0x16 => {
            println!("MVI  D,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x17 => { print_mneomonic("RAL", $input[counter..counter + 1]); size = 1 }, },

        // 18
        0x18 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x19 => { print_mneomonic("DAD  D", $input[counter..counter + 1]); size = 1 }, },
        0x1a => { print_mneomonic("LDAX D", $input[counter..counter + 1]); size = 1 }, },
        0x1b => { print_mneomonic("DCX  D", $input[counter..counter + 1]); size = 1 }, },
        0x1c => { print_mneomonic("INR  E", $input[counter..counter + 1]); size = 1 }, },
        0x1d => { print_mneomonic("DCR  E", $input[counter..counter + 1]); size = 1 }, },
        0x1e => {
            println!("MVI  E,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x1f => { print_mneomonic("RAR", $input[counter..counter + 1]); size = 1 }, },

        // 20
        0x20 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x21 => {
            println!("LXI  H,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x22 => {
            println!("SHLD ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x23 => { print_mneomonic("INX  H", $input[counter..counter + 1]); size = 1 }, },
        0x24 => { print_mneomonic("INR  H", $input[counter..counter + 1]); size = 1 }, },
        0x25 => { print_mneomonic("DCR  H", $input[counter..counter + 1]); size = 1 }, },
        0x26 => {
            println!("MVI  H,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x27 => { print_mneomonic("DDA", $input[counter..counter + 1]); size = 1 }, },

        // 28
        0x28 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x29 => { print_mneomonic("DAD  H", $input[counter..counter + 1]); size = 1 }, },
        0x2a => {
            println!("LHLD ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x2b => { print_mneomonic("DCX  H", $input[counter..counter + 1]); size = 1 }, },
        0x2c => { print_mneomonic("INR  L", $input[counter..counter + 1]); size = 1 }, },
        0x2d => { print_mneomonic("DCR  L", $input[counter..counter + 1]); size = 1 }, },
        0x2e => {
            println!("MVI  L,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x2f => { print_mneomonic("CMA", $input[counter..counter + 1]); size = 1 }, },

        // 30
        0x30 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x31 => {
            println!("LXI  SP,${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x32 => {
            println!("STA  ${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x33 => { print_mneomonic("INX  SP", $input[counter..counter + 1]); size = 1 }, },
        0x34 => { print_mneomonic("INR  M", $input[counter..counter + 1]); size = 1 }, },
        0x35 => { print_mneomonic("DCR  M", $input[counter..counter + 1]); size = 1 }, },
        0x36 => {
            println!("MVI  M,#0x{:03x}", input[counter + 1]);
            size = 3;
        },
        0x37 => { print_mneomonic("STC", $input[counter..counter + 1]); size = 1 }, },

        // 38
        0x38 => { print_mneomonic("*NOP", $input[counter..counter + 1]); size = 1 }, },
        0x39 => { print_mneomonic("DAD  SP", $input[counter..counter + 1]); size = 1 }, },
        0x3a => {
            println!("LDA  ${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x3b => { print_mneomonic("DCX  SP", $input[counter..counter + 1]); size = 1 }, },
        0x3c => { print_mneomonic("INR  A", $input[counter..counter + 1]); size = 1 }, },
        0x3d => { print_mneomonic("DCR  A", $input[counter..counter + 1]); size = 1 }, },
        0x3e => {
            println!("MVI  A,#0x{:03x}", input[counter + 1]);
            size = 3;
        },
        0x3f => { print_mneomonic("CMC", $input[counter..counter + 1]); size = 1 }, },

        // 40
        0x40 => { print_mneomonic("MOV  B,B", $input[counter..counter + 1]); size = 1 }, },
        0x41 => { print_mneomonic("MOV  B,C", $input[counter..counter + 1]); size = 1 }, },
        0x42 => { print_mneomonic("MOV  B,D", $input[counter..counter + 1]); size = 1 }, },
        0x43 => { print_mneomonic("MOV  B,E", $input[counter..counter + 1]); size = 1 }, },
        0x44 => { print_mneomonic("MOV  B,H", $input[counter..counter + 1]); size = 1 }, },
        0x45 => { print_mneomonic("MOV  B,L", $input[counter..counter + 1]); size = 1 }, },
        0x46 => { print_mneomonic("MOV  B,M", $input[counter..counter + 1]); size = 1 }, },
        0x47 => { print_mneomonic("MOV  B,A", $input[counter..counter + 1]); size = 1 }, },

        // 48
        0x48 => { print_mneomonic("MOV  C,B", $input[counter..counter + 1]); size = 1 }, },
        0x49 => { print_mneomonic("MOV  C,C", $input[counter..counter + 1]); size = 1 }, },
        0x4a => { print_mneomonic("MOV  C,D", $input[counter..counter + 1]); size = 1 }, },
        0x4b => { print_mneomonic("MOV  C,E", $input[counter..counter + 1]); size = 1 }, },
        0x4c => { print_mneomonic("MOV  C,H", $input[counter..counter + 1]); size = 1 }, },
        0x4d => { print_mneomonic("MOV  C,L", $input[counter..counter + 1]); size = 1 }, },
        0x4e => { print_mneomonic("MOV  C,M", $input[counter..counter + 1]); size = 1 }, },
        0x4f => { print_mneomonic("MOV  C,A", $input[counter..counter + 1]); size = 1 }, },

        // 50
        0x50 => { print_mneomonic("MOV  D,B", $input[counter..counter + 1]); size = 1 }, },
        0x51 => { print_mneomonic("MOV  D,C", $input[counter..counter + 1]); size = 1 }, },
        0x52 => { print_mneomonic("MOV  D,D", $input[counter..counter + 1]); size = 1 }, },
        0x53 => { print_mneomonic("MOV  D,E", $input[counter..counter + 1]); size = 1 }, },
        0x54 => { print_mneomonic("MOV  D,H", $input[counter..counter + 1]); size = 1 }, },
        0x55 => { print_mneomonic("MOV  D,L", $input[counter..counter + 1]); size = 1 }, },
        0x56 => { print_mneomonic("MOV  D,M", $input[counter..counter + 1]); size = 1 }, },
        0x57 => { print_mneomonic("MOV  D,A", $input[counter..counter + 1]); size = 1 }, },

        // 58
        0x58 => { print_mneomonic("MOV  E,B", $input[counter..counter + 1]); size = 1 }, },
        0x59 => { print_mneomonic("MOV  E,C", $input[counter..counter + 1]); size = 1 }, },
        0x5a => { print_mneomonic("MOV  E,D", $input[counter..counter + 1]); size = 1 }, },
        0x5b => { print_mneomonic("MOV  E,E", $input[counter..counter + 1]); size = 1 }, },
        0x5c => { print_mneomonic("MOV  E,H", $input[counter..counter + 1]); size = 1 }, },
        0x5d => { print_mneomonic("MOV  E,L", $input[counter..counter + 1]); size = 1 }, },
        0x5e => { print_mneomonic("MOV  E,M", $input[counter..counter + 1]); size = 1 }, },
        0x5f => { print_mneomonic("MOV  E,A", $input[counter..counter + 1]); size = 1 }, },

        // 60
        0x60 => { print_mneomonic("MOV  H,B", $input[counter..counter + 1]); size = 1 }, },
        0x61 => { print_mneomonic("MOV  H,C", $input[counter..counter + 1]); size = 1 }, },
        0x62 => { print_mneomonic("MOV  H,D", $input[counter..counter + 1]); size = 1 }, },
        0x63 => { print_mneomonic("MOV  H,E", $input[counter..counter + 1]); size = 1 }, },
        0x64 => { print_mneomonic("MOV  H,H", $input[counter..counter + 1]); size = 1 }, },
        0x65 => { print_mneomonic("MOV  H,L", $input[counter..counter + 1]); size = 1 }, },
        0x66 => { print_mneomonic("MOV  H,M", $input[counter..counter + 1]); size = 1 }, },
        0x67 => { print_mneomonic("MOV  H,A", $input[counter..counter + 1]); size = 1 }, },

        // 68
        0x68 => { print_mneomonic("MOV  L,B", $input[counter..counter + 1]); size = 1 }, },
        0x69 => { print_mneomonic("MOV  L,C", $input[counter..counter + 1]); size = 1 }, },
        0x6a => { print_mneomonic("MOV  L,D", $input[counter..counter + 1]); size = 1 }, },
        0x6b => { print_mneomonic("MOV  L,E", $input[counter..counter + 1]); size = 1 }, },
        0x6c => { print_mneomonic("MOV  L,H", $input[counter..counter + 1]); size = 1 }, },
        0x6d => { print_mneomonic("MOV  L,L", $input[counter..counter + 1]); size = 1 }, },
        0x6e => { print_mneomonic("MOV  L,M", $input[counter..counter + 1]); size = 1 }, },
        0x6f => { print_mneomonic("MOV  L,A", $input[counter..counter + 1]); size = 1 }, },

        // 70
        0x70 => { print_mneomonic("MOV  M,B", $input[counter..counter + 1]); size = 1 }, },
        0x71 => { print_mneomonic("MOV  M,C", $input[counter..counter + 1]); size = 1 }, },
        0x72 => { print_mneomonic("MOV  M,D", $input[counter..counter + 1]); size = 1 }, },
        0x73 => { print_mneomonic("MOV  M,E", $input[counter..counter + 1]); size = 1 }, },
        0x74 => { print_mneomonic("MOV  M,H", $input[counter..counter + 1]); size = 1 }, },
        0x75 => { print_mneomonic("MOV  M,L", $input[counter..counter + 1]); size = 1 }, },
        0x76 => { print_mneomonic("HLT", $input[counter..counter + 1]); size = 1 }, },
        0x77 => { print_mneomonic("MOV  M,A", $input[counter..counter + 1]); size = 1 }, },

        // 78
        0x78 => { print_mneomonic("MOV  A,B", $input[counter..counter + 1]); size = 1 }, },
        0x79 => { print_mneomonic("MOV  A,C", $input[counter..counter + 1]); size = 1 }, },
        0x7a => { print_mneomonic("MOV  A,D", $input[counter..counter + 1]); size = 1 }, },
        0x7b => { print_mneomonic("MOV  A,E", $input[counter..counter + 1]); size = 1 }, },
        0x7c => { print_mneomonic("MOV  A,H", $input[counter..counter + 1]); size = 1 }, },
        0x7d => { print_mneomonic("MOV  A,L", $input[counter..counter + 1]); size = 1 }, },
        0x7e => { print_mneomonic("MOV  A,M", $input[counter..counter + 1]); size = 1 }, },
        0x7f => { print_mneomonic("MOV  A,A", $input[counter..counter + 1]); size = 1 }, },

        // 80
        0x80 => { print_mneomonic("ADD  B", $input[counter..counter + 1]); size = 1 }, },
        0x81 => { print_mneomonic("ADD  C", $input[counter..counter + 1]); size = 1 }, },
        0x82 => { print_mneomonic("ADD  D", $input[counter..counter + 1]); size = 1 }, },
        0x83 => { print_mneomonic("ADD  E", $input[counter..counter + 1]); size = 1 }, },
        0x84 => { print_mneomonic("ADD  H", $input[counter..counter + 1]); size = 1 }, },
        0x85 => { print_mneomonic("ADD  L", $input[counter..counter + 1]); size = 1 }, },
        0x86 => { print_mneomonic("ADD  M", $input[counter..counter + 1]); size = 1 }, },
        0x87 => { print_mneomonic("ADD  A", $input[counter..counter + 1]); size = 1 }, },

        // 88
        0x88 => { print_mneomonic("ADC  B", $input[counter..counter + 1]); size = 1 }, },
        0x89 => { print_mneomonic("ADC  C", $input[counter..counter + 1]); size = 1 }, },
        0x8a => { print_mneomonic("ADC  D", $input[counter..counter + 1]); size = 1 }, },
        0x8b => { print_mneomonic("ADC  E", $input[counter..counter + 1]); size = 1 }, },
        0x8c => { print_mneomonic("ADC  H", $input[counter..counter + 1]); size = 1 }, },
        0x8d => { print_mneomonic("ADC  L", $input[counter..counter + 1]); size = 1 }, },
        0x8e => { print_mneomonic("ADC  M", $input[counter..counter + 1]); size = 1 }, },
        0x8f => { print_mneomonic("ADC  A", $input[counter..counter + 1]); size = 1 }, },

        // 90
        0x90 => { print_mneomonic("SUB  B", $input[counter..counter + 1]); size = 1 }, },
        0x91 => { print_mneomonic("SUB  C", $input[counter..counter + 1]); size = 1 }, },
        0x92 => { print_mneomonic("SUB  D", $input[counter..counter + 1]); size = 1 }, },
        0x93 => { print_mneomonic("SUB  E", $input[counter..counter + 1]); size = 1 }, },
        0x94 => { print_mneomonic("SUB  H", $input[counter..counter + 1]); size = 1 }, },
        0x95 => { print_mneomonic("SUB  L", $input[counter..counter + 1]); size = 1 }, },
        0x96 => { print_mneomonic("SUB  M", $input[counter..counter + 1]); size = 1 }, },
        0x97 => { print_mneomonic("SUB  A", $input[counter..counter + 1]); size = 1 }, },

        // 98
        0x98 => { print_mneomonic("SBB  B", $input[counter..counter + 1]); size = 1 }, },
        0x99 => { print_mneomonic("SBB  C", $input[counter..counter + 1]); size = 1 }, },
        0x9a => { print_mneomonic("SBB  D", $input[counter..counter + 1]); size = 1 }, },
        0x9b => { print_mneomonic("SBB  E", $input[counter..counter + 1]); size = 1 }, },
        0x9c => { print_mneomonic("SBB  H", $input[counter..counter + 1]); size = 1 }, },
        0x9d => { print_mneomonic("SBB  L", $input[counter..counter + 1]); size = 1 }, },
        0x9e => { print_mneomonic("SBB  M", $input[counter..counter + 1]); size = 1 }, },
        0x9f => { print_mneomonic("SBB  A", $input[counter..counter + 1]); size = 1 }, },

        // a0
        0xa0 => { print_mneomonic("ANA  B", $input[counter..counter + 1]); size = 1 }, },
        0xa1 => { print_mneomonic("ANA  C", $input[counter..counter + 1]); size = 1 }, },
        0xa2 => { print_mneomonic("ANA  D", $input[counter..counter + 1]); size = 1 }, },
        0xa3 => { print_mneomonic("ANA  E", $input[counter..counter + 1]); size = 1 }, },
        0xa4 => { print_mneomonic("ANA  H", $input[counter..counter + 1]); size = 1 }, },
        0xa5 => { print_mneomonic("ANA  L", $input[counter..counter + 1]); size = 1 }, },
        0xa6 => { print_mneomonic("ANA  M", $input[counter..counter + 1]); size = 1 }, },
        0xa7 => { print_mneomonic("ANA  A", $input[counter..counter + 1]); size = 1 }, },

        // a8
        0xa8 => { print_mneomonic("XRA  B", $input[counter..counter + 1]); size = 1 }, },
        0xa9 => { print_mneomonic("XRA  C", $input[counter..counter + 1]); size = 1 }, },
        0xaa => { print_mneomonic("XRA  D", $input[counter..counter + 1]); size = 1 }, },
        0xab => { print_mneomonic("XRA  E", $input[counter..counter + 1]); size = 1 }, },
        0xac => { print_mneomonic("XRA  H", $input[counter..counter + 1]); size = 1 }, },
        0xad => { print_mneomonic("XRA  L", $input[counter..counter + 1]); size = 1 }, },
        0xae => { print_mneomonic("XRA  M", $input[counter..counter + 1]); size = 1 }, },
        0xaf => { print_mneomonic("XRA  A", $input[counter..counter + 1]); size = 1 }, },

        // b0
        0xb0 => { print_mneomonic("ORA  B", $input[counter..counter + 1]); size = 1 }, },
        0xb1 => { print_mneomonic("ORA  C", $input[counter..counter + 1]); size = 1 }, },
        0xb2 => { print_mneomonic("ORA  D", $input[counter..counter + 1]); size = 1 }, },
        0xb3 => { print_mneomonic("ORA  E", $input[counter..counter + 1]); size = 1 }, },
        0xb4 => { print_mneomonic("ORA  H", $input[counter..counter + 1]); size = 1 }, },
        0xb5 => { print_mneomonic("ORA  L", $input[counter..counter + 1]); size = 1 }, },
        0xb6 => { print_mneomonic("ORA  M", $input[counter..counter + 1]); size = 1 }, },
        0xb7 => { print_mneomonic("ORA  A", $input[counter..counter + 1]); size = 1 }, },

        // b8
        0xb8 => { print_mneomonic("CMP  B", $input[counter..counter + 1]); size = 1 }, },
        0xb9 => { print_mneomonic("CMP  C", $input[counter..counter + 1]); size = 1 }, },
        0xba => { print_mneomonic("CMP  D", $input[counter..counter + 1]); size = 1 }, },
        0xbb => { print_mneomonic("CMP  E", $input[counter..counter + 1]); size = 1 }, },
        0xbc => { print_mneomonic("CMP  H", $input[counter..counter + 1]); size = 1 }, },
        0xbd => { print_mneomonic("CMP  L", $input[counter..counter + 1]); size = 1 }, },
        0xbe => { print_mneomonic("CMP  M", $input[counter..counter + 1]); size = 1 }, },
        0xbf => { print_mneomonic("CMP  A", $input[counter..counter + 1]); size = 1 }, },

        // c0
        0xc0 => { print_mneomonic("RNZ", $input[counter..counter + 1]); size = 1 }, },
        0xc1 => { print_mneomonic("POP  B", $input[counter..counter + 1]); size = 1 }, },
        0xc2 => {
            println!("JNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc3 => { print_mnemonic("JMP", &input[counter..counter + 3]); size = 3 },
        0xc4 => {
            println!("CNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc5 => { print_mneomonic("PUSH B", $input[counter..counter + 1]); size = 1 }, },
        0xc6 => {
            println!("ADI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xc7 => { print_mneomonic("RST  0", $input[counter..counter + 1]); size = 1 }, },

        // c8
        0xc8 => { print_mneomonic("RZ", $input[counter..counter + 1]); size = 1 }, },
        0xc9 => { print_mneomonic("RET", $input[counter..counter + 1]); size = 1 }, },
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
        0xce => {
            println!("ACI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xcf => { print_mneomonic("RST  1", $input[counter..counter + 1]); size = 1 }, },

        // d0
        0xd0 => { print_mneomonic("RNC", $input[counter..counter + 1]); size = 1 }, },
        0xd1 => { print_mneomonic("POP  D", $input[counter..counter + 1]); size = 1 }, },
        0xd2 => {
            println!("JNC  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xd3 => {
            println!("OUT  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xd4 => {
            println!("CNC  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xd5 => { print_mneomonic("PUSH D", $input[counter..counter + 1]); size = 1 }, },
        0xd6 => {
            println!("SUI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xd7 => { print_mneomonic("RST  2", $input[counter..counter + 1]); size = 1 }, },

        // d8
        0xd8 => { print_mneomonic("RC", $input[counter..counter + 1]); size = 1 }, },
        0xd9 => { print_mneomonic("*RET", $input[counter..counter + 1]); size = 1 }, },
        0xda => {
            println!("JC   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xdb => {
            println!("IN   #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xdc => {
            println!("CC   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xdd => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xde => {
            println!("SBI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xdf => { print_mneomonic("RST  3", $input[counter..counter + 1]); size = 1 }, },

        // e0
        0xe0 => { print_mneomonic("RPO", $input[counter..counter + 1]); size = 1 }, },
        0xe1 => { print_mneomonic("POP  H", $input[counter..counter + 1]); size = 1 }, },
        0xe2 => {
            println!("JPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe3 => { print_mneomonic("XTHL", $input[counter..counter + 1]); size = 1 }, },
        0xe4 => {
            println!("CPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe5 => { print_mneomonic("PUSH H", $input[counter..counter + 1]); size = 1 }, },
        0xe6 => {
            println!("ANI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xe7 => { print_mneomonic("RST  4", $input[counter..counter + 1]); size = 1 }, },

        // e8
        0xe8 => { print_mneomonic("RPE", $input[counter..counter + 1]); size = 1 }, },
        0xe9 => { print_mneomonic("PCHL", $input[counter..counter + 1]); size = 1 }, },
        0xea => {
            println!("JPE  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xeb => { print_mneomonic("XCHG", $input[counter..counter + 1]); size = 1 }, },
        0xec => {
            println!("CPE  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xed => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xee => {
            println!("XRI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xef => { print_mneomonic("RST  5", $input[counter..counter + 1]); size = 1 }, },

        // f0
        0xf0 => { print_mneomonic("RP", $input[counter..counter + 1]); size = 1 }, },
        0xf1 => { print_mneomonic("POP  PSW", $input[counter..counter + 1]); size = 1 }, },
        0xf2 => {
            println!("JP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf3 => { print_mneomonic("DI", $input[counter..counter + 1]); size = 1 }, },
        0xf4 => {
            println!("CP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf5 => { print_mneomonic("PUSH PSW", $input[counter..counter + 1]); size = 1 }, },
        0xf6 => {
            println!("ORI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xf7 => { print_mneomonic("RST  6", $input[counter..counter + 1]); size = 1 }, },

        // f8
        0xf8 => { print_mneomonic("RM", $input[counter..counter + 1]); size = 1 }, },
        0xf9 => { print_mneomonic("SPHL", $input[counter..counter + 1]); size = 1 }, },
        0xfa => {
            println!("JM   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfb => { print_mneomonic("EI", $input[counter..counter + 1]); size = 1 }, },
        0xfc => {
            println!("CM   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfd => {
            println!("*CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfe => {
            println!("CPI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xff => { print_mneomonic("RST  7", $input[counter..counter + 1]); size = 1 }, },

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
