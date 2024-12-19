use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use dashmap::DashMap;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;

fn parse_input(data: &str) -> (Regex, Vec<String>) {
    let mut lines = data.lines();
    let regex = Regex::new(&format!(
        r"^({})*$",
        lines.next().unwrap().replace(", ", "|")
    ))
    .unwrap();
    let _ = lines.next(); // blank line
    let mut data = Vec::new();
    for line in lines {
        data.push(line.to_string());
    }
    (regex, data)
}

fn parse_input2(data: &str) -> (HashSet<String>, Vec<String>) {
    let mut lines = data.lines();
    let mut patterns = HashSet::new();
    for word in lines.next().expect("no pattern input").split(", ") {
        patterns.insert(word.to_string());
    }
    let _ = lines.next(); // blank line
    let mut data = Vec::new();
    for line in lines {
        data.push(line.to_string());
    }
    (patterns, data)
}

fn count_ways(patterns: &HashSet<String>, towel: &str, cache: &DashMap<String, u64>) -> u64 {
    if towel.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(towel) {
        return *count;
    }
    let count = (1..=towel.len())
        .filter(|&tailend| patterns.contains(&towel[..tailend]))
        .map(|tailend| count_ways(patterns, &towel[tailend..], cache))
        .sum();
    cache.insert(towel.to_string(), count);
    count
}

#[time_function]
fn part1(data: &str) -> u32 {
    let (regex, towels) = parse_input(data);
    towels
        .par_iter()
        .map(|line| if regex.is_match(line) { 1 } else { 0 })
        .sum::<u32>()
}

#[time_function]
fn part2(data: &str) -> u64 {
    let (patterns, towels) = parse_input2(data);
    let cache = Arc::new(DashMap::new());
    towels
        .par_iter()
        .map(|towel| count_ways(&patterns, towel, &cache))
        .sum::<u64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(19, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 16);
    }
}
