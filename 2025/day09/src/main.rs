use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

fn parse_input() -> Vec<(i64, i64)> {
    let input = get_input();

    input
        .iter()
        .map(|l| {
            let values: Vec<&str> = l.split(",").collect();
            let x: i64 = values[0].parse().unwrap();
            let y: i64 = values[1].parse().unwrap();
            (x, y)
        })
        .collect()
}

fn calc_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let delta = |a: i64, b: i64| {
        return i64::abs(a - b) + 1;
    };

    return delta(a.0, b.0) * delta(a.1, b.1);
}

fn get_perimeter(input: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut perimeter = HashSet::new();

    let connect_points = |(x1, y1): (i64, i64), (x2, y2): (i64, i64)| {
        let mut line = Vec::new();

        let min_x = i64::min(x1, x2);
        let max_x = i64::max(x1, x2);
        let min_y = i64::min(y1, y2);
        let max_y = i64::max(y1, y2);

        if x1 == x2 {
            for y in min_y..=max_y {
                line.push((x1, y));
            }
        } else {
            for x in min_x..=max_x {
                line.push((x, y1));
            }
        }

        line
    };

    for window in input.windows(2) {
        perimeter.extend(connect_points(window[0], window[1]));
    }

    perimeter.extend(connect_points(*input.last().unwrap(), input[0]));

    perimeter
}

fn bounding_box(
    perimeter: &HashSet<(i64, i64)>,
    tiles: &Vec<(i64, i64)>,
    a: &usize,
    b: &usize,
) -> bool {
    let invalid_range = |a: i64, b: i64| {
        if a > b {
            return (b + 1)..(a);
        } else {
            return (a + 1)..b;
        }
    };

    let point1 = tiles[*a];
    let point2 = tiles[*b];

    let invalid_x = invalid_range(point1.0, point2.0);
    let invalid_y = invalid_range(point1.1, point2.1);

    for point in perimeter {
        if invalid_x.contains(&point.0) && invalid_y.contains(&point.1) {
            return false;
        }
    }

    true
}

fn part1(input: &Vec<(i64, i64)>) -> i64 {
    let size = input.len();
    let mut largest = 0;

    (0..size)
        .flat_map(|point_a| ((point_a + 1)..size).map(move |point_b| (point_b, point_a)))
        .filter(|&(point_b, point_a)| point_b > point_a)
        .for_each(|(point_b, point_a)| {
            let area = calc_area(input[point_a], input[point_b]);
            if area > largest {
                largest = area;
            }
        });

    largest
}

fn part2(input: &Vec<(i64, i64)>) -> i64 {
    let size = input.len();
    let mut largest = 0;

    let perimeter = get_perimeter(&input);

    let mut areas: Vec<(usize, usize, i64)> = (0..size)
        .flat_map(|point_a| ((point_a + 1)..size).map(move |point_b| (point_b, point_a)))
        .filter(|&(point_b, point_a)| point_b > point_a)
        .map(|(point_b, point_a)| (point_b, point_a, calc_area(input[point_b], input[point_a])))
        .collect();

    areas.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

    areas.iter().for_each(|(point_b, point_a, area)| {
        if bounding_box(&perimeter, &input, &point_b, &point_a) {
            if *area > largest {
                largest = *area;
                return;
            }
        }
    });

    for (point_b, point_a, area) in areas {
        if bounding_box(&perimeter, &input, &point_b, &point_a) {
            return area;
        }
    }

    largest
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
