use aochelpers::{get_daily_input, parse_number_grid, Coordinate, Direction::*};
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::error::Error;

#[time_function]
fn part1(data: &str) -> u32 {
    let directions = [
        NorthWest, North, NorthEast, East, SouthEast, South, SouthWest, West,
    ];
    let mut total = 0;
    let grid = parse_number_grid::<i32, char>(data);
    // for (x,val) in grid.iter {
    total += grid
        .par_iter()
        .map(|(x, val)| {
            for dir in directions {
                if *val == 'X' {
                    let m = x.neighbour(dir);
                    if grid.get(&m) == Some(&'M') {
                        let a = m.neighbour(dir);
                        if grid.get(&a) == Some(&'A') {
                            let s = a.neighbour(dir);
                            if grid.get(&s) == Some(&'S') {
                                return 1;
                            }
                        }
                    }
                }
            }
            0
        })
        .sum::<u32>();
    total
}

#[time_function]
fn part2(data: &str) -> u32 {
    let directions = [
        Coordinate { x: -1, y: -1 },
        Coordinate { x: -1, y: 1 },
        Coordinate { x: 1, y: -1 },
        Coordinate { x: 1, y: 1 },
    ];
    let grid = parse_number_grid::<i32, char>(data);
    //    for (a, val) in grid.iter() {
    grid.par_iter()
        .map(|(&a, &val)| {
            let mut crosses = 0;
            if val == 'A' {
                for dir in directions {
                    if grid.get(&(a + dir)) == Some(&'M') && grid.get(&(a - dir)) == Some(&'S') {
                        crosses += 1;
                        if crosses == 2 {
                            return 1;
                        };
                    }
                }
            }
            0
        })
        .sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(4, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    use super::*;

    #[test]
    fn test_parse() {
        let grid = parse_number_grid::<i32, char>(&TESTDATA);
        assert_eq!(grid.len(), 100);
        assert_eq!(grid.get(&Coordinate { x: 0, y: 0 }), Some(&'M'));
        assert_eq!(grid.get(&Coordinate { x: 9, y: 9 }), Some(&'X'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 9);
    }
}
