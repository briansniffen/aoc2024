use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
//use slice_deque::SliceDeque;
//use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum File {
    Data { file_id: u64 },
    Empty,
}

enum File2 {
    Data { file_id: u64, length: u64 },
    Empty { length: u64 },
}

impl File2 {
    fn length(&self) -> u64 {
        match self {
            File2::Data { length, .. } => *length,
            File2::Empty { length } => *length,
        }
    }

    fn emptyp(&self) -> bool {
        match self {
            File2::Empty { .. } => true,
            _ => false,
        }
    }
}

type Disk = Vec<File>;

type Disk2 = Vec<File2>;

fn length(s: &Disk2) -> usize {
    s.iter().map(|file| file.length()).sum::<u64>() as usize
}

fn parse_disk_map(data: &str) -> Disk {
    let mut disk_map = Disk::new();
    let mut file_number = 0;
    let mut empty_next = false;
    for byte in data.chars() {
        let length = byte.to_digit(10).unwrap();
        if empty_next {
            for _i in 0..length {
                disk_map.push(File::Empty);
            }
            empty_next = false;
        } else {
            for _i in 0..length {
                disk_map.push(File::Data {
                    file_id: file_number,
                });
            }
            empty_next = true;
            file_number += 1;
        }
    }
    disk_map
}

fn parse_disk_map2(data: &str) -> Disk2 {
    let mut disk_map = Disk2::new();
    let mut file_id = 0;
    let mut empty_next = false;
    for byte in data.chars() {
        let length: u64 = byte.to_digit(10).unwrap().into();
        if empty_next {
            disk_map.push(File2::Empty { length });
            empty_next = false;
        } else {
            disk_map.push(File2::Data { file_id, length });
            empty_next = true;
            file_id += 1;
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

fn emptiness2(disk: &Disk2) -> usize {
    disk.iter()
        .map(|file| match file {
            File2::Empty { length } => *length,
            _ => 0,
        })
        .sum::<u64>() as usize
}

fn pack(disk: &mut Disk) {
    // move files from the end of the disk to fill empty space near the front.
    let mut a = 0;
    let mut b = disk.len() - 1;
    let limit = b - emptiness(disk);
    while a < b {
        assert!(a <= limit);
        match disk[a] {
            File::Data { file_id: _ } => {
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

// on disk, find index i, assume that's empty, and replace that with *two* empties
// such that the first is length `length` and the other is whatever's left
fn split_empty_space(disk: &mut Disk2, i: usize, length: u64) {
    let e = disk.get(i).unwrap().length();
    assert!(e >= length);
    if e == length {
        return;
    }
    disk.remove(i);
    disk.insert(i, File2::Empty { length });
    disk.insert(i + 1, File2::Empty { length: e - length });
}

fn congeal(disk: &mut Disk2) {
    // splice together all adjacent empty spaces
    for i in 0..disk.len() - 1 {
        let e = disk.get(i).unwrap();
        let f = disk.get(i + 1).unwrap();
        if e.emptyp() && f.emptyp() {
            *disk = disk
                .splice(
                    i..i + 1,
                    [File2::Empty {
                        length: e.length() + f.length(),
                    }],
                )
                .collect();
        }
    }
}

fn pack_last_file(disk: &mut Disk2) {
    // move the last file to the front of the disk
    let mut a = 0;
    let mut b = disk.len() - 1;
    // move b cursor back to a file
    while disk[b].emptyp() {
        b -= 1;
    }
    // move a cursor forward to an empty
    while a < b && !disk[a].emptyp() {
        a += 1;
        if disk[a].length() <= disk[b].length() {
            continue;
        } else {
            split_empty_space(disk, a, disk[b].length());
            disk.swap(a, b);
            break;
        }
    }
}

fn pack2(disk: &mut Disk2) {
    println!("packing disk len {}", disk.len());
    // move files from the end of the disk to fill empty space near the front.
    let mut b = disk.len() - 1;
    // walk the disk from the back.  for each file, walk the disk from the front to the file's existing location looking for an empty space.
    // Finding that space, either swap them directly OR split the empty space and swap.
    while b > 0 {
        let mut a = 0;
        while disk[b].emptyp() {
            b -= 1;
        }
        while a < b && (!disk[a].emptyp() || disk[a].length() < disk[b].length()) {
            a += 1;
        }
        if a >= b {
            b -= 1;
            continue;
        }
        if disk[a].length() != disk[b].length() {
            split_empty_space(disk, a, disk[b].length());
            b = b + 1; //we split extra empty space early on, so the later part moved!
        }
        disk.swap(a, b);
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

fn checksum2(disk: &Disk2) -> u64 {
    let mut checksum = 0;
    let mut index = 0;
    for file in disk {
        match file {
            File2::Data { file_id, length } => {
                for _i in 0..*length {
                    checksum += *file_id * index;
                    index += 1;
                }
            }
            File2::Empty { length } => {
                index += length;
            }
        }
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
fn part2(data: &str) -> u64 {
    let mut disk = parse_disk_map2(data);
    pack2(&mut disk);
    checksum2(&disk)
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
        assert_eq!(disk_map.len(), 42);
    }

    #[test]
    fn test_parse2() {
        let disk_map = parse_disk_map2(TESTDATA);
        assert_eq!(disk_map.len(), 19);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 1928);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(&parse_disk_map("233")), 18);
    }

    #[test]
    fn test_checksum2() {
        assert_eq!(checksum2(&parse_disk_map2("233")), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), 2858);
    }
}
