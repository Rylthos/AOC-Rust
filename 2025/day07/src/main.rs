use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

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

fn parse_input() -> (i64, i64, HashSet<(i64, i64)>) {
    let input = get_input();

    let mut start_pos = -1;
    let height = input.len();

    let initial = input[0].clone();
    for (i, chr) in (0..).zip(initial.chars()) {
        match chr {
            'S' => {
                start_pos = i;
                break;
            }
            _ => (),
        }
    }
    let splitters = input
        .iter()
        .zip(0..)
        .map(|(l, y)| {
            l.chars()
                .zip(0..)
                .map(move |(c, x)| ((x, y), c))
                .filter(|(_, c)| *c == '^')
        })
        .flatten()
        .map(|(p, _)| p)
        .collect();

    (start_pos, height as i64, splitters)
}

fn part1(start_index: i64, height: i64, splitters: &HashSet<(i64, i64)>) -> usize {
    let mut current = HashSet::new();
    current.insert((start_index, 0));

    let mut splits = 0;

    for _ in 1..height {
        let mut next: HashSet<(i64, i64)> = HashSet::new();
        for (x, y) in current.iter() {
            if splitters.contains(&(*x, y + 1)) {
                next.insert((x - 1, y + 1));
                next.insert((x + 1, y + 1));

                splits += 1;
            } else {
                next.insert((*x, y + 1));
            }
        }

        current = next;
    }

    splits
}

fn traverse(
    (x, y): (i64, i64),
    height: i64,
    splitters: &HashSet<(i64, i64)>,
    mem: &mut HashMap<(i64, i64), usize>,
) -> usize {
    if y >= height {
        return 1;
    }

    if mem.contains_key(&(x, y)) {
        return *mem.get(&(x, y)).unwrap();
    }

    if splitters.contains(&(x, y + 1)) {
        let mut temp = 0;
        temp += traverse((x - 1, y + 1), height, splitters, mem);
        temp += traverse((x + 1, y + 1), height, splitters, mem);

        mem.insert((x, y), temp);

        return temp;
    } else {
        return traverse((x, y + 1), height, &splitters, mem);
    }
}

fn part2(start_index: i64, height: i64, splitters: &HashSet<(i64, i64)>) -> usize {
    let mut mem = HashMap::new();
    return traverse((start_index, 0), height, splitters, &mut mem);
}

fn main() {
    let (start, size, splitters) = parse_input();

    println!("Part 1: {}", part1(start, size, &splitters));
    println!("Part 2: {}", part2(start, size, &splitters));
}
