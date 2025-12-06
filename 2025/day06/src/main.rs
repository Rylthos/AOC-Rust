use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Input {
    Value(u64),
    Mult,
    Add,
}

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

fn parse_input() -> Vec<Vec<Input>> {
    let input = get_input();

    let raw: Vec<Vec<Input>> = input
        .iter()
        .map(|line| {
            line.split(" ")
                .filter(|x| x.len() != 0)
                .map(|s| {
                    if s == "*" {
                        Input::Mult
                    } else if s == "+" {
                        Input::Add
                    } else {
                        Input::Value(s.parse::<u64>().unwrap())
                    }
                })
                .collect()
        })
        .collect();

    let mut inputs = Vec::new();

    let problems = raw[0].len();
    let problem_length = raw.len();

    for problem_index in 0..problems {
        inputs.push(Vec::new());
        for index in 0..problem_length {
            inputs[problem_index].push(raw[index][problem_index])
        }
        inputs[problem_index] = inputs[problem_index].clone().into_iter().rev().collect();
    }

    inputs
}

fn parse_inputs_2() -> Vec<Vec<Input>> {
    let input = get_input();

    let operations_line = input[input.len() - 1].clone();

    let mut max_length = 0;
    for line in input.iter() {
        max_length = usize::max(max_length, line.len());
    }

    let mut op: char = operations_line.chars().nth(0).unwrap();
    let mut operations = Vec::new();
    let mut size = 0;
    let mut start = 0;

    let parse_op = |c: char| match c {
        '*' => Input::Mult,
        '+' => Input::Add,
        _ => panic!("Invalid"),
    };

    for i in operations_line[1..].chars() {
        size += 1;
        if i != ' ' {
            operations.push(((start, size - 1), parse_op(op)));

            start = size;
            op = i;
        }
    }
    operations.push(((start, max_length), parse_op(op)));

    let mut problems_raw: Vec<Vec<String>> = Vec::new();

    for line in &input[0..(input.len() - 1)] {
        problems_raw.push(Vec::new());

        for ((start, end), _) in operations.iter() {
            let len = line.len();
            let problems_len = problems_raw.len();

            let true_end = usize::min(len, *end);
            let mut substr = line[*start..true_end].to_string();

            if true_end != *end {
                let diff = end - true_end;
                for _ in 0..diff {
                    substr += " ";
                }
            }

            problems_raw[problems_len - 1].push(substr);
        }
    }

    let mut problems_t: Vec<Vec<String>> = Vec::new();

    let problems = problems_raw[0].len();
    let problem_length = problems_raw.len();

    for problem_index in 0..problems {
        problems_t.push(Vec::new());
        for index in 0..problem_length {
            problems_t[problem_index].push(problems_raw[index][problem_index].clone())
        }
        problems_t[problem_index] = problems_t[problem_index].clone();
    }

    let mut problems: Vec<Vec<Input>> = Vec::new();

    for (_, op) in operations.iter() {
        problems.push(vec![*op]);
    }

    for (index, problem) in (0..).zip(problems_t.iter()) {
        let len = problem[0].len();
        for i in 0..len {
            let mut value = 0;
            for j in problem {
                let ch = j.chars().nth(i).unwrap();

                match ch {
                    ' ' => (),
                    _ => {
                        let digit = (ch as u8) - ('0' as u8);
                        value = (value * 10) + (digit as u64);
                    }
                }
            }
            problems[index].push(Input::Value(value));
        }
    }

    problems
}

fn evaluate(input: &Vec<Vec<Input>>) -> u64 {
    let mut sum = 0;

    for problem in input {
        let operation = problem[0];
        let mut temp = if operation == Input::Mult { 1 } else { 0 };
        for input in &problem[1..problem.len()] {
            if let Input::Value(v) = input {
                match operation {
                    Input::Mult => temp *= v,
                    Input::Add => temp += v,
                    Input::Value(_) => panic!("Not possible"),
                }
            }
        }

        sum += temp
    }

    sum
}

fn main() {
    let input = parse_input();
    let input_2 = parse_inputs_2();
    println!("Part 1: {}", evaluate(&input));
    println!("Part 2: {}", evaluate(&input_2));
}
