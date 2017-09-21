pub type VReg = u8;

pub fn from_int(oc: u8) -> Option<VReg> {
    if oc > 0xf { Option::None } else { Some(oc) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        assert_eq!(Some(0), from_int(0));
        assert_eq!(Some(0xf), from_int(0xf));
        assert_eq!(None, from_int(0xf+1));
        assert_eq!(None, from_int(0x43));
    }
}