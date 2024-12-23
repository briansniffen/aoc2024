use aochelpers::get_daily_input;
use code_timing_macros::time_function;
use colored::Colorize;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u64, multi::separated_list1, IResult,
};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    program_counter: usize,
    output: Vec<u64>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Register A: {}\nRegister B: {}\nRegister C: {}\n\nProgram: ",
            self.a, self.b, self.c
        )?;
        for (i, line) in self.program.iter().enumerate() {
            if i == self.program_counter {
                write!(f, "{}", line.to_string().red().bold())?
            } else {
                write!(f, "{}", line)?
            }
            if i != self.program.len() - 1 {
                write!(f, ",")?
            }
        }
        write!(f, "\n\nOutput: ")?;
        for (i, line) in self.output.iter().enumerate() {
            if i != self.output.len() - 1 {
                write!(f, "{},", line)?
            } else {
                write!(f, "{}", line)?
            }
        }
        Ok(())
    }
}

const TWO: u64 = 2;

impl State {
    fn new(a: u64, program: Vec<u64>) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            program,
            program_counter: 0,
            output: Vec::new(),
        }
    }
    fn combo(&self, arg: u64) -> u64 {
        match arg {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            x => x,
        }
    }
    fn step(&mut self) {
        let instr = self.program[self.program_counter];
        let arg = self.program[self.program_counter + 1];
        match instr {
            0 =>
            // adv
            {
                let numerator = self.a;
                let denominator = TWO.pow(self.combo(arg).try_into().unwrap());
                self.a = numerator / denominator;
                self.program_counter += 2;
            }
            1 =>
            // bxl
            {
                self.b ^= arg;
                self.program_counter += 2;
            }
            2 =>
            // bst
            {
                self.b = self.combo(arg) % 8;
                self.program_counter += 2;
            }
            3 =>
            // jnz
            {
                if self.a != 0 {
                    self.program_counter = arg as usize;
                } else {
                    self.program_counter += 2;
                }
            }
            4 =>
            // bxc
            {
                self.b ^= self.c;
                self.program_counter += 2;
            }
            5 =>
            // out
            {
                self.output.push(self.combo(arg) % 8);
                self.program_counter += 2;
            }
            6 =>
            // bdv
            {
                let numerator = self.a;
                let denominator = TWO.pow(self.combo(arg).try_into().unwrap());
                self.b = numerator / denominator;
                self.program_counter += 2;
            }
            7 =>
            // cdv
            {
                let numerator = self.a;
                let denominator = TWO.pow(self.combo(arg).try_into().unwrap());
                self.c = numerator / denominator;
                self.program_counter += 2;
            }
            _ => {
                panic!("Invalid instruction: {} @ {} ", instr, self.program_counter)
            }
        }
    }
    fn run(&mut self, trace: bool) {
        while self.program_counter < self.program.len() {
            self.step();
            if trace {
                print!("\x1B[2J\x1B[1;1H"); // clear screen
                println!("{}", self);
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
    fn disassemble(&self) {
        let mut i = 0;
        while i < self.program.len() {
            let instr = self.program[i];
            let arg = self.program[i + 1];
            print!("{:4} ", i);
            match instr {
                0 => println!("adv {}", discombo(arg)),
                1 => println!("bxl {}", arg),
                2 => println!("bst {}", discombo(arg)),
                3 => println!("jnz {}", arg),
                4 => println!("bxc _{}", arg),
                5 => println!("out {}", discombo(arg)),
                6 => println!("bdv {}", discombo(arg)),
                7 => println!("cdv {}", discombo(arg)),
                _ => panic!("Invalid instruction: {} @ {} ", instr, i),
            }
            i += 2;
        }
    }
}

fn discombo(arg: u64) -> String {
    match arg {
        4 => "a".to_string(),
        5 => "b".to_string(),
        6 => "c".to_string(),
        7 => "error".to_string(),
        x => x.to_string(),
    }
}

fn register(i: &str) -> IResult<&str, u64> {
    let (i, _) = tag("Register ")(i)?;
    let (i, _) = alt((tag("A"), tag("B"), tag("C")))(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, val) = u64(i)?;
    let (i, _) = tag("\n")(i)?;
    Ok((i, val))
}

fn program(i: &str) -> IResult<&str, Vec<u64>> {
    let (i, _) = tag("Program: ")(i)?;
    separated_list1(tag(","), u64)(i)
}

fn parse_input(i: &str) -> IResult<&str, State> {
    let (i, a) = register(i)?;
    let (i, b) = register(i)?;
    let (i, c) = register(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, program) = program(i)?;
    assert_eq!(i, "");
    Ok((
        i,
        State {
            a,
            b,
            c,
            program,
            program_counter: 0,
            output: Vec::new(),
        },
    ))
}

#[time_function]
fn part1(data: &str) -> State {
    let (_, mut p) = parse_input(data).unwrap();
    p.run(false);
    p
}

/* Program text:
 0 bst a  ; b = a % 8;
 2 bxl 7  ; b ^= 7;
 4 cdv b  ; c = a / 2**b; == a >> b
 6 bxl 7  ; b ^= 7;
 8 bxc _6 ; b ^= c;
10 adv 3  ; a = a/8; == a >> 3
12 out b  ; println!(b)
14 jnz 0  ; if a!= 0 {goto 0;}

In other words, a is a *program*, executed 3 bits at a time.
Each time through the loop, we select the low 3 bits of a into b.
Then invert.  c gets a >> b.  Invert b back, then XOR in C.  output that,
then nudge a down and loop.  So each loop is:

loop {
    b = a & 7;
    c = a >> (b^7)
    b ^= c
    out(b&7)
    a = a >> 3
}

loop {
b = a&7;
out(b ^ (a >> (7-b)) & 7);
a = a >> 3;
}

Simplifying:

loop {
    out(a&7 ^ (a >> (a&7)^7) & 7)
    a = a >> 3
}

So that's the low 3 bits of a XOR some 3 bits of a selected by
the inverse of the low 3 bits of a.  And so we can only look up to 7 bits leftward
with that shift, so the low 7 bits dictate the output.

 */

fn simulate(a: u64) -> Vec<u64> {
    let mut a: u64 = a;
    // let mut b: u64 = 0;
    // let mut c: u64 = 0;
    let mut out: Vec<u64> = Vec::new();

    while a != 0 {
        // b = a & 7;
        // b ^= 7;
        // c = a >> b;
        // b ^= 7;
        // b ^= c;
        // a = a >> 3;
        // out.push(b & 7);
        out.push(a & 7 ^ (a >> ((a & 7) ^ 7)) & 7);
        a >>= 3;
    }
    out
}

// fn simulate_check(a: u64, target: &Vec<u64>) -> bool {
//     let mut a: u64 = a;
//     let mut b: u64 = 0;
//     let mut c: u64 = 0;
//     let mut i: usize = 0;

//     while a != 0 {
//         b = a & 7;
//         b ^= 7;
//         c = a >> b;
//         b ^= 7;
//         b ^= c;
//         a = a >> 3;
//         if (b & 7) == target[i] {
//             i += 1;
//         } else {
//             return false;
//         }
//     }
//     i == target.len()
// }

#[time_function]
fn part2(data: &str) -> u64 {
    let (_, p) = parse_input(data).unwrap();
    search(&p.program, 0, 0).unwrap()
}

// This finds a bad answer, 265078544466843, the seventh smallest answer.  I'm not sure why!
// fn search(goal: &Vec<u64>, initial_seed: u64, j: usize) -> Result<u64, String> {
//     let mut seed = initial_seed;
//     for i in 0..goal.len() {
//         seed <<= 3;
//         loop {
//             seed += 1;
//             let out = simulate(seed);
//             if simulate(seed)[..i + 1] == goal[goal.len() - 1 - i..] {
//                 println!("Found intermediate seed: {:o}", seed);
//                 // println!("Simulated: {:?}\nGoal:      {:?}", simulate(seed), goal);
//                 println!(
//                     "Simulated: {:?}\nGoal:      {:?}",
//                     out,
//                     &goal[goal.len() - out.len()..]
//                 );
//                 break;
//             }
//         }
//     }
//     if simulate(seed) == *goal {
//         println!("Found seed: {} (0o{:o})", seed, seed);
//         println!("Simulated: {:?}\nGoal:      {:?}", simulate(seed), goal);
//         return Ok(seed);
//     }
//     Err("no match".to_string())
// }

fn search(goal: &Vec<u64>, initial_seed: u64, j: usize) -> Result<u64, String> {
    let mut candidates: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    for i in 1..8 {
        candidates.push(Reverse(i));
    }

    while let Some(Reverse(candidate)) = candidates.pop() {
        let out = simulate(candidate);
        if out == *goal {
            //            println!("Found seed: {} (0o{:o})", candidate, candidate);
            return Ok(candidate);
        }
        if out == goal[goal.len() - out.len()..] {
            // println!("Found intermediate seed: {:o}", candidate);
            for i in 0..8 {
                candidates.push(Reverse((candidate << 3) + i));
            }
        }
    }
    Err("no match".to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(17, 2024)?;
    // let prog = parse_input(&data).unwrap().1;
    // println!("## Program ##\n{}", prog);
    // println!("\n## Disassembly ##\n");
    // prog.disassemble();
    println!("\n## Part I ##\n{}", part1(&data));
    println!("\n##Part II ##\n{}", part2(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    const TEST2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    use super::*;

    #[test]
    fn test_part1() {
        let prog = part1(&TESTDATA);
        assert_eq!(prog.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_simulate() {
        let out = simulate(729);
        assert_eq!(out, vec![2, 6, 3, 1]);
    }

    #[test]
    fn test_part2() {
        let data = get_daily_input(17, 2024).unwrap();
        let (_, mut p) = parse_input(&data).unwrap();
        let out = simulate(p.a);
        p.run(false);
        assert_eq!(out, p.output);
    }
}
