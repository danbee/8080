use std::io::{self, Read};

fn opcode(input: &Vec<u8>, counter: usize) -> usize {
    let size: usize;

    let opcode = input[counter];

    match opcode {
        0x00 => { println!("NOP"); size = 1 },
        0x0f => { println!("RRC"); size = 1 },
        0x21 => {
            println!("LXI  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x27 => { println!("DAA"); size = 1 },
        0x32 => {
            println!("STA  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x35 => { println!("DCR  M"); size = 1 },
        0x3a => {
            println!("LDA  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x3e => {
            println!("MVI  A,#0x{:02x}", input[counter + 1]);
            size = 2;
        },

        0x80 => { println!("ADD  B"); size = 1 },
        0x81 => { println!("ADD  C"); size = 1 },
        0x82 => { println!("ADD  D"); size = 1 },
        0x83 => { println!("ADD  E"); size = 1 },
        0x84 => { println!("ADD  H"); size = 1 },
        0x85 => { println!("ADD  L"); size = 1 },
        0x86 => { println!("ADD  M"); size = 1 },
        0x87 => { println!("ADD  A"); size = 1 },

        0x88 => { println!("ADC  B"); size = 1 },
        0x89 => { println!("ADC  C"); size = 1 },
        0x8a => { println!("ADC  D"); size = 1 },
        0x8b => { println!("ADC  E"); size = 1 },
        0x8c => { println!("ADC  H"); size = 1 },
        0x8d => { println!("ADC  L"); size = 1 },
        0x8e => { println!("ADC  M"); size = 1 },
        0x8f => { println!("ADC  A"); size = 1 },

        0x90 => { println!("SUB  B"); size = 1 },
        0x91 => { println!("SUB  C"); size = 1 },
        0x92 => { println!("SUB  D"); size = 1 },
        0x93 => { println!("SUB  E"); size = 1 },
        0x94 => { println!("SUB  H"); size = 1 },
        0x95 => { println!("SUB  L"); size = 1 },
        0x96 => { println!("SUB  M"); size = 1 },
        0x97 => { println!("SUB  A"); size = 1 },

        0x98 => { println!("SBB  B"); size = 1 },
        0x99 => { println!("SBB  C"); size = 1 },
        0x9a => { println!("SBB  D"); size = 1 },
        0x9b => { println!("SBB  E"); size = 1 },
        0x9c => { println!("SBB  H"); size = 1 },
        0x9d => { println!("SBB  L"); size = 1 },
        0x9e => { println!("SBB  M"); size = 1 },
        0x9f => { println!("SBB  A"); size = 1 },

        0xa0 => { println!("ANA  B"); size = 1 },
        0xa1 => { println!("ANA  C"); size = 1 },
        0xa2 => { println!("ANA  D"); size = 1 },
        0xa3 => { println!("ANA  E"); size = 1 },
        0xa4 => { println!("ANA  H"); size = 1 },
        0xa5 => { println!("ANA  L"); size = 1 },
        0xa6 => { println!("ANA  M"); size = 1 },
        0xa7 => { println!("ANA  A"); size = 1 },

        0xa8 => { println!("XRA  B"); size = 1 },
        0xa9 => { println!("XRA  C"); size = 1 },
        0xaa => { println!("XRA  D"); size = 1 },
        0xab => { println!("XRA  E"); size = 1 },
        0xac => { println!("XRA  H"); size = 1 },
        0xad => { println!("XRA  L"); size = 1 },
        0xae => { println!("XRA  M"); size = 1 },
        0xaf => { println!("XRA  A"); size = 1 },

        0xb0 => { println!("ORA  B"); size = 1 },
        0xb1 => { println!("ORA  C"); size = 1 },
        0xb2 => { println!("ORA  D"); size = 1 },
        0xb3 => { println!("ORA  E"); size = 1 },
        0xb4 => { println!("ORA  H"); size = 1 },
        0xb5 => { println!("ORA  L"); size = 1 },
        0xb6 => { println!("ORA  M"); size = 1 },
        0xb7 => { println!("ORA  A"); size = 1 },

        0xb8 => { println!("CMP  B"); size = 1 },
        0xb9 => { println!("CMP  C"); size = 1 },
        0xba => { println!("CMP  D"); size = 1 },
        0xbb => { println!("CMP  E"); size = 1 },
        0xbc => { println!("CMP  H"); size = 1 },
        0xbd => { println!("CMP  L"); size = 1 },
        0xbe => { println!("CMP  M"); size = 1 },
        0xbf => { println!("CMP  A"); size = 1 },

        0xc2 => {
            println!("JNZ  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc3 => {
            println!("JMP  ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc5 => { println!("PUSH B"); size = 1 },
        0xc6 => {
            println!("ADI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        0xca => {
            println!("JZ   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xcd => {
            println!("CALL ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xd5 => { println!("PUSH D"); size = 1 },
        0xda => {
            println!("JC   ${:02x}{:02x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xdb => {
            println!("IN   #0x{:02x}", input[counter + 1]);
            size = 2;
        }
        0xe5 => { println!("PUSH H"); size = 1 },
        0xf5 => { println!("PUSH PSW"); size = 1 },
        0xfe => {
            println!("CPI  #0x{:02x}", input[counter + 1]);
            size = 2;
        },
        _  => { println!("{:02x}", opcode); size = 1 },
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
        oplength = opcode(&input, counter);
        counter += oplength;
    }
}
