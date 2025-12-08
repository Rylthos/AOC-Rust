use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

use disjoint::DisjointSet;

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

fn parse_input() -> (
    HashMap<usize, (i64, i64, i64)>,
    HashMap<(usize, usize), i64>,
) {
    let input = get_input();

    let mut distance = HashMap::new();

    let lookup: HashMap<usize, (i64, i64, i64)> = input
        .iter()
        .zip(0..)
        .map(|(l, i)| {
            let values: Vec<&str> = l.split(",").collect();
            let x: i64 = values[0].parse().unwrap();
            let y: i64 = values[1].parse().unwrap();
            let z: i64 = values[2].parse().unwrap();

            (i, (x, y, z))
        })
        .collect();

    for i in 0..(lookup.len() - 1) {
        for j in (i + 1)..(lookup.len()) {
            let (x1, y1, z1) = lookup.get(&i).unwrap().clone();
            let (x2, y2, z2) = lookup.get(&j).unwrap().clone();

            let x = x2 - x1;
            let y = y2 - y1;
            let z = z2 - z1;

            let dist = x * x + y * y + z * z;
            distance.insert((i, j), dist);
        }
    }

    (lookup, distance)
}

fn next_smallest(
    distances: &HashMap<(usize, usize), i64>,
    prev: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut smallest = 10000000000;
    let mut previ = 0;
    let mut prevj = 0;
    for ((i, j), d) in distances {
        if *d < smallest && !prev.contains(&(*i, *j)) {
            smallest = *d;

            previ = *i;
            prevj = *j;
        }
    }

    prev.insert((previ, prevj));

    (previ, prevj)
}

fn part1(
    lookup: &HashMap<usize, (i64, i64, i64)>,
    distances: &HashMap<(usize, usize), i64>,
) -> usize {
    let mut set = DisjointSet::with_len(lookup.len());
    let mut prev_dist = HashSet::new();

    let pairs = if lookup.len() == 1000 {
        1000 // Input
    } else {
        10 // Example
    };

    for _ in 0..pairs {
        let (i, j) = next_smallest(distances, &mut prev_dist);
        set.join(i, j);
    }

    let sets = set.sets();

    let mut prev = HashSet::new();
    let mut max_value = || {
        let (max, i) = sets.iter().zip(0..).fold((0, 0), |(acc, acci), (x, i)| {
            if x.len() > acc && !prev.contains(&i) {
                (x.len(), i)
            } else {
                (acc, acci)
            }
        });

        prev.insert(i);
        max
    };

    let max1 = max_value();
    let max2 = max_value();
    let max3 = max_value();

    max1 * max2 * max3
}

fn part2(
    lookup: &HashMap<usize, (i64, i64, i64)>,
    distances: &HashMap<(usize, usize), i64>,
) -> usize {
    let mut set = DisjointSet::with_len(lookup.len());
    let mut prev_dist = HashSet::new();

    for _ in 0..lookup.len() {
        let (i, j) = next_smallest(distances, &mut prev_dist);
        set.join(i, j);
    }

    loop {
        let (i, j) = next_smallest(distances, &mut prev_dist);
        set.join(i, j);

        let sets = set.sets();

        if sets.len() == 1 {
            let (x1, _, _) = lookup.get(&i).unwrap();
            let (x2, _, _) = lookup.get(&j).unwrap();
            return (x1 * x2) as usize;
        } else {
            for _ in 0..(sets.len() - 1) {
                let (i, j) = next_smallest(distances, &mut prev_dist);
                set.join(i, j);
            }
        }
    }
}

fn main() {
    let (lookup, distances) = parse_input();

    println!("Part 1: {}", part1(&lookup, &distances));
    println!("Part 2: {}", part2(&lookup, &distances));
}
