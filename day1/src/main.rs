use aochelpers::get_daily_input;
use std::iter::zip;
use std::{collections::HashMap, error::Error};

fn split_whitespace_once(s: &str) -> Result<(i64, i64), String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() == 2 {
        return Ok((
            words[0].parse().expect("int"),
            words[1].parse().expect("int"),
        ));
    } else {
        return Err(format!("couldn't parse {s:?}"));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(1, 2024)?;
    let (mut xs, mut ys): (Vec<i64>, Vec<i64>) = data
        .lines()
        .map(|loc| {
            return split_whitespace_once(loc).expect("tuple");
        })
        .unzip();
    xs.sort();
    ys.sort();
    let diffs = zip(&xs, &ys).map(|(x, y)| return (x - y).abs());
    let total: i64 = diffs.sum();
    println!("part 1: {}", total);

    // let similarity: i64 = xs
    //     .iter()
    //     .map(|&x| x * ys.iter().filter(|&&y| y == x).count() as i64)
    //     .sum();
    let mut y_counts: HashMap<&i64, i64> = HashMap::new();
    ys.iter().for_each(|y| *y_counts.entry(y).or_default() += 1);
    let similarity: i64 = xs.iter().map(|x| x * y_counts.get(x).unwrap_or(&0)).sum();
    println!("part 2: {}", similarity);

    Ok(())
}
