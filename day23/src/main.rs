use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
// use std::hash::Hash;
// use std::ops::BitAnd;

fn any_initial_t(abc: &(String, String, String)) -> bool {
    let (a, b, c) = abc;
    a.starts_with('t') || b.starts_with('t') || c.starts_with('t')
}

#[time_function]
fn part1(data: &str) -> usize {
    let mut net: HashMap<String, HashSet<String>> = HashMap::new();
    let mut clusters: HashSet<(String, String, String)> = HashSet::new();
    for line in data.lines() {
        let (a, b) = line.split_once('-').expect("network connection");
        match (net.get(a), net.get(b)) {
            (Some(a_net), Some(b_net)) => {
                for c in a_net.intersection(b_net) {
                    let mut cluster = vec![a, b, c];
                    cluster.sort();
                    clusters.insert((
                        cluster[0].to_string(),
                        cluster[1].to_string(),
                        cluster[2].to_string(),
                    ));
                }
            }
            _ => {}
        }
        net.entry(a.to_string())
            .or_insert(HashSet::new())
            .insert(b.to_string());
        net.entry(b.to_string())
            .or_insert(HashSet::new())
            .insert(a.to_string());
    }
    clusters.into_iter().filter(any_initial_t).count()
}

/*  implement BK:
 algorithm BronKerbosch2(R, P, X) is
if P and X are both empty then
report R as a maximal clique
choose a pivot vertex u in P ⋃ X
for each vertex v in P \ N(u) do
    BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    P := P \ {v}
    X := X ⋃ {v}
*/

fn bronkerbosch<'a, FT>(
    r: &HashSet<String>,
    p: &HashSet<String>,
    x: &HashSet<String>,
    neighbors: &'a FT,
) -> Vec<HashSet<String>>
where
    FT: Fn(&String) -> HashSet<String> + 'a,
{
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }
    let mut results = Vec::new();
    let p_iter: Vec<String> = p.iter().cloned().collect();
    let mut p = p.clone();
    let mut x = x.clone();
    // FIXME choose a pivot u , and separate p into p-n(u) and p|n(v)
    for v in p_iter {
        let mut r = r.clone();
        r.insert(v.clone());
        let ns = neighbors(&v);
        let mut intermediate = bronkerbosch(&r, &(&p & &ns), &(&x & &ns), neighbors);
        results.append(&mut intermediate);
        p.remove(&v);
        x.insert(v);
    }
    results
}

fn cliques(graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let nodes: HashSet<String> = graph.keys().cloned().collect();
    let neighbors = |s: &String| match graph.get(s) {
        Some(ns) => ns.clone(),
        None => HashSet::new(),
    };
    bronkerbosch(&HashSet::new(), &nodes, &HashSet::new(), &neighbors)
}

#[time_function]
fn part2(data: &str) -> String {
    let mut net: HashMap<String, HashSet<String>> = HashMap::new();
    for line in data.lines() {
        let (a, b) = line.split_once('-').expect("network connection");
        net.entry(a.to_string())
            .or_insert(HashSet::new())
            .insert(b.to_string());
        net.entry(b.to_string())
            .or_insert(HashSet::new())
            .insert(a.to_string());
    }
    let mut subnets = cliques(&net);
    subnets.sort_by_key(|x| x.len());
    subnets.reverse();
    let mut names: Vec<String> = subnets[0].clone().into_iter().collect();
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
