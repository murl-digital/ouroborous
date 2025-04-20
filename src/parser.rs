use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::{complete::multispace0, digit1, streaming::char},
    combinator::{complete, map_res, value},
    error::{Error, ParseError},
    multi::many0,
    sequence::preceded,
};

use crate::Instruction;

fn increment(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Increment, char::<&str, Error<_>>('+')).parse(input)
}

fn decrement(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Decrement, char::<&str, Error<_>>('-')).parse(input)
}

fn move_right(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::MoveRight, char::<&str, Error<_>>('>')).parse(input)
}

fn move_left(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::MoveLeft, char::<&str, Error<_>>('<')).parse(input)
}

fn output(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Output, char::<&str, Error<_>>('.')).parse(input)
}

fn halt(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Halt, tag("💥")).parse(input)
}

fn jump_equals(input: &str) -> IResult<&str, Instruction> {
    let (input, (val, dest)) = (
        preceded(char('?'), map_res(digit1(), str::parse)),
        preceded(char(','), map_res(digit1(), str::parse)),
    )
        .parse_complete(input)?;

    Ok((input, Instruction::JumpEquals(val, dest)))
}

fn jump_not_equals(input: &str) -> IResult<&str, Instruction> {
    let (input, (val, dest)) = (
        preceded(char('!'), map_res(digit1(), str::parse)),
        preceded(char(','), map_res(digit1(), str::parse)),
    )
        .parse_complete(input)?;

    Ok((input, Instruction::JumpNotEquals(val, dest)))
}

fn rollback(input: &str) -> IResult<&str, Instruction> {
    let (input, amt) = preceded(tag("🦖"), map_res(digit1(), str::parse)).parse_complete(input)?;

    Ok((input, Instruction::Rollback(amt)))
}

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    preceded(multispace0, inner)
}

pub fn parse_program(program: &str) -> IResult<&str, Vec<Instruction>> {
    many0(complete(ws(alt((
        increment,
        decrement,
        move_right,
        move_left,
        output,
        halt,
        jump_equals,
        jump_not_equals,
        rollback,
    )))))
    .parse(program)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let program = "";
        let (_, result) = parse_program(program).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn increment_one() {
        let program = "+";
        let (_, result) = parse_program(program).unwrap();

        assert_eq!(result, vec![Instruction::Increment]);
    }

    #[test]
    fn increment_many() {
        let program = "+++++";
        let (_, result) = parse_program(program).unwrap();

        assert_eq!(result, vec![Instruction::Increment; 5]);
    }

    #[test]
    fn jump_equals() {
        let program = "?69,420";
        let (_, result) = parse_program(program).unwrap();

        assert_eq!(result, vec![Instruction::JumpEquals(69, 420)])
    }

    #[test]
    fn rollback() {
        let program = "🦖69";
        let (_, result) = parse_program(program).unwrap();

        assert_eq!(result, vec![Instruction::Rollback(69)])
    }
}
