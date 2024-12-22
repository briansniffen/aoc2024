use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use memoize::memoize;
use rayon::prelude::*;
use std::collections::VecDeque;
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

#[memoize]
fn sale_price(i: i64, v: Vec<i64>) -> i64 {
    let mut history = VecDeque::new();
    let mut secret = i;
    for _j in 0..4 {
        history.push_back(secret);
        secret = next(secret);
    }
    for _j in 4..2000 {
        history.push_back(secret % 10);
        secret = next(secret);
        // FIXME test for off-by-one errors
        let diffs = vec![
            history[1] - history[0],
            history[2] - history[1],
            history[3] - history[2],
            history[4] - history[3],
        ];
        history.pop_front();
        if v == diffs {
            return secret % 10;
        }
    }
    return 0;
}

#[time_function]
fn part1(data: &str) -> i64 {
    let mut ans = 0;
    for line in data.lines() {
        ans += many(line.parse::<i64>().expect("parse input"), 2000);
    }
    ans
}

#[time_function]
fn part2(data: &str) -> i64 {
    // sketch: for each monkey, find the sequences that get you 9, 8, 7, etc.
    // calculate a total value for each such sequence, which is about 20k such values.  But values are uniformly distributed, so there should be 200 sequences for each value, so 4m entries.  That's probably too many?
    // but just looking within -3..3 means 7^4 is only 2400.
    // alternate sketch: genetic algorithm working from 0,0,0,0
    let mut price = 0;
    for line in data.lines() {
        let i = line.parse::<i64>().expect("parse input");
        let p = sale_price(i, vec![-2, 1, -1, 3]);
        println!("{}: {}", i, p);
        price += p;
    }
    price
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
