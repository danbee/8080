use std::io::{self, Read};

fn opcode(input: &Vec<u8>, counter: &usize) -> usize {
    let size: usize;

    let opcode = input[counter];

    match opcode {
        0x00 => { println!("NOP"); size = 1 },
        0x21 => {
            println!("LXI ${:x}{:x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x32 => {
            println!("STA ${:x}{:x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0x3e => {
            println!("MVI A,#0x{:x}", input[counter + 1]);
            size = 2;
        },
        0xc3 => {
            println!("JMP ${:x}{:x}", input[counter + 2], input[counter + 1]);
            size = 3;
        },
        0xc5 => { println!("PUSH B"); size = 1 },
        0xd5 => { println!("PUSH D"); size = 1},
        0xe5 => { println!("PUSH H"); size = 1},
        0xf5 => { println!("PUSH PSW"); size = 1},
        _  => { println!("{:x}", opcode); size = 1},
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
        oplength = opcode(&input, &counter);
        counter += oplength;
    }
}
