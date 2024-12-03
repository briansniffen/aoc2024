use aochelpers::get_daily_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    multi::fold_many1,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::error::Error;
use std::ops::Add;

fn mul(i: &str) -> IResult<&str, u32> {
    let (i, (a, b)) = delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(i)?;
    Ok((i, a * b))
}

fn trash(i: &str) -> IResult<&str, u32> {
    let (i, _) = anychar(i)?;
    Ok((i, 0))
}

fn part1(i: &str) -> IResult<&str, u32> {
    fold_many1(alt((mul, trash)), || 0, u32::add)(i)
}

enum Instr {
    Mul { v: u32 },
    Do,
    Dont,
    Trash,
}
use Instr::*;

fn mul2(i: &str) -> IResult<&str, Instr> {
    let (i, (a, b)) = delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(i)?;
    Ok((i, Mul { v: a * b }))
}

fn do2(i: &str) -> IResult<&str, Instr> {
    let (i, _) = tag("do()")(i)?;
    Ok((i, Do))
}

fn dont2(i: &str) -> IResult<&str, Instr> {
    let (i, _) = tag("don't()")(i)?;
    Ok((i, Dont))
}

fn trash2(i: &str) -> IResult<&str, Instr> {
    let (i, _) = anychar(i)?;
    Ok((i, Trash))
}

#[derive(Clone, Copy, Debug)]
struct State {
    total: u32,
    on: bool,
}

fn interpreter(state: State, instr: Instr) -> State {
    match instr {
        Trash => state,
        Do => State { on: true, ..state },
        Dont => State { on: false, ..state },
        Mul { v } => State {
            total: state.total + if state.on { v } else { 0 },
            ..state
        },
    }
}

fn part2(i: &str) -> IResult<&str, u32> {
    let state = State { total: 0, on: true };
    let (i, state) = fold_many1(alt((mul2, do2, dont2, trash2)), || state, interpreter)(i)?;
    Ok((i, state.total))
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(3, 2024)?;
    let (_, p1answer) = part1(&data).map_err(|e| format!("Parsing error: {:?}", e))?;
    println!("part1: {}", p1answer);
    let (_, p2answer) = part2(&data).map_err(|e| format!("Parsing error: {:?}", e))?;
    println!("part2: {}", p2answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const NOTRASH: &str = "mul(2,4)mul(5,5)mul(11,8)mul(8,5)";
    const PART_TWO: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    use super::*;

    #[test]
    fn parse_mul() {
        assert_eq!(mul("mul(13,11)"), Ok(("", 143)));
    }

    #[test]
    fn test_notrash() {
        assert_eq!(part1(NOTRASH), Ok(("", 161)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTDATA), Ok(("", 161)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_TWO), Ok(("", 48)));
    }
}
