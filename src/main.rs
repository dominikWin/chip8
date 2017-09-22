mod opcode;
mod register;

fn main() {
    for i in 0..0xffff {
        if let Some(c) = opcode::Opcode::new(i) {
            println!("{}", c.to_asm());
        }
    }
}
