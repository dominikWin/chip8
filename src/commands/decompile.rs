use clap::ArgMatches;
use std::io;
use std::fs;
use util::*;
use opcode::Opcode;
use program::Chip8Program;

pub fn cmd_decompile(matches: &ArgMatches) {
    let input: Box<io::Read> = {
        let input_val = matches.value_of("input").unwrap();
        if input_val == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(fs::File::open(input_val).expect("file not found"))
        }
    };

    let program = Chip8Program::from(input).unwrap();

    let mut addr: u16 = 0x200;

    for instruction in program.instructions {
        let (l, r) = filled_hex_dual(instruction);
        let asm = Opcode::new(instruction);
        let asm = if let Some(oc) = asm {
            oc.to_asm()
        } else {
            "[UNDEFINED]".to_string()
        };
        println!("{}: {} {}   {}", filled_hex(addr), l, r, asm);
        addr += 2;
    }
}
