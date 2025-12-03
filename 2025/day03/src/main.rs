use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

fn parse_input() -> Vec<Vec<u8>> {
    let input = get_input();

    input
        .into_iter()
        .map(|line| line.chars().map(|c| (c as u8) - ('0' as u8)).collect())
        .collect()
}

fn find_max(arr: &[u8]) -> (u8, usize) {
    let mut max_value: u8 = 0;
    let mut index: usize = 0;
    for (i, &v) in (0..).zip(arr) {
        if v > max_value {
            max_value = v;
            index = i;
        }
    }
    (max_value, index)
}

fn part1(batteries: Vec<Vec<u8>>) -> u64 {
    let mut sum: u64 = 0;
    for bank in batteries {
        let slice = bank.as_slice();
        let (value, index) = find_max(&slice[..(slice.len() - 1)]);
        let (second_value, _) = find_max(&slice[(index + 1)..]);

        sum += (value as u64) * 10 + (second_value as u64);
    }
    sum
}

fn part2(batteries: Vec<Vec<u8>>) -> u64 {
    let mut sum: u64 = 0;
    for bank in batteries {
        let mut intermediate: u64 = 0;
        let mut current_index = 0;
        for i in (0..12).rev() {
            let slice = bank.as_slice();
            let (value, index) = find_max(&slice[current_index..(slice.len() - i)]);
            current_index += index + 1;

            intermediate = (intermediate * 10) + (value as u64);
        }
        sum += intermediate;
    }
    sum
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
