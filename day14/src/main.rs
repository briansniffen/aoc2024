use aochelpers::{get_daily_input, Coordinate};
use code_timing_macros::time_function;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};
// use rayon::prelude::*;
use std::error::Error;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn coordinate(input: &str) -> IResult<&str, Coordinate<i32>> {
    let (i, (x, y)) = delimited(
        alt((tag("p="), tag("v="))),
        separated_pair(i32, tag(","), i32),
        alt((tag("\n"), tag(" "), tag(""))),
    )(input)?;
    Ok((i, Coordinate { x, y }))
}

fn parse_robot(i: &str) -> IResult<&str, Robot> {
    let (i, pos) = coordinate(i)?;
    let (i, vel) = coordinate(i)?;
    Ok((
        i,
        Robot {
            x: pos.x,
            y: pos.y,
            vx: vel.x,
            vy: vel.y,
        },
    ))
}

fn parse_robots(i: &str) -> IResult<&str, Vec<Robot>> {
    let (i, robots) = many1(parse_robot)(i)?;
    Ok((i, robots))
}

fn simulate(robots: &Vec<Robot>, width: i32, height: i32, ticks: i32) -> Vec<Robot> {
    let mut out = Vec::new();
    for robot in robots {
        let mut new_robot = robot.clone();
        new_robot.x += robot.vx * ticks;
        new_robot.y += robot.vy * ticks;
        new_robot.x = new_robot.x.rem_euclid(width);
        new_robot.y = new_robot.y.rem_euclid(height);
        out.push(new_robot);
    }
    out
}

fn score(robots: &Vec<Robot>, width: i32, height: i32) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots {
        if robot.x < width / 2 && robot.y < height / 2 {
            q1 += 1;
        } else if robot.x > width / 2 && robot.y < height / 2 {
            q2 += 1;
        } else if robot.x < width / 2 && robot.y > height / 2 {
            q3 += 1;
        } else if robot.x > width / 2 && robot.y > height / 2 {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}

#[time_function]
fn part1(robots: &Vec<Robot>, width: i32, height: i32, ticks: i32) -> i32 {
    let robots = simulate(robots, width, height, ticks);
    score(&robots, width, height)
}

#[time_function]
fn part2(robots: &Vec<Robot>, width: i32, height: i32) -> usize {
    (0..(width * height))
        .map(|i| score(&simulate(robots, width, height, i), width, height))
        .enumerate()
        .min_by_key(|(_, score)| *score)
        .map(|(i, _)| i)
        .unwrap()
    // neat things at 28, 86, 129, 189, 230, 292, 331
    // so at 28 + 101x and at 86 + 103x
    //
}

fn print_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }
    print!("\x1B[2J\x1B[1;1H"); // clear screen incant
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(14, 2024)?;
    let (i, robots) = parse_robots(&data).map_err(|e| format!("Parsing error: {:?}", e))?;
    assert_eq!(i, "");
    println!("part1: {}", part1(&robots, 101, 103, 100));
    let p2answer = part2(&robots, 101, 103) as i32;
    println!("part2: {}", p2answer);
    print_robots(&simulate(&robots, 101, 103, p2answer), 101, 103);
    // let mut robots = robots.clone();
    // let mut counter = -2902;
    // if counter > 0 {
    //     robots = simulate(&robots, 101, 103, counter);
    // }
    // loop {
    //     let increment = 1;
    //     counter += increment;
    //     robots = simulate(&robots, 101, 103, increment);
    //     println!("counter: {}", counter);
    //     print_robots(&robots, 101, 103);
    //     pause();
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    use super::*;

    #[test]
    fn test_part1() {
        let (i, robots) = parse_robots(&TESTDATA).unwrap();
        assert_eq!(i, "");
        assert_eq!(part1(&robots, 11, 7, 100), 12);
    }

    #[test]
    fn test_part2() {
        let (i, robots) = parse_robots(&TESTDATA).unwrap();
        assert_eq!(i, "");
        assert_eq!(part2(&robots), 0);
    }
}
