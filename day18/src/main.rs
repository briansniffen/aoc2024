use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use pathfinding::prelude::*;
use std::error::Error;

#[derive(Debug, Clone)]
struct Map {
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
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

fn parse_input(data: &str, width: usize, height: usize, blocks: usize) -> Map {
    let coords = parse_coords(&data);
    Map::from_coords(&coords[0..blocks], width, height)
}

#[time_function]
fn part1(data: &str, width: usize, height: usize, blocks: usize) -> usize {
    let map = parse_input(data, width, height, blocks);
    dijkstra(
        &map.start,
        |p| map.grid.neighbours(*p).into_iter().map(|n| (n, 1)),
        |p| *p == map.end,
    )
    .expect("no path!")
    .1
}

#[time_function]
fn part2(data: &str, width: usize, height: usize) -> (usize, usize) {
    let coords = parse_coords(&data);
    let indices = Vec::from_iter(0..coords.len() - 1);
    let i = indices.partition_point(|&i| {
        let map = Map::from_coords(&coords[0..=i].to_vec(), width, height);
        dijkstra(
            &map.start,
            |p| map.grid.neighbours(*p).into_iter().map(|n| (n, 1)),
            |p| *p == map.end,
        )
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
