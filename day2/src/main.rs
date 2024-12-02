use aochelpers::get_daily_input;
use std::error::Error;

type Report = Vec<i64>;

fn safe(r: &Report) -> bool {
    if r.len() < 2 {
        return true;
    }
    let order = if r[0] < r[1] { 1 } else { -1 };
    for i in 1..r.len() {
        let gap = (r[i] - r[i - 1]) * order;
        if gap < 1 || gap > 3 {
            return false;
        }
    }
    true
}

fn damped_safe(r: &Report) -> bool {
    if safe(r) {
        return true;
    }
    for i in 0..r.len() {
        if safe(&[&r[..i], &r[i + 1..]].concat()) {
            return true;
        }
    }
    false
}

fn parse_data(data: &str) -> Vec<Report> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("i64"))
                .collect()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(2, 2024)?;
    let reports: Vec<Report> = parse_data(&data);

    let part1 = reports.iter().filter(|&r| safe(r)).count();
    println!("part 1: {}", part1);

    let part2 = reports.iter().filter(|&r| damped_safe(r)).count();
    println!("part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "7 6 4 2 1
			    1 2 7 8 9
			    9 7 6 2 1
			    1 3 2 4 5
			    8 6 4 4 1
			    1 3 6 7 9";
    use super::*;

    #[test]
    fn test_parser() {
        let reports: Vec<Report> = parse_data(TESTDATA);
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[3].len(), 5);
    }

    #[test]
    fn test_day1() {
        let reports: Vec<Report> = parse_data(TESTDATA);
        let part1 = reports.iter().filter(|&r| safe(r)).count();
        assert_eq!(part1, 2);
    }

    #[test]
    fn test_day2() {
        let reports: Vec<Report> = parse_data(TESTDATA);
        let part2 = reports.iter().filter(|&r| damped_safe(r)).count();
        assert_eq!(part2, 4);
    }
}
