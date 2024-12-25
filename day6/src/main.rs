use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Clone, Debug)]
struct Map {
    grid: HashMap<Coordinate<i32>, char>,
    start: Coordinate<i32>,
    width: i32,
    height: i32,
}
const START_DIR: Coordinate<i32> = Coordinate { x: 0, y: -1 };

fn parse_map(data: &str) -> Map {
    let grid = parse_number_grid::<i32, char>(data);
    let start = grid
        .iter()
        .find(|(_, &c)| c == '^')
        .map(|(k, _)| *k)
        .unwrap();
    let width = grid.keys().map(|c| c.x).max().unwrap() + 1;
    let height = grid.keys().map(|c| c.y).max().unwrap() + 1;
    Map {
        grid,
        start,
        width,
        height,
    }
}

fn rotate_dir(dir: Coordinate<i32>) -> Coordinate<i32> {
    match dir {
        Coordinate { x: 0, y: -1 } => Coordinate { x: 1, y: 0 },
        Coordinate { x: 1, y: 0 } => Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: 1 } => Coordinate { x: -1, y: 0 },
        Coordinate { x: -1, y: 0 } => Coordinate { x: 0, y: -1 },
        _ => panic!("Invalid direction"),
    }
}

#[time_function]
fn part1(data: &str) -> u32 {
    let map = parse_map(&data);
    let visited = tour_without_directions(&map);
    visited.len() as u32
}

// fn strip_dirs(c: &HashSet<(Coordinate<i32>, Coordinate<i32>)>) -> HashSet<Coordinate<i32>> {
//     c.iter().map(|&(ref a, _)| a.clone()).collect()
// }

// fn print_map(
//     map: &Map,
//     visited: &HashSet<(Coordinate<i32>, Coordinate<i32>)>,
//     candidates: &HashSet<Coordinate<i32>>,
// ) {
//     let visited = strip_dirs(visited);
//     for y in 0..map.height {
//         for x in 0..map.width {
//             let coord = Coordinate { x, y };
//             let c = map.grid.get(&coord).unwrap();
//             if *c == '^' {
//                 print!("^");
//                 continue;
//             }
//             if candidates.contains(&coord) {
//                 print!("O");
//                 continue;
//             }
//             if visited.contains(&coord) {
//                 print!("X");
//                 continue;
//             }
//             print!("{}", c);
//         }
//         println!();
//     }
// }

fn on_path_to_repeat(map: &Map) -> bool {
    let mut loc = map.start;
    let mut dir = START_DIR;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(loc, dir)) {
            return true;
        }
        visited.insert((loc, dir));
        let next = loc + dir;
        match map.grid.get(&next) {
            None => return false,
            Some(c) if c == &'#' => {
                while map.grid.get(&(loc + dir)) == Some(&'#') {
                    dir = rotate_dir(dir);
                }
                loc = loc + dir;
            }
            Some(_) => {
                loc = next;
            }
        }
    }
}

fn on_path_to_repeat_except(map: &Map, except: Coordinate<i32>) -> bool {
    let mut loc = map.start;
    let mut dir = START_DIR;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(loc, dir)) {
            return true;
        }
        visited.insert((loc, dir));
        let next = loc + dir;
        match map.grid.get(&next) {
            None => return false,
            Some(c) if c == &'#' || next == except => {
                while map.grid.get(&(loc + dir)) == Some(&'#') || (loc + dir) == except {
                    dir = rotate_dir(dir);
                }
                loc = loc + dir;
            }
            Some(_) => {
                loc = next;
            }
        }
    }
}

// fn print_loop(
//     map: &Map,
//     visited: &HashSet<(Coordinate<i32>, Coordinate<i32>)>,
//     candidate: &Coordinate<i32>,
//     candidate_dir: &Coordinate<i32>,
// ) {
//     let mut loc = *candidate;
//     let mut dir = *candidate_dir;
//     let mut visited = visited.clone();
//     let mut newly_visited = HashSet::new();
//     loop {
//         if visited.contains(&(loc, dir)) {
//             break;
//         }
//         visited.insert((loc, dir));
//         newly_visited.insert(loc);
//         let next = loc + dir;
//         match map.grid.get(&next) {
//             None => break,
//             Some(c) if c == &'#' => {
//                 dir = rotate_dir(dir);
//                 loc = loc + dir;
//             }
//             Some(_) => {
//                 loc = next;
//             }
//         }
//     }
//     print_map(map, &visited, &newly_visited);
// }

fn tour_without_directions(map: &Map) -> HashSet<Coordinate<i32>> {
    let mut visited = HashSet::new();
    let mut loc = map.start;
    let mut dir = START_DIR;
    loop {
        visited.insert(loc);
        let next = loc + dir;
        match map.grid.get(&next) {
            None => break,
            Some(c) if c == &'#' => {
                dir = rotate_dir(dir);
            }
            Some(_) => {
                loc = next;
            }
        }
    }
    visited
}

fn plausible_blocks(map: &Map) -> HashSet<Coordinate<i32>> {
    let mut visited = tour_without_directions(&map);
    visited.remove(&map.start);

    visited
        .par_iter()
        .flat_map(|&loc| {
            if on_path_to_repeat_except(&map, loc) {
                Some(loc)
            } else {
                None
            }
        })
        .collect()
}

#[time_function]
fn part2(data: &str) -> u32 {
    let map = parse_map(&data);
    let candidates: HashSet<Coordinate<i32>> = plausible_blocks(&map);
    candidates.len() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(6, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    use super::*;

    #[test]
    fn test_parse() {
        let map = parse_map(&TESTDATA);
        assert_eq!(map.grid.len(), 100);
        assert_eq!(map.start, Coordinate { x: 4, y: 6 });
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.grid.get(&Coordinate { x: 0, y: 0 }), Some(&'.'));
        assert_eq!(map.grid.get(&Coordinate { x: 9, y: 9 }), Some(&'.'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 6);
    }

    #[test]
    fn test_part2_carefully() {
        let map = parse_map(&TESTDATA);
        let candidates: HashSet<Coordinate<i32>> = plausible_blocks(&map);
        assert!(candidates.contains(&Coordinate { x: 3, y: 8 }));
        assert!(candidates.contains(&Coordinate { x: 3, y: 6 }));
        assert!(candidates.contains(&Coordinate { x: 1, y: 8 }));
        assert!(candidates.contains(&Coordinate { x: 6, y: 7 }));
        assert!(candidates.contains(&Coordinate { x: 7, y: 7 }));
        assert!(candidates.contains(&Coordinate { x: 7, y: 9 }));
    }
}
