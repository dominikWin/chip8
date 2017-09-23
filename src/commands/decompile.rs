use clap::ArgMatches;
use std::io;
use std::fs;
use util::*;
use opcode::Opcode;

pub fn cmd_decompile(matches: &ArgMatches) {
    let mut input: Box<io::Read> = {
        let input_val = matches.value_of("input").unwrap();
        if input_val == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(fs::File::open(input_val).expect("file not found"))
        }
    };

    let mut addr: u16 = 0x200;
    loop {
        let mut buf = [0u8, 2];
        if let Result::Ok(r) = input.read(&mut buf) {
            if r < 2 {
                break;
            }
            let instruction: u16 = ((buf[0] as u16) << 8) | (buf[1] as u16);
            let (l, r) = filled_hex_dual(instruction);
            let asm = Opcode::new(instruction);
            let asm = if let Some(oc) = asm {
                oc.to_asm()
            } else {
                "[UNDEFINED]".to_string()
            };
            println!("{}: {} {}   {}", filled_hex(addr), l, r, asm);
        } else {
            break;
        }
        addr += 2;
    }
}
