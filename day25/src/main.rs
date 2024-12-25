use aochelpers::{get_daily_input,parse_number_grid,Coordinate};
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use either::*;

#[derive(Debug,Clone,PartialEq,Eq)]
struct Lock {
    key: bool,
    pins: Vec<usize>,
    height: usize,
}
type Key = Lock;

fn parse_input(data: &str) -> Vec<Lock> {
    let mut ans=Vec::new();
    for grid in data.split("\n\n") {
        let grid: HashMap<Coordinate<usize>, char> = parse_number_grid(&grid);
        let height = grid.keys().map(|c| c.y).max().unwrap();
        let width = grid.keys().map(|c| c.x).max().unwrap();
        let key = grid[&Coordinate{x: 0, y: 0}] != '#';
        let mut pins: Vec<usize> = Vec::new();
        for x in 0..=width {
            let pick = if key {Left((0..=height).rev())} else {Right(0..=height)};
            for y in pick {
                if grid[&Coordinate{x,y}] == '.' {
                    let pin = if key {height-(y+1)} else {y-1};
                    pins.push(pin);
                    break;
                }
            }
        };
        ans.push(Lock {key,pins,height});
    }
    ans
}

fn fits(lock: &Lock, key: &Key) -> bool {
    assert!(!lock.key);
    assert!(key.key);
    assert_eq!(lock.pins.len(),key.pins.len());
    for x in 0..lock.pins.len() {
        if lock.pins[x]+key.pins[x]>=lock.height {
            return false;
        }
    }
    true
}

#[time_function]
fn part1(data: &str) -> usize {
    let (keys,locks):(_,Vec<_>) = parse_input(&data).into_iter().partition(|c| c.key);
    locks.par_iter().map(|lock| {
        keys.iter().filter(|key| fits(lock,key)).count()
    }).sum()
}

#[time_function]
fn part2(_data: &str) -> usize {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(25, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    use super::*;

    #[test]
    fn test_parse() {
        let locks = parse_input(&TESTDATA);
        assert_eq!(locks[0].pins,vec![0,5,3,4,3]);
        assert_eq!(locks[2].pins,vec![5,0,2,1,3]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
