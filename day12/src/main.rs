use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;

const DIRECTIONS: [Coordinate<i32>; 4] = [
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 0, y: 1 },
    Coordinate { x: -1, y: 0 },
    Coordinate { x: 0, y: -1 },
];

#[time_function]
fn part1(data: &str) -> u32 {
    let grid = parse_number_grid::<i32, char>(&data);
    let width = grid.keys().map(|c| c.x).max().unwrap() + 1;
    let height = grid.keys().map(|c| c.y).max().unwrap() + 1;
    // start with (0,0).  flood fill the region.  At boundaries, tag them as for new regions!
    let mut total = 0;
    let mut visited = HashSet::new();
    for (coord, crop) in grid.iter() {
        if visited.contains(coord) {
            continue;
        }
        let mut stack = vec![*coord];
        let mut region = HashSet::new();
        let mut perimeter = 0;
        while let Some(coord) = stack.pop() {
            for dir in DIRECTIONS {
                if region.contains(&(coord + dir)) {
                } else if (coord + dir).x < 0
                    || (coord + dir).x >= width
                    || (coord + dir).y < 0
                    || (coord + dir).y >= height
                    || grid[&(coord + dir)] != *crop
                    || visited.contains(&(coord + dir))
                {
                    // if *crop == 'B' {
                    //     println!("Found perimeter at {:?}+{:?}", coord, dir);
                    // }
                    perimeter += 1;
                } else {
                    if !stack.contains(&(coord + dir)) {
                        stack.push(coord + dir);
                    }
                }
            }
            region.insert(coord.clone());
            visited.insert(coord.clone());
        }
        let area = region.len() as u32;
        total += area * perimeter;
        //println!("Region {} has area {}, perim {}.", *crop, area, perimeter);
    }
    total
}

#[time_function]
fn part2(data: &str) -> u32 {
    let grid = parse_number_grid::<i32, char>(&data);
    let width = grid.keys().map(|c| c.x).max().unwrap() + 1;
    let height = grid.keys().map(|c| c.y).max().unwrap() + 1;
    // start with (0,0).  flood fill the region.  At boundaries, tag them as for new regions!
    let mut total = 0;
    let mut visited = HashSet::new();
    for (coord, crop) in grid.iter() {
        if visited.contains(coord) {
            continue;
        }
        let mut stack = vec![*coord];
        let mut region = HashSet::new();
        let mut corners = 0;
        while let Some(coord) = stack.pop() {
            for dir in DIRECTIONS {
                if region.contains(&(coord + dir)) {
                    // already visited!
                } else if (coord + dir).x < 0
                    || (coord + dir).x >= width
                    || (coord + dir).y < 0
                    || (coord + dir).y >= height
                    || grid[&(coord + dir)] != *crop
                    || visited.contains(&(coord + dir))
                {
                    // neighbor isn't this region, don't explore further
                } else {
                    // neighbor IS this region, explore it
                    if !stack.contains(&(coord + dir)) {
                        stack.push(coord + dir);
                    }
                }
            }
            // look at 4 neighbors, decide if it's a corner
            for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
                if (grid.get(&(coord + DIRECTIONS[a]))) != Some(crop)
                    && grid.get(&(coord + DIRECTIONS[b])) != Some(crop)
                    || (grid.get(&(coord + DIRECTIONS[a]))) == Some(crop)
                        && grid.get(&(coord + DIRECTIONS[b])) == Some(crop)
                        && grid.get(&(coord + DIRECTIONS[a] + DIRECTIONS[b])) != Some(crop)
                {
                    corners += 1;
                }
            }
            region.insert(coord.clone());
            visited.insert(coord.clone());
        }
        let area = region.len() as u32;
        total += area * corners;
        //println!("Region {} has area {}, perim {}.", *crop, area, perimeter);
    }
    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(12, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const SMALLTEST: &str = "AAAA
BBCD
BBCC
EEEC";
    const ISLANDTEST: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TESTDATA: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(part1(SMALLTEST), 140);
    }

    #[test]
    fn test_island() {
        assert_eq!(part1(ISLANDTEST), 772);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 1206);
    }
}
