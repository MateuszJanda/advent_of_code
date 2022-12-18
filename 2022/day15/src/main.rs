// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

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

#[derive(Clone, Debug)]
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

fn manhattan_distance(pos1: &Position, pos2: &Position) -> i32 {
    let r = (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs();
    // println!("{:?} {:?} {}", pos1, pos2, r);
    r
}

fn manhattan_horiz(pos1: &Position, pos2: &Position) -> i32 {
    (pos1.x - pos2.x).abs()
}

fn range_length(data: &Vec<(Position, Position)>, y_level: i32) -> i32 {
    let mut x_min = None;
    let mut x_max = None;

    for (sensor, beacon) in data {
        let distance = manhattan_distance(sensor, beacon);

        if (sensor.y - y_level).abs() > distance {
            continue;
        }

        println!("Pos {:?} {:?} {} {} {}", sensor, beacon, distance, sensor.x - distance, sensor.x + distance);

        x_min = match x_min {
            Some(x) => Some(std::cmp::min(x, sensor.x - manhattan_horiz(sensor, beacon))),
            None => Some(sensor.x - distance),
        };

        x_max = match x_max {
            Some(x) => Some(std::cmp::max(x, sensor.x + manhattan_horiz(sensor, beacon))),
            None => Some(sensor.x + distance),
        };
    }

    println!("{:?} {:?}", x_min, x_max);

    x_max.unwrap() - x_min.unwrap()
}

fn main() {
    let y_level = read_y().unwrap();
    // let mut x_min = None;
    // let mut x_max = None;
    let mut data = vec![];

    while let Some((sensor, beacon)) = read_data() {
        // x_min = match x_min {
        //     Some(x) => Some(*vec![x, sensor.x, beacon.x].iter().min().unwrap()),
        //     None => Some(std::cmp::min(sensor.x, beacon.x)),
        // };

        // x_max = match x_max {
        //     Some(x) => Some(*vec![x, sensor.x, beacon.x].iter().max().unwrap()),
        //     None => Some(std::cmp::max(sensor.x, beacon.x)),
        // };

        data.push((sensor, beacon));
    }

    // let range_length = x_max.unwrap() = x_min.unwrap();

    // [-4, 26]
    range_length(&data, y_level);

    // println!("{:?} {:?}", x_min, x_max)
}
