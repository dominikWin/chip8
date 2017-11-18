use clap::ArgMatches;
use util::*;
use program::Chip8Program;

pub fn cmd_exec(matches: &ArgMatches) {
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

    println!("Done!");
}
