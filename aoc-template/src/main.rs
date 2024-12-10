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
    let data = get_daily_input(FIXME, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "FIXME";
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
