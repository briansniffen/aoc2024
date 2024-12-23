use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
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

fn bronkerbosch<'a, T, FT>(
    r: &HashSet<T>,
    p: &HashSet<T>,
    x: &HashSet<T>,
    neighbors: &'a FT,
) -> Vec<HashSet<T>>
where
    T: Hash + Clone + Eq,
    FT: Fn(&T) -> HashSet<T> + 'a,
{
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }
    let mut results = Vec::new();

    // Choose pivot u from P ∪ X with the most neighbors in P
    let pivot = p
        .iter()
        .chain(x.iter())
        .max_by_key(|&v| neighbors(v).intersection(p).count())
        .cloned();

    let p_minus_n_u = if let Some(u) = pivot {
        p.difference(&neighbors(&u)).cloned().collect::<Vec<_>>()
    } else {
        p.iter().cloned().collect::<Vec<_>>()
    };

    for v in p_minus_n_u {
        let mut r_new = r.clone();
        r_new.insert(v.clone());
        let ns = neighbors(&v);
        let p_new: HashSet<_> = p.intersection(&ns).cloned().collect();
        let x_new: HashSet<_> = x.intersection(&ns).cloned().collect();
        let mut intermediate = bronkerbosch(&r_new, &p_new, &x_new, neighbors);
        results.append(&mut intermediate);
        let mut p = p.clone();
        p.remove(&v);
        let mut x = x.clone();
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
