use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::BTreeSet;

use closed_interval_set::RangeVec;

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

fn parse_input() -> (RangeVec<u64>, BTreeSet<u64>) {
    let input = get_input();

    let ranges: Vec<(u64, u64)> = input
        .iter()
        .filter(|s| s.contains('-'))
        .map(|s| {
            let values: Vec<&str> = s.split("-").collect();
            let low: u64 = values[0].parse().unwrap();
            let high: u64 = values[1].parse().unwrap();
            (low, high)
        })
        .collect();

    let mut intervals = RangeVec::new();
    for (low, high) in ranges {
        intervals = intervals.union(RangeVec::from_vec(vec![(low, high)]))
    }

    let ids: BTreeSet<u64> = input
        .into_iter()
        .filter(|s| !s.contains('-') && s.len() != 0)
        .map(|s| s.parse().unwrap())
        .collect();

    (intervals, ids)
}

fn part1(ranges: &RangeVec<u64>, ids: &BTreeSet<u64>) -> usize {
    let mut count = 0;
    for id in ids {
        if ranges.contains(&RangeVec::from_vec(vec![(*id, *id)])) {
            count += 1;
        }
    }

    count
}

fn part2(intervals: &RangeVec<u64>) -> u64 {
    intervals
        .iter()
        .fold(0, |acc, (low, high)| acc + ((high + 1) - low))
}

fn main() {
    let (intervals, ids) = parse_input();

    println!("Part 1: {}", part1(&intervals, &ids));
    println!("Part 2: {}", part2(&intervals));
}
