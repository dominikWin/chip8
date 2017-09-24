use program::Chip8Program;
use std::fmt;
use std::cmp;
use opcode::Opcode;
use register::VReg;

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

    pub fn exec_step(&mut self) {
        let instruction: u16 = ((self.mem[self.pc as usize] as u16) << 8) |
            ((self.mem[self.pc as usize + 1]) as u16);
        let opcode = Opcode::new(instruction);
        if let None = opcode {
            panic!("Failed to decode instruction {:x}", instruction);
        }
        let opcode = opcode.unwrap();
        let mut skip_inc_pc = false;
        match opcode {
            Opcode::CLS => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::RET => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::JMP(n) => {
                self.pc = n;
                skip_inc_pc = true;
            }
            Opcode::CALL(_) => panic!("Call to non-implemented instruction {:?}", opcode),
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
                let new_val = x_val + n;
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
            Opcode::ADDR(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SUBR(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SR(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::RSUBR(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SL(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SKIPRNEQ(x, y) => {
                if self.vreg_val(&x) != self.vreg_val(&y) {
                    self.pc += 4;
                    skip_inc_pc = true;
                }
            }
            Opcode::SI(n) => self.i = n,
            Opcode::PCN(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::RAND(_, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::DRAW(_, _, _) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SKIPKEQ(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SKIPKNEQ(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::GDELAY(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::GKEY(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SDELAY(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SSND(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::ADDI(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::SPRITE(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::BCD(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::RDUMP(_) => panic!("Call to non-implemented instruction {:?}", opcode),
            Opcode::RLOAD(_) => panic!("Call to non-implemented instruction {:?}", opcode),
        }
        if !skip_inc_pc {
            self.pc += 2;
        }
    }

    fn vreg_val(&self, vreg: &VReg) -> u8 {
        self.vregs[vreg.v as usize]
    }

    fn set_vreg_val(&mut self, vreg: &VReg, val: u8) {
        self.vregs[vreg.v as usize] = val;
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

    #[test]
    fn test_exec_JMP() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x1a, 0xbc]));
        tmp.exec_step();
        assert_eq!(0x0abc, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x30, 0x00]));
        tmp.exec_step();
        assert_eq!(0x0204, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x30, 0x01]));
        tmp.exec_step();
        assert_eq!(0x0202, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPNEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x40, 0x00]));
        tmp.exec_step();
        assert_eq!(0x0202, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x40, 0x01]));
        tmp.exec_step();
        assert_eq!(0x0204, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPREQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x52, 0x20]));
        tmp.exec_step();
        assert_eq!(0x0204, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.vregs[4] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x52, 0x40]));
        tmp.exec_step();
        assert_eq!(0x0202, tmp.pc);
    }

    #[test]
    fn test_exec_SKIPRNEQ() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.load_program(&Chip8Program::new(&[0x92, 0x20]));
        tmp.exec_step();
        assert_eq!(0x0202, tmp.pc);

        let mut tmp = Chip8State::new();
        assert_eq!(0x0200, tmp.pc);
        tmp.vregs[4] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x92, 0x40]));
        tmp.exec_step();
        assert_eq!(0x0204, tmp.pc);
    }

    #[test]
    fn test_exec_SI() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x0000, tmp.i);
        tmp.load_program(&Chip8Program::new(&[0xaa, 0xbc]));
        tmp.exec_step();
        assert_eq!(0x0abc, tmp.i);
    }

    #[test]
    fn test_exec_MOV() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.vregs[5]);
        tmp.load_program(&Chip8Program::new(&[0x65, 0xab]));
        tmp.exec_step();
        assert_eq!(0xab, tmp.vregs[5]);
    }

    #[test]
    fn test_exec_MOVR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0xcd;
        assert_eq!(0x00, tmp.vregs[0x7]);
        tmp.load_program(&Chip8Program::new(&[0x87, 0xa0]));
        tmp.exec_step();
        assert_eq!(0xcd, tmp.vregs[0x7]);
        assert_eq!(0xcd, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0xcd;
        assert_eq!(0x00, tmp.vregs[0x7]);
        tmp.load_program(&Chip8Program::new(&[0x8a, 0x70]));
        tmp.exec_step();
        assert_eq!(0x00, tmp.vregs[0x7]);
        assert_eq!(0x00, tmp.vregs[0xa]);
    }

    #[test]
    fn test_exec_OR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x21]));
        tmp.exec_step();
        assert_eq!(0x5 | 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101010;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x21]));
        tmp.exec_step();
        assert_eq!(0b11111111, tmp.vregs[0x1]);
        assert_eq!(0b10101010, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_AND() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x22]));
        tmp.exec_step();
        assert_eq!(0x5 & 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x22]));
        tmp.exec_step();
        assert_eq!(0b00000001, tmp.vregs[0x1]);
        assert_eq!(0b10101011, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_XOR() {
        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0x5;
        tmp.vregs[0x2] = 0x2;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x23]));
        tmp.exec_step();
        assert_eq!(0x5 ^ 0x2, tmp.vregs[0x1]);
        assert_eq!(0x2, tmp.vregs[0x2]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0x1] = 0b01010101;
        tmp.vregs[0x2] = 0b10101011;
        tmp.load_program(&Chip8Program::new(&[0x81, 0x23]));
        tmp.exec_step();
        assert_eq!(0b11111110, tmp.vregs[0x1]);
        assert_eq!(0b10101011, tmp.vregs[0x2]);
    }

    #[test]
    fn test_exec_ADD() {
        let mut tmp = Chip8State::new();
        assert_eq!(0x00, tmp.vregs[0xa]);
        tmp.load_program(&Chip8Program::new(&[0x7a, 0xbc]));
        tmp.exec_step();
        assert_eq!(0xbc, tmp.vregs[0xa]);

        let mut tmp = Chip8State::new();
        tmp.vregs[0xa] = 0x32;
        tmp.load_program(&Chip8Program::new(&[0x7a, 0xab]));
        tmp.exec_step();
        assert_eq!(0xab + 0x32, tmp.vregs[0xa]);
    }
}
