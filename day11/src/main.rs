use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use memoize::memoize;
use rayon::prelude::*;
use std::error::Error;

fn blink(i: u64) -> Vec<u64> {
    if i == 0 {
        vec![1]
    } else {
        let s = i.to_string();
        let l = s.len();
        if l % 2 == 0 {
            vec![s[..l / 2].parse().unwrap(), s[l / 2..].parse().unwrap()]
        } else {
            vec![i * 2024]
        }
    }
}

#[memoize]
fn run(i: u64, n: u64) -> Vec<u64> {
    if n == 0 {
        vec![i]
    } else if n == 1 {
        blink(i)
    } else {
        blink(i).into_iter().flat_map(|i| run(i, n - 1)).collect()
    }
}

#[time_function]
fn part1(data: &str) -> usize {
    let stones: Vec<u64> = data
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut total = 0;
    for stone in stones {
        total += run(stone, 25).len();
    }
    total
}

#[time_function]
fn part2(data: &str) -> usize {
    let stones: Vec<u64> = data
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut total = 0;
    for stone in stones {
        total += run(stone, 75).len();
    }
    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(11, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "125 17";
    use super::*;

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
        assert_eq!(blink(10), vec![1, 0]);
        assert_eq!(blink(99), vec![9, 9]);
        assert_eq!(blink(999), vec![2021976]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 55312);
    }
}
