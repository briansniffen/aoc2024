use aochelpers::{get_daily_input, parse_number_grid, Coordinate, Direction::*};
use code_timing_macros::time_function;
use rayon::prelude::*;
use std::{collections::HashMap, error::Error};

#[time_function]
fn part1(data: &str) -> u32 {
    let directions = [
        NorthWest, North, NorthEast, East, SouthEast, South, SouthWest, West,
    ];
    let grid = parse_number_grid::<i32, char>(data);
    let goal = "XMAS".chars().collect::<Vec<char>>();
    grid.par_iter()
        .map(|(x, val)| {
            let mut tot = 0;
            if *val == goal[0] {
                for dir in directions {
                    let mut pointer: Coordinate<i32> = *x;
                    for i in 1..goal.len() {
                        pointer = pointer.neighbour(dir);
                        if grid.get(&pointer) == Some(&goal[i]) {
                            if i == goal.len() - 1 {
                                tot += 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            tot
        })
        .sum::<u32>()
}

//rotate a grid expressed as a hashmap from coordinates to chars 90°
fn rotate_grid(
    grid: std::collections::HashMap<Coordinate<i32>, char>,
) -> Box<dyn Iterator<Item = HashMap<Coordinate<i32>, char>> + 'static> {
    let width = grid.keys().map(|c| c.x).max().unwrap();
    let height = grid.keys().map(|c| c.y).max().unwrap();
    Box::new(Box::new(0..4).map(move |rotation| {
        let mut rotated = HashMap::new();
        for (coord, char) in grid.iter() {
            let new_coord = match rotation {
                0 => coord.clone(),
                1 => Coordinate {
                    x: height - coord.y,
                    y: coord.x,
                },
                2 => Coordinate {
                    x: width - coord.x,
                    y: height - coord.y,
                },
                3 => Coordinate {
                    x: coord.y,
                    y: width - coord.x,
                },
                _ => panic!("Invalid rotation"),
            };
            rotated.insert(new_coord, *char);
        }
        rotated
    }))
}

#[time_function]
fn part2(data: &str) -> u32 {
    let goal = parse_number_grid::<i32, char>("M.M\n.A.\nS.S");
    let goals: Vec<HashMap<Coordinate<i32>,char>> = rotate_grid(goal).collect();
    let grid = parse_number_grid::<i32, char>(data);
    let mut tot = 0;
    for goal in goals.iter() {      
        tot += grid.par_iter()
            .map(|(&grid_coord, _grid_char)| {
                for (&goal_coord, &goal_char) in goal.iter() {
                    let target_coord = grid_coord + goal_coord;
                    if goal_char != '.' && grid.get(&(target_coord)) != Some(&goal_char) {
                        return 0;
                    }
                }
                return 1;
            }).sum::<u32>();
    }
    tot
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
