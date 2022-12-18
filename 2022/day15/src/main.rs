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
        // let distance = manhattan_distance(sensor, beacon);

        // if (sensor.y - y_level).abs() > distance {
        //     continue;
        // }

        match manhattan_horiz(sensor, beacon, y_level) {
            None => continue,
            Some(horiz_dist) => {
                let segment = Segment {
                    x1: sensor.x - horiz_dist,
                    x2: sensor.x + horiz_dist,
                };
                println!("Pos {:?} {:?} {:?}", sensor, beacon, segment);
                segments.insert(segment);
            }
        }
    }

    for y in -2..=16 {
        let sensor = Position { x: 8, y: 7 };
        let beacon = Position { x: 2, y: 10 };

        match manhattan_horiz(&sensor, &beacon, y) {
            None => continue,
            Some(horiz_dist) => {
                let segment = Segment {
                    x1: sensor.x - horiz_dist,
                    x2: sensor.x + horiz_dist,
                };
                println!("Test {} {:?} {}", y, segment, segment.x2 - segment.x1 + 1);
                segments.insert(segment);
            }
        }
    }

    segments
}

fn calc_range(segments: &BTreeSet<Segment>, data: &Vec<(Position, Position)>, y_level: i32) -> i32 {
    let mut x1 = None;
    let mut x2 = None;
    let mut result = 0;

    for segment in segments {
        println!("Segment {:?} {:?} {:?} ", segment, x1, x2);
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
    for (sensor, beacon) in data {
        if !visited.contains(&sensor) && sensor.y == y_level {
            visited.insert(sensor);
            result -= 1;
            println!("TUTAJ");
        }

        if !visited.contains(&beacon) && beacon.y == y_level {
            visited.insert(beacon);
            result -= 1;
        }
    }

    result
}

fn main() {
    let y_level = read_y().unwrap();
    let mut data = vec![];

    while let Some((sensor, beacon)) = read_data() {
        data.push((sensor, beacon));
    }

    let segments = build_segments(&data, y_level);
    let result1 = calc_range(&segments, &data, y_level);
    println!("{}", result1);
}
