use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

fn parse_data(data: &str) -> (HashMap<Coordinate<i32>, char>, Vec<Coordinate<i32>>) {
    let (map_data, prog_data) = data.split_once("\n\n").unwrap();
    let map = parse_number_grid(map_data);
    let prog = prog_data
        .chars()
        .map(|char| match char {
            '<' => Coordinate { x: -1, y: 0 },
            '>' => Coordinate { x: 1, y: 0 },
            '^' => Coordinate { x: 0, y: -1 },
            'v' => Coordinate { x: 0, y: 1 },
            _ => Coordinate { x: 0, y: 0 },
        })
        .collect();
    (map, prog)
}

fn score(grid: HashMap<Coordinate<i32>, char>) -> u32 {
    grid.iter()
        .filter(|(_, char)| **char == 'O')
        .map(|(coord, _)| coord.x + 100 * coord.y)
        .sum::<i32>() as u32
}

fn can_move(
    grid: &HashMap<Coordinate<i32>, char>,
    guy: &Coordinate<i32>,
    instruction: &Coordinate<i32>,
) -> bool {
    if instruction == &(Coordinate { x: 0, y: 0 }) {
        return false;
    }
    let mut finger = guy.clone();
    loop {
        finger = finger + *instruction;
        if grid.get(&finger) == Some(&'#') {
            return false;
        } else if grid.get(&finger) == Some(&'.') {
            return true;
        }
    }
}

fn move_guy(
    grid: &mut HashMap<Coordinate<i32>, char>,
    guy: &mut Coordinate<i32>,
    instruction: &Coordinate<i32>,
) {
    if instruction == &(Coordinate { x: 0, y: 0 }) {
        return;
    }
    let mut finger = guy.clone();
    loop {
        finger = finger + *instruction;
        if grid.get(&finger) == Some(&'#') {
            panic!("tried to move into a wall at {}", finger);
        } else if grid.get(&finger) == Some(&'.') {
            break;
        } else if grid.get(&finger) == None {
            panic!("walked off grid!");
        }
    }
    loop {
        grid.insert(finger, grid[&(finger - *instruction)]);
        if grid[&finger] == '@' {
            grid.insert(finger - *instruction, '.');
            *guy = finger;
            return;
        } else {
            finger = finger - *instruction;
        }
    }
}

fn print_grid(map: &HashMap<Coordinate<i32>, char>) {
    let width = map.keys().map(|coord| coord.x).max().unwrap() + 1;
    let height = map.keys().map(|coord| coord.y).max().unwrap() + 1;
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for (coord, object) in map.iter() {
        grid[coord.y as usize][coord.x as usize] = *object;
    }
    print!("\x1B[H"); // home cursor
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

#[time_function]
fn part1(data: &str) -> u32 {
    let (mut map, prog) = parse_data(data);
    let start = map.iter().find(|(_, char)| **char == '@').unwrap().0;
    let mut guy = start.clone();
    for instruction in prog {
        if can_move(&map, &guy, &instruction) {
            print_grid(&map);
            move_guy(&mut map, &mut guy, &instruction);
        }
    }
    score(map)
}

#[time_function]
fn part2(_data: &str) -> u32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(15, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const BIGTEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 2028);
        assert_eq!(part1(&BIGTEST), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
