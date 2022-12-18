// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::{
    collections::{BTreeSet, HashSet},
    io,
};

fn read_y() -> Option<i32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            Some(stripped_line.parse().unwrap())
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn read_data() -> Option<(Position, Position)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let data = stripped_line
                .replace("Sensor at x=", "")
                .replace(", y=", " ")
                .replace(": closest beacon is at x=", " ")
                .split(" ")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i32>>();

            Some((
                Position {
                    x: data[0],
                    y: data[1],
                },
                Position {
                    x: data[2],
                    y: data[3],
                },
            ))
        }
    }
}

fn manhattan_distance(sensor: &Position, beacon: &Position) -> i32 {
    (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()
}

fn manhattan_horiz(sensor: &Position, beacon: &Position, y_level: i32) -> Option<i32> {
    let distance = manhattan_distance(sensor, beacon);

    if (sensor.y - y_level).abs() > distance {
        return None;
    }

    Some(distance - (sensor.y - y_level).abs())
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
struct Segment {
    x1: i32,
    x2: i32,
}

fn build_segments(data: &Vec<(Position, Position)>, y_level: i32) -> BTreeSet<Segment> {
    let mut segments = BTreeSet::new();
    for (sensor, beacon) in data {
        match manhattan_horiz(sensor, beacon, y_level) {
            None => continue,
            Some(horiz_dist) => {
                let segment = Segment {
                    x1: sensor.x - horiz_dist,
                    x2: sensor.x + horiz_dist,
                };
                segments.insert(segment);
            }
        }
    }

    segments
}

fn calc_range(data: &Vec<(Position, Position)>, y_level: i32) -> i32 {
    let mut x1 = None;
    let mut x2 = None;
    let mut result = 0;

    for segment in build_segments(&data, y_level) {
        (x1, x2) = match (x1, x2) {
            (None, None) => (Some(segment.x1), Some(segment.x2)),
            (Some(x_start), Some(x_end)) => match segment.x1 > x_end {
                true => {
                    result += x_end - x_start + 1;
                    (Some(segment.x1), Some(segment.x2))
                }
                false => (Some(x_start), Some(std::cmp::max(x_end, segment.x2))),
            },
            _ => panic!("Unsupported case."),
        }
    }
    result += x2.unwrap() - x1.unwrap() + 1;

    let mut visited = HashSet::new();
    for (_, beacon) in data {
        if !visited.contains(&beacon) && beacon.y == y_level {
            visited.insert(beacon);
            result -= 1;
        }
    }

    result
}

const MIN_COORD: i32 = 0;
const MAX_COORD: i32 = 4000000;

fn find_frequency(data: &Vec<(Position, Position)>) -> i64 {
    for y in MIN_COORD..=MAX_COORD {
        // println!("y {}", y);
        let mut x1 = None;
        let mut x2 = None;

        for segment in build_segments(data, y) {
            let segment = Segment {
                x1: std::cmp::max(segment.x1, 0),
                x2: std::cmp::min(segment.x2, MAX_COORD),
            };

            (x1, x2) = match (x1, x2) {
                (None, None) => {
                    if segment.x1 != 0 {
                        return y as i64;
                    }
                    (Some(segment.x1), Some(segment.x2))
                }
                (Some(x_start), Some(x_end)) => match segment.x1 > x_end {
                    true => {
                        if segment.x1 - x_end > 1 {
                            return (x_end as i64 + 1) * MAX_COORD as i64 + y as i64;
                        }
                        (Some(segment.x1), Some(segment.x2))
                    }
                    false => (Some(x_start), Some(std::cmp::max(x_end, segment.x2))),
                },
                _ => panic!("Unsupported case."),
            }
        }
    }

    -1
}

#[allow(dead_code, unused)]
fn print_coverage(data: &Vec<(Position, Position)>) {
    for y in 0..=20 {
        let mut line = vec![' '; 20 + 1];
        for segment in build_segments(data, y) {
            let x_start = std::cmp::max(segment.x1, 0);
            let x_end = std::cmp::min(segment.x2, 20);
            for x in x_start..=x_end {
                line[x as usize] = '#';
            }
        }
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let y_level = read_y().unwrap();
    let mut data = vec![];

    while let Some((sensor, beacon)) = read_data() {
        data.push((sensor, beacon));
    }

    let result1 = calc_range(&data, y_level);
    println!("{}", result1);

    let result2 = find_frequency(&data);
    println!("{}", result2);
}
