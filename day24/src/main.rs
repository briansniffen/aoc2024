use aochelpers::get_daily_input;
use code_timing_macros::time_function;
//use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Ast {
    Val(bool),
    Gate { op: Op, a: String, b: String },
}
use Ast::*;

fn is_xor(a: &Ast) -> bool {
    matches!(a, Gate { op: Op::Xor, .. })
}
fn is_or(a: &Ast) -> bool {
    matches!(a, Gate { op: Op::Or, .. })
}
fn is_and(a: &Ast) -> bool {
    matches!(a, Gate { op: Op::And, .. })
}

fn direct_from(a: &str, g: &Ast) -> bool {
    match g {
        Val(_) => false,
        Gate {
            op: _,
            a: a1,
            b: b1,
        } => a == a1 || a == b1,
    }
}

#[derive(Debug, Clone)]
struct Env {
    env: HashMap<String, Ast>,
}

impl Env {
    fn eval(&mut self, name: &str) -> bool {
        if let Some(val) = self.env.get(name).cloned() {
            match val {
                Val(a) => a,
                Gate { op, a, b } => {
                    let val = op.apply(self.eval(&a), self.eval(&b));
                    self.env.insert(name.to_string(), Val(val));
                    val
                }
            }
        } else {
            panic!("Unknown variable {name}");
        }
    }
    fn print_descendants(&self, names: &Vec<String>) {
        let mut names: Vec<String> = names.clone();
        println!("print_descendants: {names:?}");
        while let Some(name) = names.pop() {
            println!("  {name}: {:?}", self.env.get(&name).unwrap());
            for (k, v) in self.env.iter() {
                if let Gate { op, a, b } = v {
                    if a == &name || b == &name {
                        println!("    {k}: {op:?} {a} {b}");
                        names.push(k.clone());
                    }
                }
            }
        }
    }
    fn swap(&mut self, a: &str, b: &str) {
        let old_a = self.env[a].clone();
        let old_b = self.env[b].clone();
        self.env.insert(a.to_string(), old_b);
        self.env.insert(b.to_string(), old_a);
    }

    fn find3(&self, op: Op, a: &str, b: &str) -> Option<String> {
        for (k, v) in self.env.iter() {
            if let Gate {
                op: op1,
                a: a1,
                b: b1,
            } = v
            {
                if *op1 == op && (a1 == a || a1 == b) && (b1 == a || b1 == b) {
                    return Some(k.clone());
                }
            }
        }
        None
    }
    fn find2(&self, op: Op, a: &str) -> Option<String> {
        for (k, v) in self.env.iter() {
            if let Gate {
                op: op1,
                a: a1,
                b: b1,
            } = v
            {
                if *op1 == op && ((a1 == a) || (b1 == a)) {
                    return Some(k.clone());
                }
            }
        }
        None
    }
}

fn parse_input(data: &str) -> Env {
    let mut asts = HashMap::new();
    for line in data.lines() {
        if let Some((name, expr)) = line.split_once(": ") {
            let val = if expr == "0" { Val(false) } else { Val(true) };
            asts.insert(name.to_string(), val);
        } else if let Some((expr, name)) = line.split_once(" -> ") {
            let mut expr = expr.split(' ');
            let a = expr.next().unwrap().to_string();
            let op = expr.next().unwrap();
            let b = expr.next().unwrap().to_string();
            let val = match op {
                "AND" => Gate { op: Op::And, a, b },
                "OR" => Gate { op: Op::Or, a, b },
                "XOR" => Gate { op: Op::Xor, a, b },
                _ => panic!("Unknown op {op}"),
            };
            asts.insert(name.to_string(), val);
        }
    }
    Env { env: asts }
}

// 2843606505 is too low!

#[time_function]
fn part1(data: &str) -> u64 {
    let mut env = parse_input(&data);
    let mut work = Vec::new();
    for name in env.env.keys().cloned() {
        if name.starts_with("z") {
            work.push(name);
        }
    }
    let mut answer = 0;
    while let Some(name) = work.pop() {
        let val = env.eval(&name);
        let offset = name[1..].parse::<u64>().unwrap();
        if val {
            answer |= 1 << offset;
        }
    }
    answer
}

