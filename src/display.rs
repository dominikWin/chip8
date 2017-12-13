use ncurses::*;

pub fn init_display() {
    initscr();
    attr_on(A_REVERSE());
    printw("Chip8");
    attr_off(A_REVERSE());
    printw("Chip8");
}

pub fn update_display() {
    refresh();
    getch();
}

pub fn close_display() {
    endwin();
}