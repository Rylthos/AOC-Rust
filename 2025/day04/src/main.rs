use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::BTreeSet;

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

fn parse_input() -> BTreeSet<(isize, isize)> {
    let input = get_input();

    input
        .into_iter()
        .zip(0..)
        .map(|(l, y)| {
            l.chars()
                .zip(0..)
                .map(|(c, x)| ((x, y.clone()), c == '@'))
                .filter(|(_, b)| *b)
                .map(|(c, _)| c)
                .collect::<Vec<(isize, isize)>>()
        })
        .flatten()
        .collect()
}

fn removeable(grid: BTreeSet<(isize, isize)>) -> BTreeSet<(isize, isize)> {
    let mut remove = BTreeSet::new();

    let check = |pair| {
        if grid.contains(&pair) {
            1
        } else {
            0
        }
    };

    for (x, y) in grid.iter() {
        let mut local_count = 0;
        local_count += check((x - 1, y - 1));
        local_count += check((*x, y - 1));
        local_count += check((x + 1, y - 1));
        local_count += check((x - 1, *y));
        local_count += check((x + 1, *y));
        local_count += check((x - 1, y + 1));
        local_count += check((*x, y + 1));
        local_count += check((x + 1, y + 1));

        if local_count < 4 {
            remove.insert((*x, *y));
        }
    }

    remove
}

fn part1(input: BTreeSet<(isize, isize)>) -> u64 {
    let remove = removeable(input);
    remove.len() as u64
}

fn part2(input: BTreeSet<(isize, isize)>) -> u64 {
    let mut count = 0;
    let mut grid = input.clone();
    loop {
        let remove = removeable(grid.clone());
        count += remove.len() as u64;

        if remove.len() == 0 {
            break;
        }

        grid = grid.difference(&remove).cloned().collect();
    }
    count
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
