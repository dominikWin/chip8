use ncurses::*;
use state::Chip8State;

const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;

pub fn init_display() {
    initscr();
    attr_on(A_REVERSE());
    printw("Chip8");
    attr_off(A_REVERSE());
    printw("Chip8");
}

pub fn update_display(state: &Chip8State) {
    clear();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let on = state.pixel_on(x, y);
            let attr_fn = if on { attr_on } else { attr_off };
            attr_fn(A_REVERSE());
            mvprintw(y as i32, x as i32, " ");
        }
    }
    mvprintw(32, 0, format!("PC: 0x{:X} ({})", state.pc, state.get_next_opcode().unwrap().to_asm()).as_ref());
    mvprintw(33, 0, format!("I: 0x{:X}", state.i).as_ref());
    for i in 0..4 {
        let r = i * 4;
        mvprintw(34 + i, 0, format!("V{:X}: 0x{:X}, V{:X}: 0x{:X}, V{:X}: 0x{:X}, V{:X}: 0x{:X}",
                                    r, state.vregs[r as usize],
                                    r + 1, state.vregs[(r + 1) as usize],
                                    r + 2, state.vregs[(r + 2) as usize],
                                    r + 3, state.vregs[(r + 3) as usize]).as_ref());
    }
    refresh();
    getch();
}

pub fn close_display() {
    endwin();
}