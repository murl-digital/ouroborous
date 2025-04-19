pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Output,
    Jump(usize),
    JumpEquals(u8, usize),
    JumpNotEquals(u8, usize),
    Rollback(usize),
    Halt,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
enum Operation {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
}

impl Operation {
    pub fn inverse(&self) -> Self {
        match self {
            Self::MoveLeft => Self::MoveRight,
            Self::MoveRight => Self::MoveLeft,
            Self::Increment => Self::Decrement,
            Self::Decrement => Self::Increment,
        }
    }

    pub fn from_instruction(instruction: Instruction) -> Option<Self> {
        match instruction {
            Instruction::MoveLeft => Some(Self::MoveLeft),
            Instruction::MoveRight => Some(Self::MoveRight),
            Instruction::Increment => Some(Self::Increment),
            Instruction::Decrement => Some(Self::Decrement),
            Instruction::Output
            | Instruction::Jump(_)
            | Instruction::JumpEquals(_, _)
            | Instruction::JumpNotEquals(_, _)
            | Instruction::Rollback(_)
            | Instruction::Halt => None,
        }
    }
}

pub struct Program<'p> {
    instructions: &'p [Instruction],
    program_counter: usize,
    memory: [u8; u16::MAX as usize],
    program_log: Vec<Operation>,
    pointer: u16,
}

impl<'p> Program<'p> {
    pub fn new(instructions: &'p [Instruction]) -> Self {
        Self {
            instructions,
            program_counter: 0,
            memory: [0u8; u16::MAX as usize],
            program_log: Vec::default(),
            pointer: 0u16,
        }
    }
}

impl Program<'_> {
    pub fn execute(&mut self) {
        loop {
            if self.program_counter >= self.instructions.len() {
                //we've reached the end of instructions without halting
                return;
            }

            let instruction = self.instructions[self.program_counter];

            if let Some(operation) = Operation::from_instruction(instruction) {
                self.do_op(operation);
            } else {
                match instruction {
                    Instruction::MoveLeft
                    | Instruction::MoveRight
                    | Instruction::Increment
                    | Instruction::Decrement => unreachable!(),
                    Instruction::Output => {
                        print!("{}", self.memory[self.pointer as usize] as char);
                    }
                    Instruction::Jump(to) => {
                        self.program_counter = to;
                        continue;
                    }
                    Instruction::JumpEquals(value, to)
                        if self.memory[self.pointer as usize] == value =>
                    {
                        self.program_counter = to;
                        continue;
                    }
                    Instruction::JumpNotEquals(value, to)
                        if self.memory[self.pointer as usize] != value =>
                    {
                        self.program_counter = to;
                        continue;
                    }
                    Instruction::Rollback(amt) => {
                        self.rollback(amt);
                    }
                    Instruction::Halt => {
                        return;
                    }
                    Instruction::JumpEquals(_, _) | Instruction::JumpNotEquals(_, _) => {}
                }
            }

            self.program_counter += 1;
        }
    }

    fn do_op(&mut self, op: Operation) {
        self.execute_op(op);

        self.program_log.push(op);
    }

    fn execute_op(&mut self, op: Operation) {
        match op {
            Operation::MoveLeft => self.pointer = self.pointer.wrapping_sub(1),
            Operation::MoveRight => self.pointer = self.pointer.wrapping_add(1),
            Operation::Increment => {
                let val = &mut self.memory[self.pointer as usize];
                *val = val.wrapping_add(1);
            }
            Operation::Decrement => {
                let val = &mut self.memory[self.pointer as usize];
                *val = val.wrapping_sub(1);
            }
        };
    }

    pub fn rollback(&mut self, amt: usize) {
        for _ in 0..amt {
            if let Some(op) = self.program_log.pop() {
                self.execute_op(op.inverse());
            } else {
                return;
            }
        }
    }
}
