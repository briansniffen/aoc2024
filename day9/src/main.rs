use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum File {
    Data { file_id: u32 },
    Empty,
}

type Disk = VecDeque<File>;

fn parse_disk_map(data: &str) -> Disk {
    let mut disk_map = Disk::new();
    let mut file_number = 0;
    let mut empty_next = false;
    for byte in data.chars() {
        let length = byte.to_digit(10).unwrap();
        if empty_next {
            for _i in 0..length {
                disk_map.push_back(File::Empty);
            }
            empty_next = false;
        } else {
            for _i in 0..length {
                disk_map.push_back(File::Data {
                    file_id: file_number,
                });
            }
            empty_next = true;
            file_number += 1;
        }
    }
    disk_map
}

fn emptiness(disk: &Disk) -> usize {
    disk.iter()
        .filter(|file| match file {
            File::Empty => true,
            _ => false,
        })
        .count()
}

fn pack(disk: &mut Disk) {
    // move files from the end of the disk to fill empty space near the front.
    let mut a = 0;
    let mut b = disk.len() - 1; // FIXME - 1?
    let limit = b - emptiness(disk);
    while a < b {
        assert!(a <= limit);
        match disk[a] {
            File::Data { file_id } => {
                a += 1;
            }
            File::Empty => {
                disk.swap(a, b);
                a += 1;
                while disk[b] == File::Empty {
                    b -= 1;
                }
            }
        }
    }
}

fn checksum(disk: &Disk) -> u64 {
    let mut checksum = 0;
    let mut index = 0;
    for file in disk {
        match file {
            File::Data { file_id } => {
                checksum += *file_id as u64 * index;
            }
            _ => {}
        }
        index += 1;
    }
    checksum
}

#[time_function]
fn part1(data: &str) -> u64 {
    let mut disk = parse_disk_map(data);
    pack(&mut disk);
    checksum(&disk)
}

#[time_function]
fn part2(_data: &str) -> u32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(9, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "2333133121414131402";
    use super::*;

    #[test]
    fn test_parse() {
        let disk_map = parse_disk_map(TESTDATA);
        assert_eq!(disk_map.len(), 19);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 0);
    }
}
