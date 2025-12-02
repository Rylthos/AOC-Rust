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

fn parse_input() -> Vec<(u64, u64)> {
    let input = get_input();
    let line = &input[0];

    let ranges = line.split(",");

    ranges
        .map(|range| {
            let values: Vec<&str> = range.split("-").collect();

            let min: u64 = values[0].to_string().parse().unwrap();
            let max: u64 = values[1].to_string().parse().unwrap();
            (min, max)
        })
        .collect()
}

fn count_digits(value: u64) -> u32 {
    let mut digits = 0;
    let mut current_value = value;
    while current_value > 0 {
        digits += 1;
        current_value /= 10;
    }

    digits
}

fn repeated(value: u64, amount: u32) -> bool {
    let digits = count_digits(value);

    if digits % amount != 0 {
        return false;
    };

    let mask = u64::pow(10, digits / amount);

    let check = value % mask;
    let mut current_value = value;

    for _ in 1..amount {
        current_value /= mask;
        let check_value = current_value % mask;
        if check != check_value {
            return false;
        }
    }

    return true;
}

fn part1(ranges: Vec<(u64, u64)>) -> u64 {
    let mut sum: u64 = 0;
    for (min_range, max_range) in ranges {
        for i in min_range..=max_range {
            if repeated(i, 2) {
                sum += i;
            }
        }
    }
    sum
}

fn part2(ranges: Vec<(u64, u64)>) -> u64 {
    let mut sum: u64 = 0;
    for (min_range, max_range) in ranges {
        for i in min_range..=max_range {
            let digits = count_digits(i);
            for j in 2..=digits {
                if repeated(i, j) {
                    sum += i;
                    break;
                }
            }
        }
    }
    sum
}

fn main() {
    assert!(repeated(55, 2));
    assert!(repeated(6464, 2));
    assert!(repeated(123123, 2));

    assert!(repeated(1111111, 7));
    assert!(repeated(1212121212, 5));
    assert!(repeated(123123123, 3));
    assert!(!repeated(100, 3));

    let input = parse_input();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
