use program::Chip8Program;
use std::fmt;
use std::cmp;
use opcode::Opcode;
use register::*;
use rand;
use rand::Rng;
use chrono::prelude::*;

#[cfg(test)]
use test::Bencher;

const FONT_START: u16 = 0x100;
const DISP_START: u16 = 0xF00;
const STACK_START: u16 = 0xEA0;

pub struct Chip8State {
    pub vregs: [u8; 16],
    pub i: u16,
    pub sp: u8,
    pub pc: u16,
    pub delay: u8,
    pub sound: u8,
    timer_start: DateTime<Local>,
    pub timer_updates: u64,
    pub mem: [u8; 0x1000],
}

impl Chip8State {
    pub fn new() -> Chip8State {
        let mut state = Chip8State {
            vregs: [0; 16],
            i: 0x0000,
            sp: 0,
            pc: 0x0200,
            delay: 0,
            sound: 0,
            timer_start: Local::now(),
            timer_updates: 0,
            mem: [0; 0x1000],
        };
        state.load_font();
        return state;
    }

    pub fn load_program(&mut self, program: &Chip8Program) {
        let mut addr = 0x0200;
        for instruction in program.instructions.iter() {
            self.mem[addr] = ((instruction & 0xff00) >> 8) as u8;
            self.mem[addr + 1] = (instruction & 0x00ff) as u8;
            addr += 2;
        }
    }

    fn load_font(&mut self) {
        // See http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.4 for reference

        // 0
        let start = FONT_START;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0x90;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0xf0;

        // 1
        let start = FONT_START + 1 * 5;
        self.mem[(start + 0) as usize] = 0x20;
        self.mem[(start + 1) as usize] = 0x60;
        self.mem[(start + 2) as usize] = 0x20;
        self.mem[(start + 3) as usize] = 0x20;
        self.mem[(start + 4) as usize] = 0x70;

        // 2
        let start = FONT_START + 2 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x10;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x80;
        self.mem[(start + 4) as usize] = 0xf0;

        // 3
        let start = FONT_START + 3 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x10;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x10;
        self.mem[(start + 4) as usize] = 0xf0;

        // 4
        let start = FONT_START + 4 * 5;
        self.mem[(start + 0) as usize] = 0x90;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x10;
        self.mem[(start + 4) as usize] = 0x10;

        // 5
        let start = FONT_START + 5 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x80;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x10;
        self.mem[(start + 4) as usize] = 0xf0;

        // 6
        let start = FONT_START + 6 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x80;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0xf0;

        // 7
        let start = FONT_START + 7 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x10;
        self.mem[(start + 2) as usize] = 0x20;
        self.mem[(start + 3) as usize] = 0x40;
        self.mem[(start + 4) as usize] = 0x40;

        // 8
        let start = FONT_START + 8 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0xf0;

        // 9
        let start = FONT_START + 9 * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x10;
        self.mem[(start + 4) as usize] = 0xf0;

        // a
        let start = FONT_START + 0xa * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0x90;

        // b
        let start = FONT_START + 0xb * 5;
        self.mem[(start + 0) as usize] = 0xe0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0xe0;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0xe0;

        // c
        let start = FONT_START + 0xc * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x80;
        self.mem[(start + 2) as usize] = 0x80;
        self.mem[(start + 3) as usize] = 0x80;
        self.mem[(start + 4) as usize] = 0xf0;

        // d
        let start = FONT_START + 0xd * 5;
        self.mem[(start + 0) as usize] = 0xe0;
        self.mem[(start + 1) as usize] = 0x90;
        self.mem[(start + 2) as usize] = 0x90;
        self.mem[(start + 3) as usize] = 0x90;
        self.mem[(start + 4) as usize] = 0xe0;

        // e
        let start = FONT_START + 0xe * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x80;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x80;
        self.mem[(start + 4) as usize] = 0xf0;

        // f
        let start = FONT_START + 0xf * 5;
        self.mem[(start + 0) as usize] = 0xf0;
        self.mem[(start + 1) as usize] = 0x80;
        self.mem[(start + 2) as usize] = 0xf0;
        self.mem[(start + 3) as usize] = 0x80;
        self.mem[(start + 4) as usize] = 0x80;
    }

