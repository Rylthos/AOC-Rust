use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::VecDeque;

type Devices = HashMap<String, Vec<String>>;

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

fn parse_input() -> Devices {
    let input = get_input();

    input
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split(" ");
            let head = split.next().unwrap();

            let arguments: Vec<String> = split.map(|s| s.to_string()).collect();
            (head[0..(head.len() - 1)].to_string(), arguments)
        })
        .collect()
}

fn paths(input: &Devices, start: &str, end: &str, memo: &mut HashMap<String, u64>) -> u64 {
    if memo.contains_key(start) {
        return memo[start];
    }

    if start == end {
        return 1;
    }

    if !input.contains_key(start) {
        return 0;
    }

    let mut count = 0;
    for entry in input[start].iter() {
        count += paths(input, entry, end, memo);
    }
    memo.insert(start.to_string(), count);
    count
}

fn part1(input: &Devices) -> u64 {
    paths(input, "you", "out", &mut HashMap::new())
}

fn part2(input: &Devices) -> u64 {
    let svr_dac = paths(input, "svr", "dac", &mut HashMap::new());
    let svr_fft = paths(input, "svr", "fft", &mut HashMap::new());

    let dac_fft = paths(input, "dac", "fft", &mut HashMap::new());
    let fft_dac = paths(input, "fft", "dac", &mut HashMap::new());

    let fft_out = paths(input, "fft", "out", &mut HashMap::new());
    let dac_out = paths(input, "dac", "out", &mut HashMap::new());

    return svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
