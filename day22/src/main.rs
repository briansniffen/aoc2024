use aochelpers::get_daily_input;
use code_timing_macros::time_function;
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

fn sale_price(i: i64, a: i64, b: i64, c: i64, d: i64) -> i64 {
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
        if vec![a, b, c, d] == diffs {
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

// bounded to -3..=3, we get 1504 in 185s.
// memoize takes 332s to get the same answer!

#[time_function]
fn part2(data: &str) -> i64 {
    // sketch: for each monkey, find the sequences that get you 9, 8, 7, etc.
    // calculate a total value for each such sequence, which is about 20k such values.  But values are uniformly distributed, so there should be 200 sequences for each value, so 4m entries.  That's probably too many?
    // but just looking within -3..3 means 7^4 is only 2400.
    // alternate: pre-calculate a map for each input of signal->value:
    // HashMap from the 4 differences before the number to the number itself, then merge all the maps together and find the max value. Ran in around 450ms on my M2 macbook air, down to around 150ms after parallelizing with rayon.
    let data: Vec<i64> = data
        .lines()
        .map(|line| line.parse::<i64>().expect("parse input"))
        .collect();
    //    let mut best_signal = vec![0, 0, 0, 0];
    //    for a in -3..=3 {
    (-9..9)
        .into_par_iter()
        .map(|a| {
            let mut best_price = 0;
            for b in -9..=9 {
                for c in -9..=9 {
                    for d in -9..=9 {
                        //                    let signal = vec![a, b, c, d];
                        let mut price = 0;
                        for i in data.iter() {
                            let p = sale_price(*i, a, b, c, d);
                            price += p;
                        }
                        if price > best_price {
                            best_price = price;
                        }
                    }
                }
            }
            best_price
        })
        .max()
        .expect("no max found")
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
