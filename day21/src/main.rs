use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use std::collections::HashMap;
use std::error::Error;

const NUMERIC: &str = "789
		       456
		       123
	               X0A";

const DIRECTIONAL: &str = "X^A
                           <v>";

#[derive(Debug, Clone)]
struct Pad {
    grid: HashMap<char, (i32, i32)>,
    cache: HashMap<(String, usize), usize>,
}

impl Pad {
    fn parse_pad(data: &str) -> Self {
        let mut grid = HashMap::new();
        for (j, line) in data.lines().enumerate() {
            for (i, char) in line.trim().chars().enumerate() {
                grid.insert(char, (i as i32, j as i32));
            }
        }
        let cache = HashMap::new();
        Pad { grid, cache }
    }

    fn paths(&self, from: &char, to: &char) -> Vec<String> {
        let mut answer = Vec::new();
        let from_pos = self.grid[&from];
        let to_pos = self.grid[&to];
        let y_move = match to_pos.1 - from_pos.1 {
            -3 => "^^^",
            -2 => "^^",
            -1 => "^",
            0 => "",
            1 => "v",
            2 => "vv",
            3 => "vvv",
            _ => panic!("bad vertical movement!"),
        };
        let x_move = match to_pos.0 - from_pos.0 {
            -2 => "<<",
            -1 => "<",
            0 => "",
            1 => ">",
            2 => ">>",
            _ => panic!("bad horizontal movement!"),
        };
        if self.grid[&'X'] != (from_pos.0, to_pos.1) {
            // start with A to make windows() work nicely in cost()
            answer.push(format!("A{y_move}{x_move}A"));
        }
        if self.grid[&'X'] != (to_pos.0, from_pos.1) {
            answer.push(format!("A{x_move}{y_move}A"));
        }
        answer.dedup();
        answer
    }

    fn cost(&mut self, path: &str, depth: usize) -> usize {
        if depth == 0 {
            return path.len() - 1; //leading A isn't a movement
        }
        if let Some(val) = self.cache.get(&(path.to_string(), depth)) {
            return *val;
        }
        let mut cost = 0;
        // for _i in 0..depth {
        //     print!("  ");
        // }
        // println!("path: {path}");
        for window in path.as_bytes().windows(2) {
            cost += self
                .paths(&(window[0] as char), &(window[1] as char))
                .into_iter()
                .map(|path| self.cost(&path, depth - 1))
                .min()
                .expect("no minimum size for {path}!")
        }
        self.cache.insert((path.to_string(), depth), cost);
        cost
    }

    fn topcost(&self, path: &str, depth: usize, next: &mut Pad) -> usize {
        if depth == 0 {
            return path.len() - 1; //leading A isn't a movement
        }
        let mut cost = 0;
        let path = format!("A{path}");
        //        println!("top path: {path}");
        for window in path.as_bytes().windows(2) {
            cost += self
                .paths(&(window[0] as char), &(window[1] as char))
                .into_iter()
                .map(|path| next.cost(&path, depth - 1))
                .min()
                .expect("no minimum size for {path}!")
        }
        cost
    }
}

#[time_function]
fn part1(data: &str) -> usize {
    let numeric = Pad::parse_pad(NUMERIC);
    let mut directional = Pad::parse_pad(DIRECTIONAL);
    data.lines()
        .map(|line| {
            line[..line.len() - 1]
                .parse::<usize>()
                .expect("can't parse number from {line}")
                * numeric.topcost(line, 3, &mut directional)
        })
        .sum()
}

// 154095786204678 is too low!

#[time_function]
fn part2(data: &str) -> usize {
    let numeric = Pad::parse_pad(NUMERIC);
    let mut directional = Pad::parse_pad(DIRECTIONAL);
    data.lines()
        .map(|line| {
            line[..line.len() - 1]
                .parse::<usize>()
                .expect("can't parse number from {line}")
                * numeric.topcost(line, 26, &mut directional)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(21, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "029A
980A
179A
456A
379A";
    use super::*;

    #[test]
    fn test_parse_pad() {
        let pad = Pad::parse_pad(NUMERIC);
        assert_eq!(pad.grid[&'5'], (1, 1));
    }

    #[test]
    fn test_numeric() {
        let mut pad = Pad::parse_pad(NUMERIC);
        assert_eq!(pad.cost(&"A0", 0), 1);
        assert_eq!(pad.cost(&"A5", 0), 1);
    }

    #[test]
    fn test_depth1() {
        let pad = Pad::parse_pad(NUMERIC);
        let mut dir = Pad::parse_pad(DIRECTIONAL);
        assert_eq!(pad.topcost(&"0", 1, &mut dir), 2);
        assert_eq!(pad.topcost(&"02", 1, &mut dir), 4);
        assert_eq!(pad.topcost(&"029", 1, &mut dir), 8);
        assert_eq!(pad.topcost(&"029A", 1, &mut dir), 12);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 126384);
    }
}
