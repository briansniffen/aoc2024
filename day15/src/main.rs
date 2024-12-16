use aochelpers::{get_daily_input, parse_number_grid, Coordinate};
use code_timing_macros::time_function;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io;
use std::io::prelude::*;

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
        .filter(|(_, char)| **char == 'O' || **char == '[')
        .map(|(coord, _)| coord.x + 100 * coord.y)
        .sum::<i32>() as u32
}

fn move_guy(grid: &mut HashMap<Coordinate<i32>, char>, instruction: &Coordinate<i32>) {
    if instruction == &(Coordinate { x: 0, y: 0 }) {
        return;
    }
    let guy = grid.iter().find(|(_, char)| **char == '@').unwrap().0;
    let mut fingers: Vec<Coordinate<i32>> = vec![guy.clone()];
    let mut to_move: Vec<Coordinate<i32>> = vec![];
    'scan: loop {
        let mut new_fingers = HashSet::new();
        let mut drop_columns: Vec<Coordinate<i32>> = vec![];
        for finger in &mut fingers {
            to_move.push(*finger);
            *finger = *finger + *instruction;
            match grid.get(&finger) {
                Some(&'#') => {
                    // can't move, drop the whole plan
                    return;
                }
                Some(&'.') => {
                    // no more movement in this column
                    if instruction.y != 0 {
                        // vertical movement in part 2
                        drop_columns.push(*finger);
                    } else {
                        // part 1, and all horizontal movement
                        break 'scan;
                    }
                }
                None => {
                    panic!("walked off the grid!");
                }
                Some(&'[') => {
                    // if we're going vertical, add column to the east
                    if instruction.y != 0 {
                        let mut new_finger = finger.clone();
                        new_finger.x += 1;
                        new_fingers.insert(new_finger);
                    }
                }
                Some(&']') => {
                    // if we're going vertical, add column to the west
                    if instruction.y != 0 {
                        let mut new_finger = finger.clone();
                        new_finger.x -= 1;
                        new_fingers.insert(new_finger);
                    }
                }
                Some(_) => {}
            }
        }
        for finger in new_fingers.into_iter() {
            if !fingers.contains(&finger) {
                fingers.push(finger)
            }
        }
        fingers.retain(|finger| !drop_columns.contains(&finger));
        if fingers.len() == 0 {
            break 'scan;
        }
    }
    while let Some(source) = to_move.pop() {
        let dest = source + *instruction;
        grid.insert(dest, grid[&source]);
        if !to_move.contains(&(source - *instruction)) {
            grid.insert(source, '.');
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
    for instruction in prog {
        print_grid(&map);
        move_guy(&mut map, &instruction);
    }
    score(map)
}

fn double_map(map: &HashMap<Coordinate<i32>, char>) -> HashMap<Coordinate<i32>, char> {
    let mut out = HashMap::new();
    for (coord, object) in map.iter() {
        let new_coord = Coordinate {
            x: coord.x * 2,
            y: coord.y,
        };
        let new_coord2 = Coordinate {
            x: coord.x * 2 + 1,
            y: coord.y,
        };
        match object {
            '.' => {
                out.insert(new_coord, '.');
                out.insert(new_coord2, '.');
            }
            '#' => {
                out.insert(new_coord, '#');
                out.insert(new_coord2, '#');
            }
            'O' => {
                out.insert(new_coord, '[');
                out.insert(new_coord2, ']');
            }
            '@' => {
                out.insert(new_coord, '@');
                out.insert(new_coord2, '.');
            }
            _ => panic!("unknown object {}", object),
        };
    }
    out
}

fn broken(map: &HashMap<Coordinate<i32>, char>) -> bool {
    let width = map.keys().map(|coord| coord.x).max().unwrap() + 1;
    for (coord, c) in map {
        if c == &'[' {
            let coord2 = Coordinate {
                x: coord.x + 1,
                y: coord.y,
            };
            if map[&coord2] != ']' {
                return true;
            }
        }
    }
    return false;
}

#[time_function]
fn part2(data: &str) -> u32 {
    let (small_map, prog) = parse_data(data);
    let mut map = double_map(&small_map);
    for instruction in prog {
        //        print_grid(&map);
        move_guy(&mut map, &instruction);
    }
    score(map)
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
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
        assert_eq!(part2(&BIGTEST), 9021);
    }
}
