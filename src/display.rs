use ncurses::*;
use state::Chip8State;
use std::char::from_u32;

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
    let mut on_pxs = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let on = state.pixel_on(x, y);
            if on {
                on_pxs = on_pxs + 1;
            }
            let attr_fn = if on { attr_on } else { attr_off };
            attr_fn(A_REVERSE());
            mvprintw(y as i32, x as i32, if on {"0"} else {" "});
        }
    }

    attr_off(A_REVERSE());

    let opcode = state.get_next_opcode().unwrap();
    mvprintw(32, 0, format!("PC: 0x{:X} (0x{:X} / {})", state.pc, opcode.to_bin(), opcode.to_asm()).as_ref());
    mvprintw(33, 0, format!("I: 0x{:X}, On pixels: {}", state.i, on_pxs).as_ref());
    for i in 0..4 {
        let r = i * 4;
        mvprintw(34 + i, 0, format!("V{:X}: 0x{:X}, V{:X}: 0x{:X}, V{:X}: 0x{:X}, V{:X}: 0x{:X}",
                                    r, state.vregs[r as usize],
                                    r + 1, state.vregs[(r + 1) as usize],
                                    r + 2, state.vregs[(r + 2) as usize],
                                    r + 3, state.vregs[(r + 3) as usize]).as_ref());
    }
    mvprintw(38, 0, format!("DELAY: {}, SOUND: {}, UPDATES: {}", state.delay, state.sound, state.timer_updates).as_ref());

    refresh();
}

pub fn get_char() -> u8 {
    loop {
        let key = getch() as u32;
        refresh();
        let c = from_u32(key);
        if c.is_none() { continue; }
        let c = c.unwrap();
        if c >= '0' && c <= '9' {
            return (c as u8) - '0' as u8;
        }
        let c = c.to_ascii_lowercase();
        if c >= 'a' && c <= 'f' {
            return (c as u8) - 'a' as u8;
        }
    }
}

pub fn close_display() {
    endwin();
}