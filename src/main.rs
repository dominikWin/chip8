extern crate clap;

use clap::{Arg, App, SubCommand};


mod opcode;
mod register;
mod commands;
mod util;

fn main() {
    let matches = App::new("chip8")
        .version("0.1.0")
        .author("Dominik Winecki <dominikwinecki@gmail.com>")
        .about("A CHIP8 emulator")
        .subcommand(
            SubCommand::with_name("decompile")
                .about("Prints the decompiled file to stdout")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .value_name("INPUT")
                        .required(true),
                ),
        )
        .get_matches();

    if matches.subcommand_name().is_none() {
        println!("No command found!\n\n{}", matches.usage());
        return;
    }

    match matches.subcommand_name().unwrap() {
        "decompile" => commands::decompile::cmd_decompile(
            &matches.subcommand_matches("decompile").unwrap(),
        ),
        other => panic!("Invalid subcommand {}", other),
    }
}
