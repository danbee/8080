use std::io::{self, Read};

fn main() {
    let mut input: Vec<u8> = vec![];

    let mut counter: usize = 0;

    io::stdin()
        .read_to_end(&mut input)
        .unwrap();

    // for byte in input.iter() {
        // println!("{}", byte);
    // };

    while counter < input.len() {
        // get the instruction
        let opcode = input[counter];
        match opcode {
            0x00 => println!("NOP"),
            0xc3 => {
                counter += 1;
                let low  = input[counter];
                counter += 1;
                let high = input[counter];
                println!("JMP ${:x}{:x}", high, low);
            }
            _  => println!("{:x}", opcode),
        }
        counter += 1;
    }
}
