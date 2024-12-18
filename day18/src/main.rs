use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use pathfinding::prelude::*;
use std::error::Error;

#[derive(Debug, Clone)]
struct Map {
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        let mut inv = self.grid.clone();
        inv.invert();
        println!("{:?}", inv);
        println!("Start: {:?}, End: {:?}", self.start, self.end);
    }
    fn from_coords(coords: &[(usize, usize)], width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height);
        for &coord in coords {
            grid.add_vertex(coord);
        }
        grid.invert();
        let start = (0, 0);
        let end = (width - 1, height - 1);
        Map { grid, start, end }
    }
    fn from_string(data: &str, width: usize, height: usize, blocks: usize) -> Self {
        let coords = parse_coords(&data);
        Map::from_coords(&coords[0..blocks], width, height)
    }

    fn path_cost(&self) -> Option<usize> {
        astar(
            &self.start,
            |p| self.grid.neighbours(*p).into_iter().map(|n| (n, 1)),
            |p| self.grid.distance(*p, self.end),
            |p| *p == self.end,
        )
        .and_then(|x| Some(x.1))
    }
}

fn parse_coords(data: &str) -> Vec<(usize, usize)> {
    data.lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect(&format!("parse error!: {}", line));
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect()
}

#[time_function]
fn part1(data: &str, width: usize, height: usize, blocks: usize) -> usize {
    Map::from_string(data, width, height, blocks)
        .path_cost()
        .unwrap()
}

#[time_function]
fn part2(data: &str, width: usize, height: usize) -> (usize, usize) {
    let coords = parse_coords(&data);
    let indices = Vec::from_iter(0..coords.len() - 1);
    let i = indices.partition_point(|&i| {
        Map::from_coords(&coords[0..=i].to_vec(), width, height)
            .path_cost()
            .is_some()
    });
    coords[i]
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(18, 2024)?;
    println!("part1: {}", part1(&data, 71, 71, 1024));
    let part2 = part2(&data, 71, 71);
    println!("part2: {},{}", part2.0, part2.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA, 7, 7, 12), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA, 7, 7), (6, 1));
    }
}
