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

fn parse_input() -> Vec<i16> {
    let input = get_input();

    let mut moves = Vec::new();

    for line in input {
        let direction = line.chars().nth(0).unwrap();
        let amount: i16 = line.get(1..).unwrap().parse().unwrap();

        if direction == 'L' {
            moves.push(-amount);
        } else {
            moves.push(amount);
        }
    }

    moves
}

fn pos_mod(value: i64, shift: i16, max: i64) -> i64 {
    let mut new_value = (value + (shift as i64)) % max;

    if new_value < 0 {
        new_value = max + new_value;
    }
    new_value
}

fn pos_mod_count(value: i64, shift: i16, max: i64) -> (i64, i64) {
    let mut new_value = value;

    let mut count = 0;

    let sign = if shift < 0 { -1 } else { 1 };
    let max_value = if shift < 0 { -shift } else { shift };

    for _ in 0..max_value {
        new_value += sign;

        if new_value == 0 || new_value == max {
            count += 1;
        }

        if new_value < 0 {
            new_value = max + new_value
        }
        if new_value >= max {
            new_value = new_value - max;
        }
    }

    (new_value, count)
}

fn part1(moves: Vec<i16>) -> i64 {
    let mut dial = 50;
    let mut count = 0;
    for m in moves {
        dial = pos_mod(dial, m, 100);
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part2(moves: Vec<i16>) -> i64 {
    let mut dial = 50;
    let mut count = 0;
    for m in moves {
        let new_count;
        (dial, new_count) = pos_mod_count(dial, m, 100);
        count += new_count
    }
    count
}

fn main() {
    let moves = parse_input();

    assert_eq!(pos_mod(0, 0, 100), 0);
    assert_eq!(pos_mod(0, -1, 100), 99);
    assert_eq!(pos_mod(99, 1, 100), 0);
    assert_eq!(pos_mod(5, -10, 100), 95);
    assert_eq!(pos_mod(95, 5, 100), 0);

    assert_eq!(pos_mod_count(50, -68, 100), (82, 1));
    assert_eq!(pos_mod_count(82, -30, 100), (52, 0));
    assert_eq!(pos_mod_count(52, 48, 100), (0, 1));
    assert_eq!(pos_mod_count(0, -5, 100), (95, 0));

    println!("Part 1: {}", part1(moves.clone()));
    println!("Part 2: {}", part2(moves));
}
