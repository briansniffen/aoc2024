use aochelpers::get_daily_input;
use code_timing_macros::time_function;
// use rayon::prelude::*;
use std::{cmp::Ordering::*, collections::HashSet, error::Error};

fn middle_element(v: &Vec<u32>) -> u32 {
    v[v.len() / 2]
}

fn is_sorted(book: &Vec<u32>, constraints: &HashSet<(u32, u32)>) -> bool {
    for (a, b) in constraints {
        let early = book.iter().position(|&x| x == *a);
        let late = book.iter().position(|&x| x == *b);
        match (early, late) {
            (Some(early), Some(late)) => {
                if early > late {
                    return false;
                }
            }
            _ => { //pass
            }
        }
    }
    true
}

#[time_function]
fn part1(data: &str) -> u32 {
    let (constraints, books) = parse_input(data);
    let mut total = 0;
    for book in books {
        if is_sorted(&book, &constraints) {
            total += middle_element(&book);
        }
    }
    total
}

fn by_constraints(
    constraints: &HashSet<(u32, u32)>,
) -> impl FnMut(&u32, &u32) -> std::cmp::Ordering + '_ {
    move |a, b| {
        if a == b {
            Equal
        } else if constraints.contains(&(*a, *b)) {
            Less
        } else {
            Greater
        }
    }
}

#[time_function]
fn part2(data: &str) -> u32 {
    let (constraints, books) = parse_input(data);
    let mut total = 0;
    for mut book in books {
        if !is_sorted(&book, &constraints) {
            book.sort_by(by_constraints(&constraints));
            total += middle_element(&book);
        }
    }
    total
}

fn parse_input(data: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut constraint_map = HashSet::new();
    let (constraints, books) = data.split_once("\n\n").unwrap();
    for line in constraints.lines() {
        let (early, late) = line.split_once("|").unwrap();
        let early = early.parse::<u32>().unwrap();
        let late = late.parse::<u32>().unwrap();
        constraint_map.insert((early, late));
    }
    let books = books
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (constraint_map, books)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(5, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    use super::*;

    #[test]
    fn test_parse() {
        let (constraints, books) = parse_input(TESTDATA);
        assert_eq!(constraints.len(), 21);
        assert_eq!(books.len(), 6);
        assert_eq!(books[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(books[5], vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn test_middle_element() {
        assert_eq!(middle_element(&vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTDATA), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTDATA), 123);
    }
}