    pub fn get_next_opcode(&self) -> Option<Opcode> {
        let instruction: u16 = ((self.mem[self.pc as usize] as u16) << 8) |
            ((self.mem[self.pc as usize + 1]) as u16);
        Opcode::new(instruction)
    }

    pub fn exec_step(&mut self, get_char_fn: fn () -> u8) {
        self.update_timers();
        let opcode = self.get_next_opcode();
        if let None = opcode {
            panic!("Failed to decode instruction {:x}", self.pc);
        }
        let opcode = opcode.unwrap();
        let mut skip_inc_pc = false;
        match opcode {
            Opcode::CLS => for i in DISP_START..0xFFF + 1 {
                self.mem[i as usize] = 0x00;
            }
            Opcode::RET => {
                let pc = self.stack_pop();
                self.pc = pc;
                skip_inc_pc = true;
            }
            Opcode::JMP(n) => {
                self.pc = n;
                skip_inc_pc = true;
            }
            Opcode::CALL(n) => {
                let pc = self.pc;
                self.stack_push(pc);
                self.pc = n;
                skip_inc_pc = true;
            }
            Opcode::SKIPEQ(x, n) => {
                if self.vreg_val(&x) == n {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::SKIPNEQ(x, n) => {
                if self.vreg_val(&x) != n {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::SKIPREQ(x, y) => {
                if self.vreg_val(&x) == self.vreg_val(&y) {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::MOV(x, n) => self.set_vreg_val(&x, n),
            Opcode::ADD(x, n) => {
                let x_val = self.vreg_val(&x);
                let (new_val, _) = x_val.overflowing_add(n);
                self.set_vreg_val(&x, new_val);
            }
            Opcode::MOVR(x, y) => {
                let y_val = self.vreg_val(&y);
                self.set_vreg_val(&x, y_val);
            }
            Opcode::OR(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let new_val = x_val | y_val;
                self.set_vreg_val(&x, new_val);
            }
            Opcode::AND(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let new_val = x_val & y_val;
                self.set_vreg_val(&x, new_val);
            }
            Opcode::XOR(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let new_val = x_val ^ y_val;
                self.set_vreg_val(&x, new_val);
            }
            Opcode::ADDR(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let (new_val, carry) = x_val.overflowing_add(y_val);
                self.set_vreg_val(&x, new_val);
                self.vregs[0xf] = carry as u8;
            }
            Opcode::SUBR(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let (new_val, carry) = x_val.overflowing_sub(y_val);
                self.set_vreg_val(&x, new_val);
                self.vregs[0xf] = carry as u8;
            }
            Opcode::SR(x, y) => {
                let y_val = self.vreg_val(&y);
                let lsb = y_val & 0x1;
                let new_val = y_val >> 1;
                self.set_vreg_val(&x, new_val);
                self.set_vreg_val(&y, new_val);
                self.vregs[0xf] = lsb;
            }
            Opcode::RSUBR(x, y) => {
                let x_val = self.vreg_val(&x);
                let y_val = self.vreg_val(&y);
                let (new_val, carry) = y_val.overflowing_sub(x_val);
                self.set_vreg_val(&x, new_val);
                self.vregs[0xf] = carry as u8;
            }
            Opcode::SL(x, y) => {
                let y_val = self.vreg_val(&y);
                let msb = (y_val & 0b10000000) >> 7;
                let new_val = y_val << 1;
                self.set_vreg_val(&x, new_val);
                self.set_vreg_val(&y, new_val);
                self.vregs[0xf] = msb;
            }
            Opcode::SKIPRNEQ(x, y) => {
                if self.vreg_val(&x) != self.vreg_val(&y) {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::SI(n) => self.i = n,
            Opcode::JMPR(n) => {
                self.pc = n + (self.vregs[0x0] as u16);
                skip_inc_pc = true;
            }
            Opcode::RAND(x, n) => {
                self.set_vreg_val(&x, random(n));
            }
            Opcode::DRAW(x, y, n) => {
                for height in 0..n {
                    let mut row: u8 = self.mem[(self.i + height as u16) as usize];
                    for width in 0..8 {
                        let val: bool = row & (1 << 7) != 0;
                        row = row << 1;
                        let (x, _) = self.vreg_val(&x).overflowing_add(width);
                        let (y, _) = self.vreg_val(&y).overflowing_add(height);
                        if x < 64 && y < 32 {
                            self.write_pixel(x, y, val);
                        }
                    }
                }
            }
            Opcode::SKIPKEQ(x) => {
                if self.vreg_val(&x) == get_char_fn() {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::SKIPKNEQ(x) => {
                if self.vreg_val(&x) != get_char_fn() {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::GDELAY(x) => {
                let delay = self.delay;
                self.set_vreg_val(&x, delay);
            }
            Opcode::GKEY(x) => {
                self.set_vreg_val(&x, get_char_fn());
            }
            Opcode::SDELAY(x) => {
                let delay = self.vreg_val(&x);
                self.delay = delay;
            }
            Opcode::SSND(x) => {
                let snd = self.vreg_val(&x);
                self.sound = snd;
            }
            Opcode::ADDI(x) => {
                let sum = self.i + self.vreg_val(&x) as u16;
                self.i = sum;
            }
            Opcode::SPRITE(x) => {
                let c = self.vreg_val(&x);
                assert!(c <= 0xf);
                self.i = (c * 5) as u16 + FONT_START;
            }
            Opcode::BCD(x) => {
                let val = self.vreg_val(&x);
                let hundreds: u8 = val / 100;
                let tens: u8 = (val % 100) / 10;
                let ones: u8 = val % 10;
                self.mem[self.i as usize] = hundreds;
                self.mem[(self.i + 1) as usize] = tens;
                self.mem[(self.i + 2) as usize] = ones;
            }
            Opcode::RDUMP(x) => {
                for reg in 0..x.v + 1 {
                    self.mem[(self.i + reg as u16) as usize] = self.vregs[reg as usize];
                }
                self.i += x.v as u16 + 1;
            }
            Opcode::RLOAD(x) => {
                for reg in 0..x.v + 1 {
                    self.vregs[reg as usize] = self.mem[(self.i + reg as u16) as usize];
                }
                self.i += x.v as u16 + 1;
            }
        }
        if !skip_inc_pc {
            self.pc += 2;
        }
    }

    pub fn update_timers(&mut self) {
        let now = Local::now();
        let duration = now.signed_duration_since(self.timer_start);
        let ms = duration.num_milliseconds() as u64;
        let expected_updates = (ms * 60) / 1000;
        let updates_to_makeup = expected_updates - self.timer_updates;
        for _ in 0..updates_to_makeup {
            self.timer_updates += 1;

            if self.sound > 0 {
                self.sound -= 1;
            }
            if self.delay > 0 {
                self.delay -= 0;
            }
        }
    }


    fn vreg_val(&self, vreg: &VReg) -> u8 {
        self.vregs[vreg.v as usize]
    }

    fn set_vreg_val(&mut self, vreg: &VReg, val: u8) {
        self.vregs[vreg.v as usize] = val;
    }

    fn stack_push(&mut self, val: u16) {
        assert_ne!(self.sp, 16);
        let upper: u8 = (val >> 8) as u8;
        let lower: u8 = (val & 0xff) as u8;
        self.mem[(STACK_START + (self.sp * 2) as u16) as usize] = upper;
        self.mem[(STACK_START + (self.sp * 2 + 1) as u16) as usize] = lower;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        assert_ne!(self.sp, 0);
        self.sp -= 1;
        let upper = self.mem[(STACK_START + (self.sp * 2) as u16) as usize];
        let lower = self.mem[(STACK_START + (self.sp * 2 + 1) as u16) as usize];

        ((upper as u16) << 8) | (lower as u16)
    }

    fn decode_pixel(x: u8, y: u8) -> (u16, u8) {
        assert!(x < 64);
        assert!(y < 32);
        let addr = DISP_START as usize + (x as usize / 8) + y as usize * 8;
        assert!(addr <= 0xFFF);
        let mask = 0x1 << (x % 8);
        (addr as u16, mask)
    }

    pub fn pixel_on(&self, x: u8, y: u8) -> bool {
        let (addr, mask) = Chip8State::decode_pixel(x, y);
        self.mem[addr as usize] & mask != 0
    }

    fn write_pixel(&mut self, x: u8, y: u8, val: bool) -> bool {
        let (addr, mask) = Chip8State::decode_pixel(x, y);
        let current = self.mem[addr as usize] & mask != 0;

        if current != val {
            self.mem[addr as usize] = (self.mem[addr as usize] & !mask) | (mask & if val { 0xff } else { 0 });
        }

        !(current && !val)
    }
}

fn random(mask: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let rand = rng.gen::<u8>() & mask;
    rand
}

impl fmt::Debug for Chip8State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Chip8State {{ vregs: {:?}, i: {:?}, sp: {:?}, pc: {:?}, delay: {:?}, sound: {:?} }}",
            self.vregs,
            self.i,
            self.sp,
            self.pc,
            self.delay,
            self.sound
        )
    }
}

impl cmp::PartialEq for Chip8State {
    fn eq(&self, other: &Chip8State) -> bool {
        self.vregs == other.vregs && self.i == other.i && self.sp == other.sp &&
            self.pc == other.pc && self.delay == other.delay && self.sound == other.sound &&
            {
                let mut eq = true;
                for i in 0..self.mem.len() {
                    if self.mem[i] != other.mem[i] {
                        eq = false;
                        break;
                    }
                }
                eq
            }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    fn fake_getchar() -> u8 {0}

    use super::*;

    #[test]
    fn test_load_program() {
        let mut tmp = Chip8State::new();
        tmp.load_program(&Chip8Program::new(&[0u8; 0]));
        assert_eq!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0x00, tmp.mem[0x0200]);
        assert_eq!(0x00, tmp.mem[0x0201]);
        assert_eq!(0x00, tmp.mem[0x0202]);
        tmp.load_program(&Chip8Program::new(&[0xab, 0xcd]));
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0xab, tmp.mem[0x0200]);
        assert_eq!(0xcd, tmp.mem[0x0201]);
        assert_eq!(0x00, tmp.mem[0x0202]);

        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0x00, tmp.mem[0x0200]);
        assert_eq!(0x00, tmp.mem[0x0201]);
        assert_eq!(0x00, tmp.mem[0x0202]);
        assert_eq!(0x00, tmp.mem[0x0203]);
        assert_eq!(0x00, tmp.mem[0x0204]);
        assert_eq!(0x00, tmp.mem[0x0205]);
        tmp.load_program(&Chip8Program::new(&[0xab, 0xcd, 0x12, 0x34]));
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0xab, tmp.mem[0x0200]);
        assert_eq!(0xcd, tmp.mem[0x0201]);
        assert_eq!(0x12, tmp.mem[0x0202]);
        assert_eq!(0x34, tmp.mem[0x0203]);
        assert_eq!(0x00, tmp.mem[0x0204]);
        assert_eq!(0x00, tmp.mem[0x0205]);

        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0x00, tmp.mem[0x0200]);
        assert_eq!(0x00, tmp.mem[0x0201]);
        assert_eq!(0x00, tmp.mem[0x0202]);
        assert_eq!(0x00, tmp.mem[0x0203]);
        assert_eq!(0x00, tmp.mem[0x0204]);
        assert_eq!(0x00, tmp.mem[0x0205]);
        tmp.load_program(&Chip8Program::new(&[0xab, 0xcd, 0x12, 0x34, 0x56]));
        assert_eq!(0x00, tmp.mem[0x01ff]);
        assert_eq!(0xab, tmp.mem[0x0200]);
        assert_eq!(0xcd, tmp.mem[0x0201]);
        assert_eq!(0x12, tmp.mem[0x0202]);
        assert_eq!(0x34, tmp.mem[0x0203]);
        assert_eq!(0x00, tmp.mem[0x0204]);
        assert_eq!(0x00, tmp.mem[0x0205]);
    }

    #[test]
    fn test_eq() {
        assert_eq!(Chip8State::new(), Chip8State::new());

        let mut tmp = Chip8State::new();
        tmp.load_program(&Chip8Program::new(&[0u8; 0]));
        assert_eq!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.load_program(&Chip8Program::new(&[0x00, 0x01]));
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp1 = Chip8State::new();
        let mut tmp2 = Chip8State::new();
        tmp1.load_program(&Chip8Program::new(&[0x00, 0x01]));
        tmp2.load_program(&Chip8Program::new(&[0x00, 0x01]));
        assert_eq!(tmp1, tmp2);

        let mut tmp = Chip8State::new();
        tmp.vregs[3] = 0x34;
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.vregs[3] = 0x00;
        assert_eq!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.i = 0x34;
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.sp = 0x34;
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.pc = 0x34;
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.delay = 0x34;
        assert_ne!(Chip8State::new(), tmp);

        let mut tmp = Chip8State::new();
        tmp.sound = 0x34;
        assert_ne!(Chip8State::new(), tmp);
    }

    #[test]
    fn test_exec_JMP() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x1a, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0abc, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x30, 0x00]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0204, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x30, 0x01]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0202, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPNEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x40, 0x00]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0202, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x40, 0x01]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0204, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPREQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x52, 0x20]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0204, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.vregs[4] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x52, 0x40]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0202, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPRNEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x92, 0x20]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0202, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.vregs[4] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x92, 0x40]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0204, tmp.pc);
    }

    #[test]
    fn test_exec_SI() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0000, tmp.i);
        tmp.load_program(&Chip8Program::new(&[0xaa, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0abc, tmp.i);
    }

    #[test]
    fn test_exec_MOV() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.vregs[5]);
        tmp.load_program(&Chip8Program::new(&[0x65, 0xab]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xab, tmp.vregs[5]);
    }

    #[test]
    fn test_exec_MOVR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0xcd;
        assert_eq!(0x00, tmp.vregs[0x7]);
        tmp.load_program(&Chip8Program::new(&[0x87, 0xa0]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xcd, tmp.vregs[0x7]);
        assert_eq!(0xcd, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0xcd;
        assert_eq!(0x00, tmp.vregs[0x7]);
        tmp.load_program(&Chip8Program::new(&[0x8a, 0x70]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x00, tmp.vregs[0x7]);
        assert_eq!(0x00, tmp.vregs[0xa]);
    }

    #[test]
    fn test_exec_OR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x21]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x5 | 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101010;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x21]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b11111111, tmp.vregs[0x1]);
        assert_eq!(0b10101010, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_AND() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x22]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x5 & 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x22]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b00000001, tmp.vregs[0x1]);
        assert_eq!(0b10101011, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_XOR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x23]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x5 ^ 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x23]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b11111110, tmp.vregs[0x1]);
        assert_eq!(0b10101011, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_ADD() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.vregs[0xa]);
        tmp.load_program(&Chip8Program::new(&[0x7a, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xbc, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x7a, 0xab]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xab + 0x32, tmp.vregs[0xa]);
    }

    #[test]
    fn test_exec_ADDR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x24]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x7, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x34;
        tmp.vregs[0x2] = 0x24;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x24]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x34 + 0x24, tmp.vregs[0x1]);
        assert_eq!(0x24, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0xff;
        tmp.vregs[0x2] = 0x01;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x24]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0, tmp.vregs[0x1]);
        assert_eq!(0x1, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0xff;
        tmp.vregs[0x2] = 0x05;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x24]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x4, tmp.vregs[0x1]);
        assert_eq!(0x5, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);
    }

    #[test]
    fn test_exec_SUBR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x25]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x3, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0xfa;
        tmp.vregs[0x2] = 0x23;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x25]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfa - 0x23, tmp.vregs[0x1]);
        assert_eq!(0x23, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x6;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x25]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xff, tmp.vregs[0x1]);
        assert_eq!(0x6, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x7;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x25]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfe, tmp.vregs[0x1]);
        assert_eq!(0x7, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);
    }

    #[test]
    fn test_exec_RSUBR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x2;
        tmp.vregs[0x2] = 0x5;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x27]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x3, tmp.vregs[0x1]);
        assert_eq!(0x5, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x23;
        tmp.vregs[0x2] = 0xfa;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x27]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfa - 0x23, tmp.vregs[0x1]);
        assert_eq!(0xfa, tmp.vregs[0x2]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x6;
        tmp.vregs[0x2] = 0x5;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x27]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xff, tmp.vregs[0x1]);
        assert_eq!(0x5, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x7;
        tmp.vregs[0x2] = 0x5;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x27]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfe, tmp.vregs[0x1]);
        assert_eq!(0x5, tmp.vregs[0x2]);
        assert_eq!(0x1, tmp.vregs[0xf]);
    }

    #[test]
    fn test_exec_SR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b10101010;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xa6]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b01010101, tmp.vregs[0xa]);
        assert_eq!(0b01010101, tmp.vregs[0x8]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xa6]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b01010101, tmp.vregs[0xa]);
        assert_eq!(0b01010101, tmp.vregs[0x8]);
        assert_eq!(0x1, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xa6, 0x88, 0xa6]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x1, tmp.vregs[0xf]);
        tmp.exec_step(fake_getchar);
        assert_eq!(0b00101010, tmp.vregs[0xa]);
        assert_eq!(0b00101010, tmp.vregs[0x8]);
        assert_eq!(0x1, tmp.vregs[0xf]);
    }

    #[test]
    fn test_exec_SL() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b10101010;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xae]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b01010100, tmp.vregs[0xa]);
        assert_eq!(0b01010100, tmp.vregs[0x8]);
        assert_eq!(0x1, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b00101010;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xae]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0b01010100, tmp.vregs[0xa]);
        assert_eq!(0b01010100, tmp.vregs[0x8]);
        assert_eq!(0x0, tmp.vregs[0xf]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0b00101010;
        tmp.load_program(&Chip8Program::new(&[0x88, 0xae, 0x88, 0xae]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0, tmp.vregs[0xf]);
        tmp.exec_step(fake_getchar);
        assert_eq!(0b10101000, tmp.vregs[0xa]);
        assert_eq!(0b10101000, tmp.vregs[0x8]);
        assert_eq!(0x0, tmp.vregs[0xf]);
    }

    #[test]
    fn test_exec_JMPR() {
        let mut tmp = Chip8State::new();
        tmp.load_program(&Chip8Program::new(&[0xba, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0abc, tmp.pc);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x0] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0xba, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0abc + 0x2, tmp.pc);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x0] = 0xad;
        tmp.load_program(&Chip8Program::new(&[0xba, 0xbc]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x0abc + 0xad, tmp.pc);
    }

    #[test]
    fn test_exec_RAND() {
        fn gen_rand(mask: u8) -> u8 {
            let mut tmp = Chip8State::new();
            tmp.load_program(&Chip8Program::new(&[0xca, mask]));
            tmp.exec_step(fake_getchar);
            return tmp.vregs[0xa];
        }

        for _ in 0..1000 {
            assert!(gen_rand(0b00000001) < 2);
            assert!(gen_rand(0b00000011) < 4);
            assert!(gen_rand(0b00000111) < 8);
            assert!(gen_rand(0b00001111) < 16);
            assert!(gen_rand(0b00011111) < 32);
            assert!(gen_rand(0b00111111) < 64);
            assert!(gen_rand(0b01111111) < 128);
        }
    }

    #[bench]
    fn bench_random(b: &mut Bencher) {
        b.iter(|| random(0xff));
    }

    #[test]
    fn test_exec_GDELAY() {
        let mut tmp = Chip8State::new();
        tmp.delay = 0x21;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x07]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x21, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.delay = 0xfa;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x07]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfa, tmp.vregs[0xa]);
    }

    #[test]
    fn test_exec_SDELAY() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0x21;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x15]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x21, tmp.delay);
        assert_eq!(0x21, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0xfa;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x15]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0xfa, tmp.delay);
        assert_eq!(0xfa, tmp.vregs[0xa]);
    }

    #[test]
    fn test_exec_ADDI() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0x21;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x1e]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x21, tmp.i);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0x21;
        tmp.i = 0xda;
        tmp.load_program(&Chip8Program::new(&[0xfa, 0x1e]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0x21 + 0xda, tmp.i);
    }

    #[test]
    fn test_exec_RDUMP() {
        let mut tmp = Chip8State::new();
        for i in 0..16 {
            tmp.vregs[i] = i as u8 * 2;
        }
        tmp.i = 0x520;
        tmp.load_program(&Chip8Program::new(&[0xf8, 0x55]));
        tmp.exec_step(fake_getchar);
        for i in 0..0x9 {
            assert_eq!(tmp.mem[i as usize + 0x520], i * 2);
        }
        for i in 0x9..16 {
            assert_ne!(tmp.mem[i as usize + 0x520], i * 2);
            assert_eq!(tmp.mem[i as usize + 0x520], 0);
        }
        assert_eq!(tmp.i, 0x520 + 0x9);
    }

    #[test]
    fn test_exec_RLOAD() {
        let mut tmp = Chip8State::new();
        for i in 0..16 {
            tmp.mem[i as usize + 0x520] = i as u8 * 2;
        }
        tmp.i = 0x520;
        tmp.load_program(&Chip8Program::new(&[0xf8, 0x65]));
        tmp.exec_step(fake_getchar);
        for i in 0..0x9 {
            assert_eq!(tmp.vregs[i], i as u8 * 2);
        }
        for i in 0x9..16 {
            assert_ne!(tmp.vregs[i], i as u8 * 2);
            assert_eq!(tmp.vregs[i], 0);
        }
        assert_eq!(tmp.i, 0x520 + 0x9);
    }

    #[test]
    fn test_exec_BCD() {
        let mut tmp = Chip8State::new();
        tmp.i = 0x521;
        tmp.vregs[0x5] = 153;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x33]));
        tmp.exec_step(fake_getchar);
        assert_eq!(1, tmp.mem[0x521]);
        assert_eq!(5, tmp.mem[0x522]);
        assert_eq!(3, tmp.mem[0x523]);
        assert_eq!(0x521, tmp.i);

        let mut tmp = Chip8State::new();
        tmp.i = 0x521;
        tmp.vregs[0x5] = 003;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x33]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0, tmp.mem[0x521]);
        assert_eq!(0, tmp.mem[0x522]);
        assert_eq!(3, tmp.mem[0x523]);
        assert_eq!(0x521, tmp.i);

        let mut tmp = Chip8State::new();
        tmp.i = 0x521;
        tmp.vregs[0x5] = 255;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x33]));
        tmp.exec_step(fake_getchar);
        assert_eq!(2, tmp.mem[0x521]);
        assert_eq!(5, tmp.mem[0x522]);
        assert_eq!(5, tmp.mem[0x523]);
        assert_eq!(0x521, tmp.i);

        let mut tmp = Chip8State::new();
        tmp.i = 0x521;
        tmp.vregs[0x5] = 32;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x33]));
        tmp.exec_step(fake_getchar);
        assert_eq!(0, tmp.mem[0x521]);
        assert_eq!(3, tmp.mem[0x522]);
        assert_eq!(2, tmp.mem[0x523]);
        assert_eq!(0x521, tmp.i);
    }

    #[test]
    fn test_exec_SPRITE() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x5] = 0x0;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x29]));
        tmp.exec_step(fake_getchar);
        assert_eq!(FONT_START, tmp.i);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x5] = 0xa;
        tmp.load_program(&Chip8Program::new(&[0xf5, 0x29]));
        tmp.exec_step(fake_getchar);
        assert_eq!(FONT_START + 5 * 0xa, tmp.i);
    }

    #[test]
    fn test_exec_CLS() {
        let mut tmp = Chip8State::new();
        for i in 0xf00..0xfff + 1 {
            tmp.mem[i as usize] = 0b10101010;
        }
        tmp.load_program(&Chip8Program::new(&[0x00, 0xe0]));
        tmp.exec_step(fake_getchar);
        for i in 0xf00..0xfff + 1 {
            assert_eq!(tmp.mem[i as usize], 0x00);
        }
    }

    #[test]
    fn test_exec_SSND() {
        let mut tmp = Chip8State::new();
        assert_eq!(tmp.sound, 0x00);
        tmp.vregs[0xb] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0xfb, 0x18]));
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.sound, 0x32);
    }

    #[test]
    fn test_exec_CALL() {
        let mut tmp = Chip8State::new();
        assert_eq!(tmp.pc, 0x200);
        assert_eq!(tmp.sp, 0);
        tmp.load_program(&Chip8Program::new(&[0x24, 0x56]));
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.pc, 0x456);
        assert_eq!(tmp.sp, 1);
        assert_eq!(tmp.mem[STACK_START as usize], 0x02);
        assert_eq!(tmp.mem[STACK_START as usize + 1], 0x00);

        let mut tmp = Chip8State::new();
        assert_eq!(tmp.pc, 0x200);
        assert_eq!(tmp.sp, 0);
        tmp.load_program(&Chip8Program::new(&[0x00, 0x00, 0x24, 0x56]));
        tmp.pc += 2;
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.pc, 0x456);
        assert_eq!(tmp.sp, 1);
        assert_eq!(tmp.mem[STACK_START as usize], 0x02);
        assert_eq!(tmp.mem[STACK_START as usize + 1], 0x02);

        let mut tmp = Chip8State::new();
        assert_eq!(tmp.pc, 0x200);
        assert_eq!(tmp.sp, 0);
        tmp.load_program(&Chip8Program::new(&[0x22, 0x02, 0x22, 0x34]));
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.pc, 0x202);
        assert_eq!(tmp.sp, 1);
        assert_eq!(tmp.mem[STACK_START as usize], 0x02);
        assert_eq!(tmp.mem[STACK_START as usize + 1], 0x00);
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.pc, 0x0234);
        assert_eq!(tmp.sp, 2);
        assert_eq!(tmp.mem[STACK_START as usize + 2], 0x02);
        assert_eq!(tmp.mem[STACK_START as usize + 3], 0x02);
    }

    #[test]
    fn test_exec_RET() {
        let mut tmp = Chip8State::new();
        tmp.mem[STACK_START as usize] = 0x05;
        tmp.mem[STACK_START as usize + 1] = 0x67;
        tmp.sp = 1;
        tmp.load_program(&Chip8Program::new(&[0x00, 0xee]));
        tmp.exec_step(fake_getchar);
        assert_eq!(tmp.pc, 0x0567);
        assert_eq!(tmp.sp, 0);
    }
}
