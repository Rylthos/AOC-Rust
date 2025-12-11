use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::VecDeque;

use z3::ast::Int;
use z3::{Optimize, Solver};

type Target = Vec<bool>;
type Wiring = Vec<Vec<usize>>;
type Joltage = Vec<usize>;

fn get_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}", display, err),
        Ok(_) => (),
    }

    s.split("\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.to_string())
        .collect()
}

fn parse_input() -> Vec<(Target, Wiring, Joltage)> {
    let mut output = Vec::new();
    let input = get_input();
    for line in input {
        let mut target = Vec::new();
        let mut wiring = Vec::new();
        let mut joltage = Vec::new();
        let section = line.split(" ");
        for sect in section {
            match sect.chars().nth(0).unwrap() {
                '[' => {
                    let target_area = sect[1..(sect.len() - 1)].to_string();

                    for ch in target_area.chars() {
                        if ch == '.' {
                            target.push(false);
                        } else {
                            target.push(true);
                        }
                    }
                }
                '(' => {
                    let target_area = sect[1..(sect.len() - 1)].to_string();

                    wiring.push(target_area.split(",").map(|s| s.parse().unwrap()).collect());
                }
                '{' => {
                    let target_area = sect[1..(sect.len() - 1)].to_string();

                    joltage = target_area.split(",").map(|s| s.parse().unwrap()).collect();
                }
                _ => panic!("Not possible"),
            }
        }

        output.push((target, wiring, joltage))
    }

    output
}

fn min_changes(target: &Target, wiring: &Wiring) -> usize {
    let mut stack = VecDeque::new();
    let initial: Target = target.iter().map(|_| false).collect();
    stack.push_back((initial, 0));

    while stack.len() != 0 {
        let (entry, steps) = stack.pop_front().unwrap();

        let mut valid = true;
        for (e, t) in entry.iter().zip(target) {
            if e != t {
                valid = false;
                break;
            }
        }
        if valid {
            return steps;
        }

        for wire in wiring {
            let mut new_wire = entry.clone();
            for change in wire {
                new_wire[*change] = !new_wire[*change];
            }
            stack.push_back((new_wire, steps + 1));
        }
    }

    panic!("Invalid");
}

fn min_changes_jolt(target: &Joltage, wiring: &Wiring) -> usize {
    let opt = Optimize::new();

    let targets = target.len();
    let options = wiring.len();

    let mut joltages: Vec<Vec<Int>> = Vec::new();
    for wire in wiring {
        let mut temp: Vec<Int> = target.iter().map(|_| Int::from_u64(0)).collect();
        for change in wire {
            temp[*change] = Int::from_u64(1);
        }
        joltages.push(temp);
    }

    let mut times_used = Vec::new();
    for i in 0..options {
        let var = Int::new_const(format!("x_{}", i));
        opt.assert(&var.ge(0));
        times_used.push(var);
    }

    for d in 0..targets {
        let mut sum = Int::from_u64(0);

        for i in 0..options {
            sum = Int::add(&[&sum, &Int::mul(&[&joltages[i][d], &times_used[i]])])
        }

        opt.assert(&sum.eq(&Int::from_u64(target[d] as u64)));
    }

    let mut total = Int::from_u64(0);
    for i in 0..options {
        total = Int::add(&[&total, &times_used[i]]);
    }

    opt.minimize(&total);

    match opt.check(&[]) {
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let mut total_uses = 0;

            for i in 0..options {
                let v = model
                    .get_const_interp(&times_used[i])
                    .unwrap()
                    .as_u64()
                    .unwrap();
                total_uses += v;
            }

            return total_uses as usize;
        }

        other => panic!("Unsat or Unknown: {:?}", other),
    }
}

fn part1(input: &Vec<(Target, Wiring, Joltage)>) -> usize {
    let mut count = 0;
    for (target, wiring, _) in input {
        count += min_changes(&target, &wiring);
    }
    count
}

fn part2(input: &Vec<(Target, Wiring, Joltage)>) -> usize {
    let mut count = 0;
    for (_, wiring, joltage) in input {
        count += min_changes_jolt(&joltage, &wiring);
    }
    count
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
