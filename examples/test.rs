use ouroborous::{Instruction, Program};

fn main() {
    let mut instructions = vec![];

    for _ in 0..65 {
        instructions.push(Instruction::Increment);
    }

    for _ in 0..15 {
        instructions.push(Instruction::Output);
    }

    instructions.push(Instruction::Rollback(1));
    instructions.push(Instruction::Output);

    let mut program = Program::new(&instructions);
    program.execute();
}
