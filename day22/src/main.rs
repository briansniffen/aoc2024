use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::iter::successors;

fn next(i: &i64) -> Option<i64> {
    let i = ((i << 06) ^ i) % 0x100_0000;
    let i = ((i >> 05) ^ i) % 0x100_0000;
    let i = ((i << 11) ^ i) % 0x100_0000;
    Some(i)
}

#[time_function]
fn part1(data: &str) -> i64 {
    let mut ans = 0;
    for line in data.lines() {
        let mut monkey = successors(line.parse::<i64>().ok(), next).skip(2000);
        ans += monkey.next().expect("infinite monkey");
    }
    ans
}

// first, brute force approach:
// bounded to -3..=3, we get 1504 in 185s.
// memoize takes 332s to get the same answer!
// let's go the other way...

#[time_function]
fn part2(data: &str) -> i64 {
    let data: Vec<i64> = data
        .lines()
        .map(|line| line.parse::<i64>().expect("parse input"))
        .collect();
    let market = DashMap::new();
    data.into_par_iter().for_each(|monkey| {
        let monkey: Vec<i64> = successors(Some(monkey), next).take(2000).collect();
        let mut seen = HashSet::new();
        for window in monkey.windows(5) {
            let diffs = vec![
                window[1] % 10 - window[0] % 10,
                window[2] % 10 - window[1] % 10,
                window[3] % 10 - window[2] % 10,
                window[4] % 10 - window[3] % 10,
            ];
            if seen.contains(&diffs) {
                continue;
            }
            seen.insert(diffs.clone());
            let price = window[4] % 10;
            market
                .entry(diffs.clone())
                .and_modify(|n| *n += price)
                .or_insert(price);
        }
    });
    *market.into_read_only().values().max().expect("some max")
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(22, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "1
10
100
2024";
    const TEST2: &str = "1
2
3
2024";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST2), 23);
    }
}
