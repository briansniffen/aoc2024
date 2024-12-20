use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use pathfinding::prelude::*;
//use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

type Map = HashMap<Coordinate<i32>, char>;

#[time_function]
fn part1(data: &str, thresh: usize) -> usize {
    let grid: Map = parse_number_grid(&data);
    let start: Coordinate<i32> = *grid.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end: Coordinate<i32> = *grid.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let orthodox: Vec<Coordinate<i32>> = bfs(
        &start,
        |&p| {
            p.neighbours()
                .into_iter()
                .filter(|c| grid.get(c).unwrap_or(&'#') != &'#')
                .collect::<Vec<Coordinate<i32>>>()
        },
        |&p| p == end,
    )
    .unwrap();
    let mut answers = Vec::new();
    for (&c, &v) in grid.iter() {
        if v != '#' {
            continue;
        } else {
            let mut grid = grid.clone();
            grid.entry(c).and_modify(|v| *v = '.');
            let candidate: Vec<Coordinate<i32>> = bfs(
                &start,
                |&p| {
                    p.neighbours()
                        .into_iter()
                        .filter(|c| grid.get(c).unwrap_or(&'#') != &'#')
                        .collect::<Vec<Coordinate<i32>>>()
                },
                |&p| p == end,
            )
            .unwrap();
            if candidate.len() + thresh < orthodox.len() {
                answers.push(c);
            }
        }
    }
    answers.len()
}

#[time_function]
fn part2(_data: &str) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(20, 2024)?;
    println!("part1: {}", part1(&data, 100));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA, 2), 44);
        assert_eq!(part1(&TESTDATA, 4), 30);
        assert_eq!(part1(&TESTDATA, 6), 16);
        assert_eq!(part1(&TESTDATA, 8), 14);
        assert_eq!(part1(&TESTDATA, 10), 10);
        assert_eq!(part1(&TESTDATA, 12), 8);
        assert_eq!(part1(&TESTDATA, 20), 1);
        assert_eq!(part1(&TESTDATA, 36), 1);
        assert_eq!(part1(&TESTDATA, 38), 1);
        assert_eq!(part1(&TESTDATA, 40), 1);
        assert_eq!(part1(&TESTDATA, 64), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
