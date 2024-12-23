use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::error::Error;
use std::collections::{HashMap,HashSet};
use pathfinding::prelude::connected_components;

fn any_initial_t(abc:&(String,String,String)) -> bool {
    let (a,b,c) = abc;
    a.starts_with('t') || b.starts_with('t') || c.starts_with('t')
}

#[time_function]
fn part1(data: &str) -> usize {
    let mut net: HashMap<String,HashSet<String>> = HashMap::new();
    let mut clusters: HashSet<(String,String,String)> = HashSet::new();
    for line in data.lines() {
        let (a,b) = line.split_once('-').expect("network connection");
        match (net.get(a),net.get(b)) {
            (Some(a_net),Some(b_net)) => {
                for c in a_net.intersection(b_net) {
                    let mut cluster = vec![a,b,c];
                    cluster.sort();
                    clusters.insert((cluster[0].to_string(),cluster[1].to_string(),cluster[2].to_string()));
                }
            }
            _ => {}
        }
        net.entry(a.to_string()).or_insert(HashSet::new()).insert(b.to_string());
        net.entry(b.to_string()).or_insert(HashSet::new()).insert(a.to_string());
    }
    clusters.into_iter().filter(any_initial_t).count()
}

#[time_function]
fn part2(data: &str) -> String {
    let mut net: HashMap<String,HashSet<String>> = HashMap::new();
    for line in data.lines() {
        let (a,b) = line.split_once('-').expect("network connection");
        net.entry(a.to_string()).or_insert(HashSet::new()).insert(b.to_string());
        net.entry(b.to_string()).or_insert(HashSet::new()).insert(a.to_string());
    }
    let computers: Vec<String> = net.keys().map(|x| x.to_string()).collect();
    // FIXME finds the entire connected subset rather than tightly connected
    let mut subnets: Vec<HashSet<String>> = connected_components(&computers, |a:&String| net.get(a).unwrap().clone());
    subnets.sort_by_key(|x| x.len());
    for subnet in &subnets {
        println!("{:?}", subnet);
    }
    let mut names : Vec<String> = subnets[0].clone().into_iter().collect();
    names.sort();
    names.join(",").to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(23, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TESTDATA), "co,de,ka,ta");
    }
}
