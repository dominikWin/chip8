use register::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Opcode {
    CLS,
    RET,
    JMP(u16),
    CALL(u16),
    SKIPEQ(VReg, u8),
    SKIPNEQ(VReg, u8),
    SKIPREQ(VReg, VReg),
    MOV(VReg, u8),
    ADD(VReg, u8),
    MOVR(VReg, VReg),
    OR(VReg, VReg),
    AND(VReg, VReg),
    XOR(VReg, VReg),
    ADDR(VReg, VReg),
    SUBR(VReg, VReg),
    SR(VReg, VReg),
    RSUB(VReg, VReg),
    SL(VReg, VReg),
    SKIPRNEQ(VReg, VReg),
    SI(u16),
    PCN(u16),
    RAND(VReg, u8),
    DRAW(VReg, VReg, u8),
    SKIPKEQ(VReg),
    SKIPKNEQ(VReg),
    GDELAY(VReg),
    GKEY(VReg),
    SDELAY(VReg),
    SSND(VReg),
    ADDI(VReg),
    SPRITE(VReg),
    BCD(VReg),
    RDUMP(VReg),
    RLOAD(VReg),
}

impl Opcode {
    pub fn new(cmd: u16) -> Option<Opcode> {
        match cmd {
            0x00e0 => Some(Opcode::CLS),
            0x00ee => Some(Opcode::RET),
            n @ 0x1000 ... 0x1fff => Some(Opcode::JMP(n & 0x0fff)),
            n @ 0x2000 ... 0x2fff => Some(Opcode::CALL(n & 0x0fff)),
            n @ 0x3000 ... 0x3fff => Some(Opcode::SKIPEQ(
                from_int(get_x(n)).unwrap(),
                (n & 0x00ff) as u8,
            )),
            n @ 0x4000 ... 0x4fff => Some(Opcode::SKIPNEQ(
                from_int(get_x(n)).unwrap(),
                (n & 0x00ff) as u8,
            )),
            n @ 0x5000 ... 0x5fff if n & 0x000f == 0 => Some(Opcode::SKIPREQ(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x6000 ... 0x6fff => Some(
                Opcode::MOV(from_int(get_x(n)).unwrap(), (n & 0x00ff) as u8),
            ),
            n @ 0x7000 ... 0x7fff => Some(
                Opcode::ADD(from_int(get_x(n)).unwrap(), (n & 0x00ff) as u8),
            ),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 0 => Some(Opcode::MOVR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 1 => Some(Opcode::OR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 2 => Some(Opcode::AND(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 3 => Some(Opcode::XOR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 4 => Some(Opcode::ADDR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 5 => Some(Opcode::SUBR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 6 => Some(Opcode::SR(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 7 => Some(Opcode::RSUB(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x8000 ... 0x8fff if n & 0x000f == 0xe => Some(Opcode::SL(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0x9000 ... 0x9fff if n & 0x000f == 0 => Some(Opcode::SKIPRNEQ(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
            )),
            n @ 0xa000 ... 0xafff => Some(Opcode::SI(n & 0x0fff)),
            n @ 0xb000 ... 0xbfff => Some(Opcode::PCN(n & 0x0fff)),
            n @ 0xc000 ... 0xcfff => Some(Opcode::RAND(
                from_int(get_x(n)).unwrap(),
                (n & 0x00ff) as u8,
            )),
            n @ 0xd000 ... 0xdfff => Some(Opcode::DRAW(
                from_int(get_x(n)).unwrap(),
                from_int(get_y(n)).unwrap(),
                (n & 0x000f) as u8,
            )),
            n @ 0xe000 ... 0xefff if n & 0x00ff == 0x9e => Some(Opcode::SKIPKEQ(
                from_int(get_x(n)).unwrap(),
            )),
            n @ 0xe000 ... 0xefff if n & 0x00ff == 0xa1 => Some(Opcode::SKIPKNEQ(
                from_int(get_x(n)).unwrap(),
            )),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x07 => Some(Opcode::GDELAY(
                from_int(get_x(n)).unwrap(),
            )),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x0a => Some(
                Opcode::GKEY(from_int(get_x(n)).unwrap()),
            ),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x15 => Some(Opcode::SDELAY(
                from_int(get_x(n)).unwrap(),
            )),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x18 => Some(
                Opcode::SSND(from_int(get_x(n)).unwrap()),
            ),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x1e => Some(
                Opcode::ADDI(from_int(get_x(n)).unwrap()),
            ),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x29 => Some(Opcode::SPRITE(
                from_int(get_x(n)).unwrap(),
            )),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x33 => Some(
                Opcode::BCD(from_int(get_x(n)).unwrap()),
            ),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x55 => Some(
                Opcode::RDUMP(from_int(get_x(n)).unwrap()),
            ),
            n @ 0xf000 ... 0xffff if n & 0x00ff == 0x65 => Some(
                Opcode::RLOAD(from_int(get_x(n)).unwrap()),
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(None, Opcode::new(0x0000));

        assert_eq!(Some(Opcode::CLS), Opcode::new(0x00e0));

        assert_eq!(Some(Opcode::RET), Opcode::new(0x00ee));

        assert_eq!(Some(Opcode::JMP(0x0000)), Opcode::new(0x1000));
        assert_eq!(Some(Opcode::JMP(0x0123)), Opcode::new(0x1123));
        assert_eq!(Some(Opcode::JMP(0x0abf)), Opcode::new(0x1abf));

        assert_eq!(Some(Opcode::CALL(0x0000)), Opcode::new(0x2000));
        assert_eq!(Some(Opcode::CALL(0x0123)), Opcode::new(0x2123));
        assert_eq!(Some(Opcode::CALL(0x0abf)), Opcode::new(0x2abf));

        assert_eq!(Some(Opcode::SKIPEQ(0x0, 0x43)), Opcode::new(0x3043));
        assert_eq!(Some(Opcode::SKIPEQ(0xc, 0x00)), Opcode::new(0x3c00));
        assert_eq!(Some(Opcode::SKIPEQ(0xf, 0xff)), Opcode::new(0x3fff));

        assert_eq!(Some(Opcode::SKIPNEQ(0x0, 0x43)), Opcode::new(0x4043));
        assert_eq!(Some(Opcode::SKIPNEQ(0xc, 0x00)), Opcode::new(0x4c00));
        assert_eq!(Some(Opcode::SKIPNEQ(0xf, 0xff)), Opcode::new(0x4fff));

        assert_eq!(Some(Opcode::SKIPREQ(0x0, 0x4)), Opcode::new(0x5040));
        assert_eq!(Some(Opcode::SKIPREQ(0xc, 0x0)), Opcode::new(0x5c00));
        assert_eq!(Some(Opcode::SKIPREQ(0xf, 0xf)), Opcode::new(0x5ff0));
        assert_eq!(None, Opcode::new(0x5ff1));
        assert_eq!(None, Opcode::new(0x5ffa));

        assert_eq!(Some(Opcode::MOV(0x0, 0x32)), Opcode::new(0x6032));
        assert_eq!(Some(Opcode::MOV(0xf, 0x6c)), Opcode::new(0x6f6c));
        assert_eq!(Some(Opcode::MOV(0xc, 0xff)), Opcode::new(0x6cff));

        assert_eq!(Some(Opcode::ADD(0x0, 0x32)), Opcode::new(0x7032));
        assert_eq!(Some(Opcode::ADD(0xf, 0x6c)), Opcode::new(0x7f6c));
        assert_eq!(Some(Opcode::ADD(0xc, 0xff)), Opcode::new(0x7cff));

        assert_eq!(Some(Opcode::MOVR(0x0, 0x4)), Opcode::new(0x8040));
        assert_eq!(Some(Opcode::MOVR(0xc, 0x0)), Opcode::new(0x8c00));
        assert_eq!(Some(Opcode::MOVR(0xf, 0xf)), Opcode::new(0x8ff0));

        assert_eq!(Some(Opcode::OR(0x0, 0x4)), Opcode::new(0x8041));
        assert_eq!(Some(Opcode::OR(0xc, 0x0)), Opcode::new(0x8c01));
        assert_eq!(Some(Opcode::OR(0xf, 0xf)), Opcode::new(0x8ff1));

        assert_eq!(Some(Opcode::AND(0x0, 0x4)), Opcode::new(0x8042));
        assert_eq!(Some(Opcode::AND(0xc, 0x0)), Opcode::new(0x8c02));
        assert_eq!(Some(Opcode::AND(0xf, 0xf)), Opcode::new(0x8ff2));

        assert_eq!(Some(Opcode::XOR(0x0, 0x4)), Opcode::new(0x8043));
        assert_eq!(Some(Opcode::XOR(0xc, 0x0)), Opcode::new(0x8c03));
        assert_eq!(Some(Opcode::XOR(0xf, 0xf)), Opcode::new(0x8ff3));

        assert_eq!(Some(Opcode::ADDR(0x0, 0x4)), Opcode::new(0x8044));
        assert_eq!(Some(Opcode::ADDR(0xc, 0x0)), Opcode::new(0x8c04));
        assert_eq!(Some(Opcode::ADDR(0xf, 0xf)), Opcode::new(0x8ff4));

        assert_eq!(Some(Opcode::SUBR(0x0, 0x4)), Opcode::new(0x8045));
        assert_eq!(Some(Opcode::SUBR(0xc, 0x0)), Opcode::new(0x8c05));
        assert_eq!(Some(Opcode::SUBR(0xf, 0xf)), Opcode::new(0x8ff5));

        assert_eq!(Some(Opcode::SR(0x0, 0x4)), Opcode::new(0x8046));
        assert_eq!(Some(Opcode::SR(0xc, 0x0)), Opcode::new(0x8c06));
        assert_eq!(Some(Opcode::SR(0xf, 0xf)), Opcode::new(0x8ff6));

        assert_eq!(Some(Opcode::RSUB(0x0, 0x4)), Opcode::new(0x8047));
        assert_eq!(Some(Opcode::RSUB(0xc, 0x0)), Opcode::new(0x8c07));
        assert_eq!(Some(Opcode::RSUB(0xf, 0xf)), Opcode::new(0x8ff7));

        assert_eq!(Some(Opcode::SL(0x0, 0x4)), Opcode::new(0x804e));
        assert_eq!(Some(Opcode::SL(0xc, 0x0)), Opcode::new(0x8c0e));
        assert_eq!(Some(Opcode::SL(0xf, 0xf)), Opcode::new(0x8ffe));

        assert_eq!(Some(Opcode::SKIPRNEQ(0x0, 0x4)), Opcode::new(0x9040));
        assert_eq!(Some(Opcode::SKIPRNEQ(0xc, 0x0)), Opcode::new(0x9c00));
        assert_eq!(Some(Opcode::SKIPRNEQ(0xf, 0xf)), Opcode::new(0x9ff0));
        assert_eq!(None, Opcode::new(0x5ff1));
        assert_eq!(None, Opcode::new(0x5ffa));

        assert_eq!(Some(Opcode::SI(0x0000)), Opcode::new(0xa000));
        assert_eq!(Some(Opcode::SI(0x01af)), Opcode::new(0xa1af));
        assert_eq!(Some(Opcode::SI(0x0fff)), Opcode::new(0xafff));

        assert_eq!(Some(Opcode::PCN(0x0000)), Opcode::new(0xb000));
        assert_eq!(Some(Opcode::PCN(0x01af)), Opcode::new(0xb1af));
        assert_eq!(Some(Opcode::PCN(0x0fff)), Opcode::new(0xbfff));

        assert_eq!(Some(Opcode::RAND(0x0, 0x32)), Opcode::new(0xc032));
        assert_eq!(Some(Opcode::RAND(0xf, 0x6c)), Opcode::new(0xcf6c));
        assert_eq!(Some(Opcode::RAND(0xc, 0xff)), Opcode::new(0xccff));

        assert_eq!(Some(Opcode::DRAW(0x0, 0x3, 0x2)), Opcode::new(0xd032));
        assert_eq!(Some(Opcode::DRAW(0xf, 0x6, 0xc)), Opcode::new(0xdf6c));
        assert_eq!(Some(Opcode::DRAW(0xc, 0xf, 0xf)), Opcode::new(0xdcff));

        assert_eq!(Some(Opcode::SKIPKEQ(0x0)), Opcode::new(0xe09e));
        assert_eq!(Some(Opcode::SKIPKEQ(0xf)), Opcode::new(0xef9e));
        assert_eq!(Some(Opcode::SKIPKEQ(0xc)), Opcode::new(0xec9e));

        assert_eq!(Some(Opcode::SKIPKNEQ(0x0)), Opcode::new(0xe0a1));
        assert_eq!(Some(Opcode::SKIPKNEQ(0xf)), Opcode::new(0xefa1));
        assert_eq!(Some(Opcode::SKIPKNEQ(0xc)), Opcode::new(0xeca1));

        assert_eq!(Some(Opcode::GDELAY(0x0)), Opcode::new(0xf007));
        assert_eq!(Some(Opcode::GDELAY(0xf)), Opcode::new(0xff07));
        assert_eq!(Some(Opcode::GDELAY(0xc)), Opcode::new(0xfc07));

        assert_eq!(Some(Opcode::GKEY(0x0)), Opcode::new(0xf00a));
        assert_eq!(Some(Opcode::GKEY(0xf)), Opcode::new(0xff0a));
        assert_eq!(Some(Opcode::GKEY(0xc)), Opcode::new(0xfc0a));

        assert_eq!(Some(Opcode::SDELAY(0x0)), Opcode::new(0xf015));
        assert_eq!(Some(Opcode::SDELAY(0xf)), Opcode::new(0xff15));
        assert_eq!(Some(Opcode::SDELAY(0xc)), Opcode::new(0xfc15));

        assert_eq!(Some(Opcode::SSND(0x0)), Opcode::new(0xf018));
        assert_eq!(Some(Opcode::SSND(0xf)), Opcode::new(0xff18));
        assert_eq!(Some(Opcode::SSND(0xc)), Opcode::new(0xfc18));

        assert_eq!(Some(Opcode::ADDI(0x0)), Opcode::new(0xf01e));
        assert_eq!(Some(Opcode::ADDI(0xf)), Opcode::new(0xff1e));
        assert_eq!(Some(Opcode::ADDI(0xc)), Opcode::new(0xfc1e));

        assert_eq!(Some(Opcode::SPRITE(0x0)), Opcode::new(0xf029));
        assert_eq!(Some(Opcode::SPRITE(0xf)), Opcode::new(0xff29));
        assert_eq!(Some(Opcode::SPRITE(0xc)), Opcode::new(0xfc29));

        assert_eq!(Some(Opcode::BCD(0x0)), Opcode::new(0xf033));
        assert_eq!(Some(Opcode::BCD(0xf)), Opcode::new(0xff33));
        assert_eq!(Some(Opcode::BCD(0xc)), Opcode::new(0xfc33));

        assert_eq!(Some(Opcode::RDUMP(0x0)), Opcode::new(0xf055));
        assert_eq!(Some(Opcode::RDUMP(0xf)), Opcode::new(0xff55));
        assert_eq!(Some(Opcode::RDUMP(0xc)), Opcode::new(0xfc55));

        assert_eq!(Some(Opcode::RLOAD(0x0)), Opcode::new(0xf065));
        assert_eq!(Some(Opcode::RLOAD(0xf)), Opcode::new(0xff65));
        assert_eq!(Some(Opcode::RLOAD(0xc)), Opcode::new(0xfc65));
    }
}
