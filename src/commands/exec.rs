use clap::ArgMatches;
use util::*;
use display;
use state::Chip8State;


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

    let mut state: Chip8State = Chip8State::new();

    state.load_program(&program);

    println!("Done!");

    display::init_display();

    loop {
        display::update_display(&state);
        state.exec_step(display::get_char);
    }

    display::close_display();
}
