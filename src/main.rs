use std::io::{self, Read};

fn opcode(input: &Vec<u8>, counter: usize) -> usize {
    let size: usize;

    let opcode = input[counter];

    match opcode {
        // 00
        0x00 => { println!("NOP"); size = 1 },
        0x01 => {
            println!("LXI  B,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x02 => { println!("STAX B"); size = 1 },
        0x03 => { println!("INX  B"); size = 1 },
        0x04 => { println!("INR  B"); size = 1 },
        0x05 => { println!("DCR  B"); size = 1 },
        0x06 => {
            println!("MVI  B,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x07 => { println!("RLC"); size = 1 },

        // 08
        0x08 => { println!("*NOP"); size = 1 },
        0x09 => { println!("DAD  B"); size = 1 },
        0x0a => { println!("LDAX B"); size = 1 },
        0x0b => { println!("DCX  B"); size = 1 },
        0x0c => { println!("INR  C"); size = 1 },
        0x0d => { println!("DCR  C"); size = 1 },
        0x0e => {
            println!("MVI  C,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x0f => { println!("RRC"); size = 1 },

        // 10
        0x10 => { println!("*NOP"); size = 1 },
        0x11 => {
            println!("LXI  D,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x12 => { println!("STAX D"); size = 1 },
        0x13 => { println!("INX  D"); size = 1 },
        0x14 => { println!("INR  D"); size = 1 },
        0x15 => { println!("DCR  D"); size = 1 },
        0x16 => {
            println!("MVI  D,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x17 => { println!("RAL"); size = 1 },

        // 18
        0x18 => { println!("*NOP"); size = 1 },
        0x19 => { println!("DAD  D"); size = 1 },
        0x1a => { println!("LDAX D"); size = 1 },
        0x1b => { println!("DCX  D"); size = 1 },
        0x1c => { println!("INR  E"); size = 1 },
        0x1d => { println!("DCR  E"); size = 1 },
        0x1e => {
            println!("MVI  E,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x1f => { println!("RAR"); size = 1 },

        // 20
        0x20 => { println!("*NOP"); size = 1 },
        0x21 => {
            println!("LXI  H,${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x22 => {
            println!("SHLD ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x23 => { println!("INX  H"); size = 1 },
        0x24 => { println!("INR  H"); size = 1 },
        0x25 => { println!("DCR  H"); size = 1 },
        0x26 => {
            println!("MVI  H,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x27 => { println!("DDA"); size = 1 },

        // 28
        0x28 => { println!("*NOP"); size = 1 },
        0x29 => { println!("DAD  H"); size = 1 },
        0x2a => {
            println!("LHLD ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x2b => { println!("DCX  H"); size = 1 },
        0x2c => { println!("INR  L"); size = 1 },
        0x2d => { println!("DCR  L"); size = 1 },
        0x2e => {
            println!("MVI  L,#0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0x2f => { println!("CMA"); size = 1 },

        // 30
        0x30 => { println!("*NOP"); size = 1 },
        0x31 => {
            println!("LXI  SP,${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x32 => {
            println!("STA  ${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x33 => { println!("INX  SP"); size = 1 },
        0x34 => { println!("INR  M"); size = 1 },
        0x35 => { println!("DCR  M"); size = 1 },
        0x36 => {
            println!("MVI  M,#0x{:03x}", input[counter + 1]);
            size = 3;
        },
        0x37 => { println!("STC"); size = 1 },

        // 38
        0x38 => { println!("*NOP"); size = 1 },
        0x39 => { println!("DAD  SP"); size = 1 },
        0x3a => {
            println!("LDA  ${:03x}{:03x}", input[counter + 3], input[counter + 1]);
            size = 3;
        },
        0x3b => { println!("DCX  SP"); size = 1 },
        0x3c => { println!("INR  A"); size = 1 },
        0x3d => { println!("DCR  A"); size = 1 },
        0x3e => {
            println!("MVI  A,#0x{:03x}", input[counter + 1]);
            size = 3;
        },
        0x3f => { println!("CMC"); size = 1 },

        // 40
        0x40 => { println!("MOV  B,B"); size = 1 },
        0x41 => { println!("MOV  B,C"); size = 1 },
        0x42 => { println!("MOV  B,D"); size = 1 },
        0x43 => { println!("MOV  B,E"); size = 1 },
        0x44 => { println!("MOV  B,H"); size = 1 },
        0x45 => { println!("MOV  B,L"); size = 1 },
        0x46 => { println!("MOV  B,M"); size = 1 },
        0x47 => { println!("MOV  B,A"); size = 1 },

        // 48
        0x48 => { println!("MOV  C,B"); size = 1 },
        0x49 => { println!("MOV  C,C"); size = 1 },
        0x4a => { println!("MOV  C,D"); size = 1 },
        0x4b => { println!("MOV  C,E"); size = 1 },
        0x4c => { println!("MOV  C,H"); size = 1 },
        0x4d => { println!("MOV  C,L"); size = 1 },
        0x4e => { println!("MOV  C,M"); size = 1 },
        0x4f => { println!("MOV  C,A"); size = 1 },

        // 50
        0x50 => { println!("MOV  D,B"); size = 1 },
        0x51 => { println!("MOV  D,C"); size = 1 },
        0x52 => { println!("MOV  D,D"); size = 1 },
        0x53 => { println!("MOV  D,E"); size = 1 },
        0x54 => { println!("MOV  D,H"); size = 1 },
        0x55 => { println!("MOV  D,L"); size = 1 },
        0x56 => { println!("MOV  D,M"); size = 1 },
        0x57 => { println!("MOV  D,A"); size = 1 },

        // 58
        0x58 => { println!("MOV  E,B"); size = 1 },
        0x59 => { println!("MOV  E,C"); size = 1 },
        0x5a => { println!("MOV  E,D"); size = 1 },
        0x5b => { println!("MOV  E,E"); size = 1 },
        0x5c => { println!("MOV  E,H"); size = 1 },
        0x5d => { println!("MOV  E,L"); size = 1 },
        0x5e => { println!("MOV  E,M"); size = 1 },
        0x5f => { println!("MOV  E,A"); size = 1 },

        // 60
        0x60 => { println!("MOV  H,B"); size = 1 },
        0x61 => { println!("MOV  H,C"); size = 1 },
        0x62 => { println!("MOV  H,D"); size = 1 },
        0x63 => { println!("MOV  H,E"); size = 1 },
        0x64 => { println!("MOV  H,H"); size = 1 },
        0x65 => { println!("MOV  H,L"); size = 1 },
        0x66 => { println!("MOV  H,M"); size = 1 },
        0x67 => { println!("MOV  H,A"); size = 1 },

        // 68
        0x68 => { println!("MOV  L,B"); size = 1 },
        0x69 => { println!("MOV  L,C"); size = 1 },
        0x6a => { println!("MOV  L,D"); size = 1 },
        0x6b => { println!("MOV  L,E"); size = 1 },
        0x6c => { println!("MOV  L,H"); size = 1 },
        0x6d => { println!("MOV  L,L"); size = 1 },
        0x6e => { println!("MOV  L,M"); size = 1 },
        0x6f => { println!("MOV  L,A"); size = 1 },

        // 70
        0x70 => { println!("MOV  M,B"); size = 1 },
        0x71 => { println!("MOV  M,C"); size = 1 },
        0x72 => { println!("MOV  M,D"); size = 1 },
        0x73 => { println!("MOV  M,E"); size = 1 },
        0x74 => { println!("MOV  M,H"); size = 1 },
        0x75 => { println!("MOV  M,L"); size = 1 },
        0x76 => { println!("HLT"); size = 1 },
        0x77 => { println!("MOV  M,A"); size = 1 },

        // 78
        0x78 => { println!("MOV  A,B"); size = 1 },
        0x79 => { println!("MOV  A,C"); size = 1 },
        0x7a => { println!("MOV  A,D"); size = 1 },
        0x7b => { println!("MOV  A,E"); size = 1 },
        0x7c => { println!("MOV  A,H"); size = 1 },
        0x7d => { println!("MOV  A,L"); size = 1 },
        0x7e => { println!("MOV  A,M"); size = 1 },
        0x7f => { println!("MOV  A,A"); size = 1 },

        // 80
        0x80 => { println!("ADD  B"); size = 1 },
        0x81 => { println!("ADD  C"); size = 1 },
        0x82 => { println!("ADD  D"); size = 1 },
        0x83 => { println!("ADD  E"); size = 1 },
        0x84 => { println!("ADD  H"); size = 1 },
        0x85 => { println!("ADD  L"); size = 1 },
        0x86 => { println!("ADD  M"); size = 1 },
        0x87 => { println!("ADD  A"); size = 1 },

        // 88
        0x88 => { println!("ADC  B"); size = 1 },
        0x89 => { println!("ADC  C"); size = 1 },
        0x8a => { println!("ADC  D"); size = 1 },
        0x8b => { println!("ADC  E"); size = 1 },
        0x8c => { println!("ADC  H"); size = 1 },
        0x8d => { println!("ADC  L"); size = 1 },
        0x8e => { println!("ADC  M"); size = 1 },
        0x8f => { println!("ADC  A"); size = 1 },

        // 90
        0x90 => { println!("SUB  B"); size = 1 },
        0x91 => { println!("SUB  C"); size = 1 },
        0x92 => { println!("SUB  D"); size = 1 },
        0x93 => { println!("SUB  E"); size = 1 },
        0x94 => { println!("SUB  H"); size = 1 },
        0x95 => { println!("SUB  L"); size = 1 },
        0x96 => { println!("SUB  M"); size = 1 },
        0x97 => { println!("SUB  A"); size = 1 },

        // 98
        0x98 => { println!("SBB  B"); size = 1 },
        0x99 => { println!("SBB  C"); size = 1 },
        0x9a => { println!("SBB  D"); size = 1 },
        0x9b => { println!("SBB  E"); size = 1 },
        0x9c => { println!("SBB  H"); size = 1 },
        0x9d => { println!("SBB  L"); size = 1 },
        0x9e => { println!("SBB  M"); size = 1 },
        0x9f => { println!("SBB  A"); size = 1 },

        // a0
        0xa0 => { println!("ANA  B"); size = 1 },
        0xa1 => { println!("ANA  C"); size = 1 },
        0xa2 => { println!("ANA  D"); size = 1 },
        0xa3 => { println!("ANA  E"); size = 1 },
        0xa4 => { println!("ANA  H"); size = 1 },
        0xa5 => { println!("ANA  L"); size = 1 },
        0xa6 => { println!("ANA  M"); size = 1 },
        0xa7 => { println!("ANA  A"); size = 1 },

        // a8
        0xa8 => { println!("XRA  B"); size = 1 },
        0xa9 => { println!("XRA  C"); size = 1 },
        0xaa => { println!("XRA  D"); size = 1 },
        0xab => { println!("XRA  E"); size = 1 },
        0xac => { println!("XRA  H"); size = 1 },
        0xad => { println!("XRA  L"); size = 1 },
        0xae => { println!("XRA  M"); size = 1 },
        0xaf => { println!("XRA  A"); size = 1 },

        // b0
        0xb0 => { println!("ORA  B"); size = 1 },
        0xb1 => { println!("ORA  C"); size = 1 },
        0xb2 => { println!("ORA  D"); size = 1 },
        0xb3 => { println!("ORA  E"); size = 1 },
        0xb4 => { println!("ORA  H"); size = 1 },
        0xb5 => { println!("ORA  L"); size = 1 },
        0xb6 => { println!("ORA  M"); size = 1 },
        0xb7 => { println!("ORA  A"); size = 1 },

        // b8
        0xb8 => { println!("CMP  B"); size = 1 },
        0xb9 => { println!("CMP  C"); size = 1 },
        0xba => { println!("CMP  D"); size = 1 },
        0xbb => { println!("CMP  E"); size = 1 },
        0xbc => { println!("CMP  H"); size = 1 },
        0xbd => { println!("CMP  L"); size = 1 },
        0xbe => { println!("CMP  M"); size = 1 },
        0xbf => { println!("CMP  A"); size = 1 },

        // c0
        0xc0 => { println!("RNZ"); size = 1 },
        0xc1 => { println!("POP  B"); size = 1 },
        0xc2 => {
            println!("JNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc3 => {
            println!("JMP  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc4 => {
            println!("CNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc5 => { println!("PUSH B"); size = 1 },
        0xc6 => {
            println!("ADI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xc7 => { println!("RST  0"); size = 1 },

        // c8
        0xc8 => { println!("RZ"); size = 1 },
        0xc9 => { println!("RET"); size = 1 },
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
        0xcf => { println!("RST  1"); size = 1 },

        // d0
        0xd0 => { println!("RNC"); size = 1 },
        0xd1 => { println!("POP  D"); size = 1 },
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
        0xd5 => { println!("PUSH D"); size = 1 },
        0xd6 => {
            println!("SUI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xd7 => { println!("RST  2"); size = 1 },

        // d8
        0xd8 => { println!("RC"); size = 1 },
        0xd9 => { println!("*RET"); size = 1 },
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
        0xdf => { println!("RST  3"); size = 1 },

        // e0
        0xe0 => { println!("RPO"); size = 1 },
        0xe1 => { println!("POP  H"); size = 1 },
        0xe2 => {
            println!("JPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe3 => { println!("XTHL"); size = 1 },
        0xe4 => {
            println!("CPO  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xe5 => { println!("PUSH H"); size = 1 },
        0xe6 => {
            println!("ANI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xe7 => { println!("RST  4"); size = 1 },

        // e8
        0xe8 => { println!("RPE"); size = 1 },
        0xe9 => { println!("PCHL"); size = 1 },
        0xea => {
            println!("JPE  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xeb => { println!("XCHG"); size = 1 },
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
        0xef => { println!("RST  5"); size = 1 },

        // f0
        0xf0 => { println!("RP"); size = 1 },
        0xf1 => { println!("POP  PSW"); size = 1 },
        0xf2 => {
            println!("JP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf3 => { println!("DI"); size = 1 },
        0xf4 => {
            println!("CP   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xf5 => { println!("PUSH PSW"); size = 1 },
        0xf6 => {
            println!("ORI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xf7 => { println!("RST  6"); size = 1 },

        // f8
        0xf8 => { println!("RM"); size = 1 },
        0xf9 => { println!("SPHL"); size = 1 },
        0xfa => {
            println!("JM   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xfb => { println!("EI"); size = 1 },
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
        0xff => { println!("RST  7"); size = 1 },

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
