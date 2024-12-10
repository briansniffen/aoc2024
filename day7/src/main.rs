use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::error::Error;

fn parse_exprs(data: &str) -> Vec<(u64, Vec<u64>)> {
    data.lines()
        .map(|line| {
            let (goal, rest) = line.split_once(':').expect("expected ':'");
            let nums = rest
                .split_whitespace()
                .map(|n| n.parse().expect("integer elements"))
                .collect();
            (goal.parse().expect("integer goal"), nums)
        })
        .collect()
}

fn reach(goal: &u64, nums: &[u64]) -> bool {
    let goal = *goal;
    let nums = &nums[0..nums.len()];
    if nums.len() == 1 {
        return nums[0] == goal;
    } else {
        let last = nums.last().unwrap();
        let a = if goal % nums.last().unwrap() == 0 {
            reach(&(goal / last), &nums[0..nums.len() - 1])
        } else {
            false
        };
        let b = if goal >= *last {
            reach(&(goal - last), &nums[0..nums.len() - 1])
        } else {
            false
        };
        return a || b;
    }
}

fn reach_concat(goal: u64, nums: &[u64]) -> bool {
    if nums.len() == 1 {
        return nums[0] == goal;
    } else {
        let last = nums.last().unwrap();

        let c = match goal.to_string().strip_suffix(&last.to_string()) {
            Some(shorter) if shorter.len() > 0 => {
                reach_concat(shorter.parse().unwrap(), &nums[0..nums.len() - 1])
            }
            None | Some(_) => false,
        };
        let a = if goal % nums.last().unwrap() == 0 {
            reach_concat(goal / last, &nums[0..nums.len() - 1])
        } else {
            false
        };
        let b = if goal >= *last {
            reach_concat(goal - last, &nums[0..nums.len() - 1])
        } else {
            false
        };
        return c || a || b;
    }
}

#[time_function]
fn part1(data: &str) -> u64 {
    let exprs = parse_exprs(&data);
    exprs
        .iter()
        .filter_map(|(goal, nums)| if reach(goal, nums) { Some(goal) } else { None })
        .sum()
}

#[time_function]
fn part2(data: &str) -> u64 {
    let exprs = parse_exprs(&data);
    exprs
        .par_iter()
        .filter_map(|(goal, nums)| {
            if reach_concat(*goal, nums) {
                Some(goal)
            } else {
                None
            }
        })
        .sum()
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
        assert_eq!(part1(&TESTDATA), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 11387);
    }
}
