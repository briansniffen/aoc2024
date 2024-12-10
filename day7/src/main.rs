use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::error::Error;

#[time_function]
fn part1(data: &str) -> u32 {
    0
}

#[time_function]
fn part2(data: &str) -> u32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(7, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
