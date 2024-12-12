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
    let data = get_daily_input(10, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_map(&TESTDATA).count_trailheads, 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
