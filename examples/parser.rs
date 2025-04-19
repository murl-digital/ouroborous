use std::fs::read_to_string;

use ouroborous::{Program, parser::parse_program};

fn main() {
    let mut file = read_to_string("example.tbf").unwrap();
    let (_, instructions) = parse_program(&file).unwrap();
    println!("{instructions:?}");
    let mut program = Program::new(&instructions);
    program.execute();
}
