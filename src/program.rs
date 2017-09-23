use std::io;
use std::vec::Vec;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Chip8Program {
    pub instructions: Vec<u16>,
}

impl Chip8Program {
    pub fn new(source: &[u8]) -> Chip8Program {
        let mut instructions = vec![];
        if source.len() % 2 == 1 {
            warn!("Creating program ending with half an instruction");
        }
        for i in 0..source.len() / 2 {
            let instruction: u16 = ((source[i * 2] as u16) << 8) | (source[i * 2 + 1] as u16);
            instructions.push(instruction);
        }
        Chip8Program { instructions }
    }

    pub fn from(mut source: Box<io::Read>) -> Result<Chip8Program, io::Error> {
        let mut buf = Vec::new();
        let size = source.read_to_end(&mut buf);
        if let Err(e) = size {
            return Err(e);
        }
        assert_eq!(size.unwrap(), buf.len());
        Ok(Chip8Program::new(&buf[..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Chip8Program { instructions: vec![] },
            Chip8Program::new(&[])
        );
        assert_eq!(
            Chip8Program { instructions: vec![] },
            Chip8Program::new(&[0x12])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x0000] },
            Chip8Program::new(&[0x00, 0x00])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x1234] },
            Chip8Program::new(&[0x12, 0x34])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x1234] },
            Chip8Program::new(&[0x12, 0x34, 0x12])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x1234, 0x1242] },
            Chip8Program::new(&[0x12, 0x34, 0x12, 0x42])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x1234, 0x1242, 0xabcd] },
            Chip8Program::new(&[0x12, 0x34, 0x12, 0x42, 0xab, 0xcd])
        );
        assert_eq!(
            Chip8Program { instructions: vec![0x1234, 0x1242, 0xabcd] },
            Chip8Program::new(&[0x12, 0x34, 0x12, 0x42, 0xab, 0xcd, 0xff])
        );
    }
}
