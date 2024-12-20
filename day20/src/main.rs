use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use pathfinding::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

fn path(
    grid: &HashMap<Coordinate<i32>, char>,
    start: Coordinate<i32>,
    end: Coordinate<i32>,
) -> Option<Vec<Coordinate<i32>>> {
    bfs(
        &start,
        |&p| {
            p.neighbours()
                .into_iter()
                .filter(|c| grid.get(c).unwrap_or(&'#') != &'#')
                .collect::<Vec<Coordinate<i32>>>()
        },
        |&p| p == end,
    )
}

fn find_cheats(
    path: &Vec<Coordinate<i32>>,
    thresh: usize,
    cheat: usize,
) -> Vec<(Coordinate<i32>, Coordinate<i32>)> {
    //    for i in 0..path.len() - thresh {
    (0..path.len() - thresh)
        .into_par_iter()
        .flat_map(|i| {
            let mut cheats: Vec<(Coordinate<i32>, Coordinate<i32>)> = Vec::new();
            for j in i + thresh..path.len() {
                let md = path[i].manhattan_distance(&path[j]) as usize;
                // You're not teleporting, you have to cover the distance,
                // so manhattan has to be <= cheat *and* i to j has to be more than thresh by enough to cover the distance
                if md <= cheat && j - i >= thresh + md {
                    cheats.push((path[i], path[j]));
                }
            }
            cheats
        })
        .collect()
}

#[time_function]
fn part1(data: &str, thresh: usize) -> usize {
    let grid: HashMap<Coordinate<i32>, char> = parse_number_grid(&data);
    let start = *grid.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = *grid.iter().find(|(_, v)| **v == 'E').unwrap().0;

    let orthodox = path(&grid, start, end).expect("no path found!");

    find_cheats(&orthodox, thresh, 2).len()
}

#[time_function]
fn part2(data: &str, thresh: usize) -> usize {
    let grid: HashMap<Coordinate<i32>, char> = parse_number_grid(&data);
    let start = *grid.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = *grid.iter().find(|(_, v)| **v == 'E').unwrap().0;

    let orthodox = path(&grid, start, end).expect("no path found!");

    find_cheats(&orthodox, thresh, 20).len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(20, 2024)?;
    println!("part1: {}", part1(&data, 100));
    println!("part2: {}", part2(&data, 100));

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
        assert_eq!(part1(&TESTDATA, 64), 1);
        assert_eq!(part1(&TESTDATA, 40), 2);
        assert_eq!(part1(&TESTDATA, 38), 3);
        assert_eq!(part1(&TESTDATA, 36), 4);
        assert_eq!(part1(&TESTDATA, 20), 5);
        assert_eq!(part1(&TESTDATA, 12), 8);
        assert_eq!(part1(&TESTDATA, 10), 10);
        assert_eq!(part1(&TESTDATA, 8), 14);
        assert_eq!(part1(&TESTDATA, 6), 16);
        assert_eq!(part1(&TESTDATA, 4), 30);
        assert_eq!(part1(&TESTDATA, 2), 44);
    }
}
