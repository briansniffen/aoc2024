use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;

fn next(i: i64) -> i64 {
    let i = ((i << 6) ^ i) % 16777216;
    let i = ((i >> 5) ^ i) % 16777216;
    let i = ((i << 11) ^ i) % 16777216;
    i
}

fn many(i: i64, n: i64) -> i64 {
    let mut ans = i;
    for _j in 0..n {
        ans = next(ans);
    }
    ans
}

fn many_vec(i: i64, n: i64) -> Vec<i64> {
    let mut ans = Vec::new();
    let mut i = i;
    for _j in 0..n {
        ans.push(i);
        i = next(i);
    }
    ans
}

#[time_function]
fn part1(data: &str) -> i64 {
    let mut ans = 0;
    for line in data.lines() {
        ans += many(line.parse::<i64>().expect("parse input"), 2000);
    }
    ans
}

// first, brute force approach:
// bounded to -3..=3, we get 1504 in 185s.
// memoize takes 332s to get the same answer!

#[time_function]
fn part2(data: &str) -> i64 {
    let data: Vec<i64> = data
        .lines()
        .map(|line| line.parse::<i64>().expect("parse input"))
        .collect();
    let market = DashMap::new();
    data.into_par_iter().for_each(|monkey| {
        let monkey = many_vec(monkey, 2000);
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
