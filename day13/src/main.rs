use aochelpers::{get_daily_input, Coordinate};
use code_timing_macros::time_function;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, u32},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::error::Error;

struct Machine {
    button_a: Coordinate<i64>,
    button_b: Coordinate<i64>,
    prize: Coordinate<i64>,
}

fn coordinate(input: &str) -> IResult<&str, Coordinate<i64>> {
    let (i, (x, y)) = delimited(
        tag("X+"),
        separated_pair(u32, tag(", Y+"), u32),
        alt((tag("\n"), tag(""))),
    )(input)?;
    Ok((
        i,
        Coordinate {
            x: x as i64,
            y: y as i64,
        },
    ))
}

fn eq_coordinate(input: &str) -> IResult<&str, Coordinate<i64>> {
    let (i, (x, y)) = delimited(
        tag("X="),
        separated_pair(u32, tag(", Y="), u32),
        alt((tag("\n"), tag(""))),
    )(input)?;
    Ok((
        i,
        Coordinate {
            x: x as i64,
            y: y as i64,
        },
    ))
}

fn big_eq_coordinate(input: &str) -> IResult<&str, Coordinate<i64>> {
    let (i, (x, y)) = delimited(
        tag("X="),
        separated_pair(u32, tag(", Y="), u32),
        alt((tag("\n"), tag(""))),
    )(input)?;
    Ok((
        i,
        Coordinate {
            x: 10000000000000 + x as i64,
            y: 10000000000000 + y as i64,
        },
    ))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (i, _) = tag("Button A: ")(input)?;
    let (i, button_a) = coordinate(i)?;
    let (i, _) = tag("Button B: ")(i)?;
    let (i, button_b) = coordinate(i)?;
    let (i, _) = tag("Prize: ")(i)?;
    let (i, prize) = eq_coordinate(i)?;
    let (i, _) = multispace0(i)?;
    Ok((
        i,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_big_machine(input: &str) -> IResult<&str, Machine> {
    let (i, _) = tag("Button A: ")(input)?;
    let (i, button_a) = coordinate(i)?;
    let (i, _) = tag("Button B: ")(i)?;
    let (i, button_b) = coordinate(i)?;
    let (i, _) = tag("Prize: ")(i)?;
    let (i, prize) = big_eq_coordinate(i)?;
    let (i, _) = multispace0(i)?;
    Ok((
        i,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_machines(i: &str) -> IResult<&str, Vec<Machine>> {
    let (i, machines) = many1(parse_machine)(i)?;
    Ok((i, machines))
}

fn parse_big_machines(i: &str) -> IResult<&str, Vec<Machine>> {
    let (i, machines) = many1(parse_big_machine)(i)?;
    Ok((i, machines))
}

fn solve(machine: &Machine) -> Option<(i64, i64)> {
    // prize.x = a * button_a.x + b * button_b.x
    // prize.y = a * button_a.y + b * button_b.y
    // xp = a * xa + b * xb
    // yp = a * ya + b * yb
    let xp = machine.prize.x;
    let xa = machine.button_a.x;
    let xb = machine.button_b.x;
    let yp = machine.prize.y;
    let ya = machine.button_a.y;
    let yb = machine.button_b.y;
    let determinant = xa * yb - xb * ya;
    if determinant == 0 {
        return None;
    }
    let a = (yb * xp - yp * xb) / determinant;
    let b = (yp * xa - ya * xp) / determinant;
    if xp == a * xa + b * xb && yp == a * ya + b * yb {
        Some((a, b))
    } else {
        None
    }
}

fn cost(play: (i64, i64)) -> i64 {
    play.0 * 3 + play.1
}

#[time_function]
fn part1(data: &str) -> IResult<&str, i64> {
    let (i, machines) = parse_machines(&data)?;
    Ok((
        i,
        machines
            .iter()
            .map(|m| cost(solve(m).unwrap_or((0, 0))))
            .sum::<i64>(),
    ))
}

#[time_function]
fn part2(data: &str) -> IResult<&str, i64> {
    let (i, machines) = parse_big_machines(&data)?;
    Ok((
        i,
        machines
            .iter()
            .map(|m| cost(solve(m).unwrap_or((0, 0))))
            .sum::<i64>(),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(13, 2024)?;
    let (_, p1answer) = part1(&data).map_err(|e| format!("Parsing error: {:?}", e))?;
    println!("part1: {}", p1answer);
    let (_, p2answer) = part2(&data).map_err(|e| format!("Parsing error: {:?}", e))?;
    println!("part2: {}", p2answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    use super::*;

    #[test]
    fn test_solve() {
        let machine = Machine {
            button_a: Coordinate { x: 94, y: 34 },
            button_b: Coordinate { x: 22, y: 67 },
            prize: Coordinate { x: 8400, y: 5400 },
        };
        assert_eq!(solve(&machine), Some((80, 40)));
        let machine = Machine {
            button_a: Coordinate { x: 26, y: 66 },
            button_b: Coordinate { x: 67, y: 21 },
            prize: Coordinate { x: 12748, y: 12176 },
        };
        assert_eq!(solve(&machine), None);

        let machine = Machine {
            button_a: Coordinate { x: 17, y: 86 },
            button_b: Coordinate { x: 84, y: 37 },
            prize: Coordinate { x: 7870, y: 6450 },
        };
        assert_eq!(solve(&machine), Some((38, 86)));

        let machine = Machine {
            button_a: Coordinate { x: 69, y: 23 },
            button_b: Coordinate { x: 27, y: 71 },
            prize: Coordinate { x: 18641, y: 10279 },
        };
        assert_eq!(solve(&machine), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), Ok(("", 480)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), Ok(("", 875318608908)));
    }
}