#[time_function]
fn part2(data: &str) -> String {
    let mut env = parse_input(&data);
    let z_max = env
        .env
        .keys()
        .filter(|z| z.starts_with("z"))
        .map(|z| z[1..].parse::<u64>().unwrap())
        .max()
        .expect("no max!");
    let mut answer = Vec::new();
    // for each output bit z, check that it is fed from an xor.
    // If it's not, look at where its x & y bits feed; swap those back
    // in a normal full adder, x & y go to an xor & an and.
    // then that xor goes to an xor & an and (with the carry bit)
    // then that xor is the z,
    // and the and gets or'd with the first-tier end to be the carry_out
    /*
    x01 XOR y01 -> int1 // sqr
    x01 AND y01 -> int2 // kqf
    int1 XOR c00 -> z01 // mwk SWAP
    int1 AND c00 -> int3 //kgd and mwq is the c09
    int3 OR int2 -> c01 // z10 SWAP
     */
    for i in 0..z_max {
        // z_max is correctly an OR, not XOR
        let z = format!("z{i:02}");
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        if !is_xor(&env.env[&z]) {
            // find xor descendant that's not directly connected to x or y
            // that's what should connect to z
            let int1 = env
                .find3(Op::Xor, &x, &y)
                .expect("gotta be in here somewhere");
            let int2 = env
                .find3(Op::And, &x, &y)
                .expect("gotta be in here somewhere");
            let should_be_z = env.find2(Op::Xor, &int1).expect(&format!(
                "gotta be in here somewhere; {int1} looking for {z}"
            ));
            // whoops, descendants goes all the way to the end of the line!
            // maybe subtract out descendants of x+1, y+1?
            //            env.print_descendants(&vec![x, y]);
            // swap z and that descendant
            env.swap(&z, &should_be_z);
            println!("swapping {} and {}", &z, &should_be_z);
            answer.push(z);
            answer.push(should_be_z);
        }
    }
    // uh, I guess now we strobe a pair of 1s from low to high input bits &
    // look for bad carry?

    /*
    I found this by screwing around with lines 258-300, and seeing where it panicked.  It would be nice to clean this up
    and find them mechanically!  Also, something is FIXME-level broken with carry inputs, because they don't get set right
    by the swaps above.
    */
    answer.push("hsw".to_string());
    answer.push("jmh".to_string());
    env.swap("hsw", "jmh");

    for i in 0..z_max {
        let mut env = env.clone();
        for j in 0..=z_max {
            let x = format!("x{j:02}");
            let y = format!("y{j:02}");
            env.env.insert(x, Val(i == j));
            env.env.insert(y, Val(i == j));
        }
        let z1 = format!("z{:02}", i + 1);
        if !env.clone().eval(&z1) {
            println!("{} is missing a carry input", z1);
            let z = format!("z{i:02}");
            let x = format!("x{i:02}");
            let y = format!("y{i:02}");
            let int1 = env
                .find3(Op::Xor, &x, &y)
                .expect(&format!("gotta be in here somewhere, {x} XOR {y}"));
            let int2 = env
                .find3(Op::And, &x, &y)
                .expect("gotta be in here somewhere");
            let should_be_and = env.find2(Op::And, &int1).expect(&format!(
                "gotta be in here somewhere; {int1} looking for AND"
            ));
            // if int1 == "hsw" {
            //     answer.push("hsw".to_string());
            //     continue;
            // }
            let should_be_z = env.find2(Op::Xor, &int1).expect(&format!(
                "gotta be in here somewhere; {int1} looking for {z}"
            ));
            // should_be_or is the carry_out of x+y=z
            let should_be_or = env.find3(Op::Or, &int2, &should_be_and).expect(&format!(
                "gotta be in here somewhere; {int2} looking for OR"
            ));
            // find carry_in of z1 and swap
            assert_eq!(z, should_be_z);

            // for j in 0..=z_max {
            //     let z = format!("z{j:02}");
            //     println!("z{j} is {}", env.clone().eval(&z));
            // }
            // z1 itself isn't part of the answer; it's the carry bits upstream of it
            //            answer.push(z1);
        }
    }
    answer.sort();
    answer.join(",").to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(24, 2024)?;
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    const TESTBIGDATA: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTDATA), 4);
    }
    #[test]
    fn test_part1big() {
        assert_eq!(part1(&TESTBIGDATA), 2024);
    }
}
