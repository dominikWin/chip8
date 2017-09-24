use program::Chip8Program;
use std::fmt;
use std::cmp;

pub struct Chip8State {
    pub vregs: [u8; 16],
    pub i: u16,
    pub sp: u16,
    pub pc: u16,
    pub delay: u8,
    pub sound: u8,
    pub mem: [u8; 0x1000],
}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State {
            vregs: [0; 16],
            i: 0x0000,
            sp: 0x0EA0,
            pc: 0x0200,
            delay: 0,
            sound: 0,
            mem: [0; 0x1000],
        }
    }

    pub fn load_program(&mut self, program: &Chip8Program) {
        let mut addr = 0x0200;
        for instruction in program.instructions.iter() {
            self.mem[addr] = ((instruction & 0xff00) >> 8) as u8;
            self.mem[addr + 1] = (instruction & 0x00ff) as u8;
            addr += 2;
        }
    }
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

#[cfg(test)]
mod tests {
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
}
