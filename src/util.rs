use program::Chip8Program;
use std::io;
use std::fs;
use clap::ArgMatches;

pub fn program_from_jnput(matches: &ArgMatches) -> Result<Chip8Program, io::Error> {
    let input: Box<io::Read> = {
        let input_val = matches.value_of("input").unwrap();
        if input_val == "-" {
            Box::new(io::stdin())
        } else {
            let file = fs::File::open(input_val);
            if let Err(e) = file {
                println!(
                    "File {} can't be opened: {}",
                    matches.value_of("input").unwrap(),
                    e
                );
                return Err(e);
            }
            Box::new(file.unwrap())
        }
    };

    Chip8Program::from(input)
}

pub fn filled_hex(n: u16) -> String {
    format!(
        "{:x}{:x}{:x}{:x}",
        (n & 0xf000) >> 12,
        (n & 0x0f00) >> 8,
        (n & 0x00f0) >> 4,
        (n & 0x000f)
    )
}

pub fn filled_hex_dual(n: u16) -> (String, String) {
    (
        format!("{:x}{:x}", (n & 0xf000) >> 12, (n & 0x0f00) >> 8),
        format!("{:x}{:x}", (n & 0x00f0) >> 4, (n & 0x000f)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filled_hex() {
        assert_eq!("0000".to_string(), filled_hex(0x0000));
        assert_eq!("1234".to_string(), filled_hex(0x1234));
        assert_eq!("0ab0".to_string(), filled_hex(0x0ab0));
        assert_eq!("ffff".to_string(), filled_hex(0xffff));
    }

    #[test]
    fn test_filled_hex_dual() {
        assert_eq!(
            ("00".to_string(), "00".to_string()),
            filled_hex_dual(0x0000)
        );
        assert_eq!(
            ("12".to_string(), "34".to_string()),
            filled_hex_dual(0x1234)
        );
        assert_eq!(
            ("0a".to_string(), "b0".to_string()),
            filled_hex_dual(0x0ab0)
        );
        assert_eq!(
            ("ff".to_string(), "ff".to_string()),
            filled_hex_dual(0xffff)
        );
    }
}
