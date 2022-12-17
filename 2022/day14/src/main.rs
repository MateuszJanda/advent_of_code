// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::BTreeMap;
use std::{collections::HashSet, io};

fn read_path() -> Option<Vec<(i32, i32)>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let positions = stripped_line
                .split(" -> ")
                .filter_map(|position| {
                    position
                        .split_once(",")
                        .and_then(|(x, y)| Some((x.parse().unwrap(), y.parse().unwrap())))
                })
                .collect::<Vec<(i32, i32)>>();

            Some(positions)
        }
    }
}

#[derive(PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
struct Obstacle {
    begin: i32,
    end: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Sand {
    x: i32,
    y: i32,
}

impl Obstacle {
    fn new(val1: i32, val2: i32) -> Self {
        Obstacle {
            begin: std::cmp::min(val1, val2),
            end: std::cmp::max(val1, val2),
        }
    }

    fn is_contact(&self, pos: i32) -> bool {
        pos >= self.begin && pos <= self.end
    }
}

fn find_rocks(horiz_rocks: &BTreeMap<i32, Vec<Obstacle>>, x: i32, y: i32) -> Option<i32> {
    for (lvl, rocks) in horiz_rocks.range(y..) {
        if rocks.into_iter().find(|rock| rock.is_contact(x)).is_some() {
            return Some(*lvl);
        }
    }
    None
}

fn find_sand(sands: &HashSet<Sand>, x: i32, mut y: i32) -> Option<i32> {
    let mut result = None;
    loop {
        match sands.contains(&Sand { x, y }) {
            true => result = Some(y),
            false => break,
        }
        y -= 1;
    }

    result
}

fn is_obstacle(
    verti_rocks: &BTreeMap<i32, Vec<Obstacle>>,
    horiz_rocks: &BTreeMap<i32, Vec<Obstacle>>,
    sands: &HashSet<Sand>,
    x: i32,
    y: i32,
) -> bool {
    if let Some(rocks) = horiz_rocks.get(&y) {
        if rocks.into_iter().find(|rock| rock.is_contact(x)).is_some() {
            return true;
        }
    }

    if let Some(rocks) = verti_rocks.get(&x) {
        if rocks.into_iter().find(|rock| rock.is_contact(y)).is_some() {
            return true;
        }
    }

    sands.contains(&Sand { x, y })
}

enum Cmd {
    DropAgain(i32, i32),
    RestAt(i32, i32),
    Abyss,
}

fn drop_sand(
    verti_rocks: &BTreeMap<i32, Vec<Obstacle>>,
    horiz_rocks: &BTreeMap<i32, Vec<Obstacle>>,
    sands: &HashSet<Sand>,
    x: i32,
    y: i32,
) -> Cmd {
    match find_rocks(horiz_rocks, x, y) {
        Some(rock_level) => {
            println!("rock_level {}", rock_level);
            let y = rock_level - 1;

            match find_sand(sands, x, y) {
                Some(sand_level) => {
                    println!("Tutaj");
                    let y = sand_level - 1;
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1) {
                        Cmd::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1) {
                        Cmd::DropAgain(x + 1, y + 1)
                    } else {
                        Cmd::RestAt(x, y)
                    }
                }
                None => Cmd::RestAt(x, y),
            }
        }
        None => Cmd::Abyss,
    }
}
fn main() {
    // Position (like X) -> Vecotr of Ranges
    let mut verti_rocks: BTreeMap<i32, Vec<Obstacle>> = BTreeMap::new();
    let mut horiz_rocks: BTreeMap<i32, Vec<Obstacle>> = BTreeMap::new();
    let mut sands: HashSet<Sand> = HashSet::new();

    while let Some(path) = read_path() {
        let mut x = None;
        let mut y = None;
        for pos in path {
            match (x, y) {
                (None, None) => {
                    x = Some(pos.0);
                    y = Some(pos.1);
                }
                (Some(x_pos), Some(y_pos)) => {
                    if x_pos == pos.0 {
                        let obstacle = Obstacle::new(y_pos, pos.1);
                        let obstacles = verti_rocks.entry(x_pos).or_default();
                        obstacles.push(obstacle);
                    } else if y_pos == pos.1 {
                        let obstacle = Obstacle::new(x_pos, pos.0);
                        let obstacles = horiz_rocks.entry(y_pos).or_default();
                        obstacles.push(obstacle);
                    } else {
                        panic!("X != pos.0 or Y != pos.1");
                    }

                    x = Some(pos.0);
                    y = Some(pos.1);
                }
                (_, _) => panic!("Both X and Y should be initiated or set to None"),
            }
            println!("{}:{} ", pos.0, pos.1);
        }
    }

    let mut x = 500;
    let mut y = 0;

    println!("Start {} {}", x, y);
    while let cmd = drop_sand(&verti_rocks, &horiz_rocks, &sands, x, y) {
        match cmd {
            Cmd::Abyss => break,
            Cmd::DropAgain(x_pos, y_pos) => {
                println!("DropAgain {} {}", x_pos, y_pos);
                x = x_pos;
                y = y_pos;
            }
            Cmd::RestAt(x_pos, y_pos) => {
                println!("Rest {} {}", x_pos, y_pos);
                sands.insert(Sand { x: x_pos, y: y_pos });
                x = 500;
                y = 0;
            }
        }
    }

    println!("{}", sands.len());
}
