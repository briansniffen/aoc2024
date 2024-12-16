use aochelpers::{get_daily_input, parse_number_grid, Coordinate, Direction, Direction::*};
use code_timing_macros::time_function;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::u32::MAX;

type Coord = Coordinate<u32>;
type Map = HashMap<Coord, char>;

#[derive(Eq, PartialEq, Clone)]
struct Step {
    cost: u32,
    coord: Coord,
    direction: Direction,
    path: Vec<Coord>,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn parse_map(data: &str) -> Map {
    parse_number_grid(data)
}

const CARDINAL: [Direction; 4] = [North, East, South, West];

fn find_path(grid: Map) -> (u32, u32) {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut good_seats: HashSet<Coordinate<u32>> = HashSet::new();
    let mut max_cost = MAX;
    let start: Coord = *grid.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let end: Coord = *grid.iter().find(|(_, c)| **c == 'E').unwrap().0;
    let start_direction = East;
    queue.push(Step {
        cost: 0,
        coord: start,
        direction: start_direction,
        path: vec![start],
    });
    while let Some(Step {
        cost,
        coord,
        direction,
        path,
    }) = queue.pop()
    {
        if cost > max_cost {
            continue;
        }
        if let Some(&visited_cost) = visited.get(&(coord, direction)) {
            if visited_cost < cost {
                continue;
            }
        }
        if coord == end {
            max_cost = cost;
            good_seats.extend(path.iter());
        }
        visited.insert((coord, direction), cost);
        for next_direction in CARDINAL {
            let next_coord = coord.neighbour(next_direction);
            if let Some(next_char) = grid.get(&next_coord) {
                if next_char == &'#' {
                    continue;
                }
                let next_cost;
                if direction == next_direction {
                    next_cost = cost + 1;
                } else {
                    next_cost = cost + 1001;
                }
                queue.push(Step {
                    cost: next_cost,
                    coord: next_coord,
                    direction: next_direction,
                    path: {
                        let mut new_path: Vec<Coordinate<u32>> = path.clone();
                        new_path.push(next_coord);
                        new_path
                    },
                });
            }
        }
    }
    (max_cost, good_seats.len() as u32)
}

#[time_function]
fn part1(data: &str) -> u32 {
    let grid = parse_map(&data);
    find_path(grid).0
}

#[time_function]
fn part2(data: &str) -> u32 {
    let grid = parse_map(&data);
    find_path(grid).1
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(16, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const BIGTEST: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 7036);
        assert_eq!(part1(&BIGTEST), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 45);
        assert_eq!(part2(&BIGTEST), 64);
    }
}
