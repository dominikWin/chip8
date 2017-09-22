#[derive(Debug)]
#[derive(PartialEq)]
pub struct VReg {
    v: u8,
}

pub fn from_int(oc: u8) -> Option<VReg> {
    if oc > 0xf {
        Option::None
    } else {
        Some(VReg { v: oc })
    }
}

pub fn get_x(oc: u16) -> u8 {
    ((oc >> 8) & 0x0f) as u8
}

pub fn get_y(oc: u16) -> u8 {
    ((oc >> 4) & 0x0f) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        assert_eq!(Some(VReg { v: 0 }), from_int(0));
        assert_eq!(Some(VReg { v: 0xf }), from_int(0xf));
        assert_eq!(None, from_int(0xf + 1));
        assert_eq!(None, from_int(0x43));
    }

    #[test]
    fn test_get_x() {
        assert_eq!(0x0, get_x(0x3043));
        assert_eq!(0x8, get_x(0x4820));
        assert_eq!(0xc, get_x(0x7c37));
    }

    #[test]
    fn test_get_y() {
        assert_eq!(0x4, get_y(0x3043));
        assert_eq!(0x2, get_y(0x4820));
        assert_eq!(0xf, get_y(0x7cf7));
    }
}
