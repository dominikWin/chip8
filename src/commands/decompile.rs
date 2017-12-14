use clap::ArgMatches;
use util::*;
use opcode::Opcode;

pub fn cmd_decompile(matches: &ArgMatches) {
    let program = program_from_jnput(matches);

    if let Err(e) = program {
        println!(
            "File {} can't be read: {}",
            matches.value_of("input").unwrap(),
            e
        );
        return;
    }

    let program = program.unwrap();

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
