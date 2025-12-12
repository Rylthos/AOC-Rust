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

    s.split("\n").map(|s| s.to_string()).collect()
}

fn parse_input() -> (Vec<(u32, Vec<bool>)>, Vec<(u32, u32, Vec<u32>)>) {
    let input = get_input();

    let mut index = 0;

    let mut presents = Vec::new();

    for _ in 0..6 {
        let mut present = Vec::new();
        let mut count = 0;
        for i in 1..=3 {
            for char in input[index + i].chars() {
                if char == '#' {
                    present.push(true);
                    count += 1;
                } else {
                    present.push(false);
                }
            }
        }

        index += 5;

        presents.push((count, present));
    }

    let mut trees = Vec::new();
    for i in index..(input.len() - 1) {
        let mut values = input[i].split(" ");

        let mut size = values.next().unwrap().split('x');
        let width_value = size.next().unwrap();
        let width: u32 = width_value.parse().unwrap();
        let height_value = size.next().unwrap();
        let height: u32 = height_value[0..(height_value.len() - 1)].parse().unwrap();

        let mut amount: Vec<u32> = Vec::new();
        for a in values {
            amount.push(a.parse().unwrap());
        }

        trees.push((width, height, amount));
    }

    (presents, trees)
}

fn part1(presents: &Vec<(u32, Vec<bool>)>, trees: &Vec<(u32, u32, Vec<u32>)>) -> usize {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for (i, (w, h, elements)) in (0..).zip(trees) {
        let total_size = w * h;

        let max_presents = (w / 3) * (h / 3);

        let mut total_present_size = 0;
        let mut total_presents = 0;
        for (j, e) in (0..).zip(elements) {
            total_present_size += e * presents[j].0;
            total_presents += e;
        }

        // Perfect packing
        if total_present_size > total_size {
            invalid.push(i);
            continue;
        }

        // Worst packing
        if total_presents > max_presents {
            invalid.push(i);
            continue;
        }

        valid.push(i);
    }

    println!("Total: {}", trees.len());
    println!("Invalid: {}", invalid.len());
    println!("Valid: {}", valid.len());

    valid.len()
}

fn main() {
    let (presents, trees) = parse_input();

    println!("Part 1: {}", part1(&presents, &trees));
}
