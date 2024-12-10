use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Clone, Debug)]
struct Map {
    antennae: HashMap<char, HashSet<Coordinate<i32>>>,
    width: i32,
    height: i32,
}

impl Map {
    fn contains(&self, p: Coordinate<i32>) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }
}

// Antinodes of two antennae
fn antinodes2(a: &Coordinate<i32>, b: &Coordinate<i32>) -> (Coordinate<i32>, Coordinate<i32>) {
    let diff = *a - *b;
    (*b - diff, *a + diff)
}

// All the antinodes of a set of antennae, filtered to be on the map
fn antinodes(map: &Map, antennae: &HashSet<Coordinate<i32>>) -> HashSet<Coordinate<i32>> {
    let mut answer = HashSet::new();
    for a in antennae.iter() {
        for b in antennae.iter() {
            if a != b {
                let (c, d) = antinodes2(a, b);
                if map.contains(c) {
                    answer.insert(c);
                };
                if map.contains(d) {
                    answer.insert(d);
                };
            }
        }
    }
    answer
}

fn parse_map(data: &str) -> Map {
    let grid = parse_number_grid(&data);
    let width = grid.keys().map(|c| c.x).max().unwrap() + 1;
    let height = grid.keys().map(|c| c.y).max().unwrap() + 1;
    let mut antennae: HashMap<char, HashSet<Coordinate<i32>>> = HashMap::new();
    for (coord, letter) in grid.iter() {
        if *letter != '.' {
            antennae.entry(*letter).or_default().insert(*coord);
        }
    }

    Map {
        antennae,
        width,
        height,
    }
}

#[time_function]
fn part1(data: &str) -> usize {
    let map = parse_map(&data);
    let mut all_antinodes: HashSet<Coordinate<i32>> = HashSet::new();
    for (_freq, antennae) in map.antennae.iter() {
        all_antinodes.extend(antinodes(&map, antennae));
    }
    all_antinodes.len()
}

fn slopenodes(map: &Map, antennae: &HashSet<Coordinate<i32>>) -> HashSet<Coordinate<i32>> {
    let mut answer = HashSet::new();
    for a in antennae.iter() {
        for b in antennae.iter() {
            if a != b {
                // find all points on the line through a,b but within bounds
                let slope = *a - *b;
                let mut candidate = *a;
                while map.contains(candidate) {
                    candidate = candidate - slope;
                }
                loop {
                    candidate = candidate + slope;
                    if map.contains(candidate) {
                        answer.insert(candidate);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    answer
}

#[time_function]
fn part2(data: &str) -> usize {
    let map = parse_map(&data);
    let mut all_antinodes: HashSet<Coordinate<i32>> = HashSet::new();
    for (_freq, antennae) in map.antennae.iter() {
        all_antinodes.extend(slopenodes(&map, antennae));
    }
    all_antinodes.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(8, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    use super::*;

    #[test]
    fn test_a2() {
        let a = Coordinate { x: 4, y: 3 };
        let b = Coordinate { x: 5, y: 5 };
        let (c, d) = antinodes2(&a, &b);
        assert_eq!(d, Coordinate { x: 3, y: 1 });
        assert_eq!(c, Coordinate { x: 6, y: 7 });
    }

    #[test]
    fn test_parse() {
        let map = parse_map(TESTDATA);
        assert_eq!(map.antennae.len(), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 34);
    }
}
