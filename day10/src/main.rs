use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use std::collections::{HashMap, HashSet};
use std::error::Error;

struct Map {
    grid: HashMap<Coordinate<i32>, char>,
}

type Path = Vec<Coordinate<i32>>;

impl Map {
    fn count_trailheads(&self) -> usize {
        self.grid.iter().filter(|(_, &c)| c == '0').count()
    }

    fn trailheads(&self) -> Vec<Coordinate<i32>> {
        self.grid
            .iter()
            .filter(|(_, &c)| c == '0')
            .map(|(c, _)| c.clone())
            .collect()
    }

    fn paths_from(&self, trailhead: Coordinate<i32>) -> Vec<Path> {
        let mut paths: Vec<Path> = Vec::new();
        let mut level = self.grid[&trailhead];
        paths.push(vec![trailhead]);

        while level < '9' {
            let mut new_paths = Vec::new();
            for path in paths.into_iter() {
                let neighbours =
                    path.last().unwrap().neighbours().into_iter().filter(|c| {
                        match self.grid.get(c) {
                            Some(ch) if *ch == (level as u8 + 1) as char => true,
                            _ => false,
                        }
                    });
                for neighbour in neighbours.into_iter() {
                    if self.grid.get(&neighbour) == None {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(neighbour);
                    new_paths.push(new_path);
                }
            }
            paths = new_paths;
            level = (level as u8 + 1) as char;
        }
        paths
    }
}

fn parse_map(data: &str) -> Map {
    let grid = parse_number_grid(data);
    Map { grid }
}

#[time_function]
fn part1(data: &str) -> usize {
    let map = parse_map(data);
    let mut sum = 0;
    for trailhead in map.trailheads() {
        let mut finals: HashSet<Coordinate<i32>> = HashSet::new();
        let paths = map.paths_from(trailhead);
        for path in paths {
            finals.insert(path.last().unwrap().clone());
        }
        sum += finals.len();
    }
    sum
}

#[time_function]
fn part2(data: &str) -> usize {
    let map = parse_map(data);
    let mut sum = 0;
    for trailhead in map.trailheads() {
        let paths = map.paths_from(trailhead);
        sum += paths.len();
    }
    sum
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
        assert_eq!(parse_map(&TESTDATA).count_trailheads(), 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 81);
    }
}
