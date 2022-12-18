// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::BTreeMap;
use std::{collections::HashSet, io};

const START_X: i32 = 500;
const START_Y: i32 = 0;

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
struct Rock {
    begin: i32,
    end: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Sand {
    x: i32,
    y: i32,
}

impl Rock {
    fn new(val1: i32, val2: i32) -> Self {
        Rock {
            begin: std::cmp::min(val1, val2),
            end: std::cmp::max(val1, val2),
        }
    }

    fn is_contact(&self, pos: i32) -> bool {
        pos >= self.begin && pos <= self.end
    }
}

fn find_rocks(horiz_rocks: &BTreeMap<i32, Vec<Rock>>, x: i32, y: i32) -> Option<i32> {
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
    verti_rocks: &BTreeMap<i32, Vec<Rock>>,
    horiz_rocks: &BTreeMap<i32, Vec<Rock>>,
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

fn is_inf_floor(highest_y: i32, y: i32) -> bool {
    y >= highest_y
}

enum CmdPart1 {
    DropAgain(i32, i32),
    RestAt(i32, i32),
    Abyss,
}

fn drop_sand_to_abyss(
    verti_rocks: &BTreeMap<i32, Vec<Rock>>,
    horiz_rocks: &BTreeMap<i32, Vec<Rock>>,
    sands: &HashSet<Sand>,
    x: i32,
    y: i32,
) -> CmdPart1 {
    match find_rocks(horiz_rocks, x, y) {
        Some(rock_level) => {
            let y = rock_level - 1;

            match find_sand(sands, x, y) {
                Some(sand_level) => {
                    let y = sand_level - 1;
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1) {
                        CmdPart1::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1) {
                        CmdPart1::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart1::RestAt(x, y)
                    }
                }
                None => {
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1) {
                        CmdPart1::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1) {
                        CmdPart1::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart1::RestAt(x, y)
                    }
                }
            }
        }
        None => CmdPart1::Abyss,
    }
}

fn simulation_part1(
    verti_rocks: &BTreeMap<i32, Vec<Rock>>,
    horiz_rocks: &BTreeMap<i32, Vec<Rock>>,
) -> usize {
    let mut sands: HashSet<Sand> = HashSet::new();

    let mut x = START_X;
    let mut y = START_Y;

    loop {
        match drop_sand_to_abyss(&verti_rocks, &horiz_rocks, &sands, x, y) {
            CmdPart1::Abyss => break,
            CmdPart1::DropAgain(x_pos, y_pos) => {
                x = x_pos;
                y = y_pos;
            }
            CmdPart1::RestAt(x_pos, y_pos) => {
                sands.insert(Sand { x: x_pos, y: y_pos });
                x = START_X;
                y = START_Y;
            }
        }
    }

    return sands.len();
}

enum CmdPart2 {
    DropAgain(i32, i32),
    RestAt(i32, i32),
}

fn drop_sand_to_inf_floor(
    verti_rocks: &BTreeMap<i32, Vec<Rock>>,
    horiz_rocks: &BTreeMap<i32, Vec<Rock>>,
    sands: &HashSet<Sand>,
    highest_y: i32,
    x: i32,
    y: i32,
) -> CmdPart2 {
    match find_rocks(horiz_rocks, x, y) {
        Some(rock_level) => {
            let y = rock_level - 1;

            match find_sand(sands, x, y) {
                Some(sand_level) => {
                    let y = sand_level - 1;
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart2::RestAt(x, y)
                    }
                }
                None => {
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart2::RestAt(x, y)
                    }
                }
            }
        }
        None => {
            let y = highest_y as i32 - 1;

            match find_sand(sands, x, y) {
                Some(sand_level) => {
                    let y = sand_level - 1;
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart2::RestAt(x, y)
                    }
                }
                None => {
                    // Move down-left
                    if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x - 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x - 1, y + 1)
                    // Move down-right
                    } else if !is_obstacle(&verti_rocks, &horiz_rocks, &sands, x + 1, y + 1)
                        && !is_inf_floor(highest_y, y + 1)
                    {
                        CmdPart2::DropAgain(x + 1, y + 1)
                    } else {
                        CmdPart2::RestAt(x, y)
                    }
                }
            }
        }
    }
}

fn simulation_part2(
    verti_rocks: &BTreeMap<i32, Vec<Rock>>,
    horiz_rocks: &BTreeMap<i32, Vec<Rock>>,
    highest_y: i32,
) -> usize {
    let mut sands: HashSet<Sand> = HashSet::new();

    let mut x = START_X;
    let mut y = START_Y;

    loop {
        match drop_sand_to_inf_floor(&verti_rocks, &horiz_rocks, &sands, highest_y, x, y) {
            CmdPart2::DropAgain(x_pos, y_pos) => {
                x = x_pos;
                y = y_pos;
            }
            CmdPart2::RestAt(x_pos, y_pos) => {
                sands.insert(Sand { x: x_pos, y: y_pos });
                x = START_X;
                y = START_Y;

                if x_pos == START_X && y_pos == START_Y {
                    break;
                }
            }
        }
    }

    return sands.len();
}

#[allow(dead_code, unused)]
fn print_sands(sands: &HashSet<Sand>) {
    println!("------------");
    for y in 0..10 {
        let mut line = vec![];
        for x in 493..504 {
            match sands.contains(&Sand { x, y }) {
                true => line.push('o'),
                false => line.push(' '),
            }
        }

        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    // Position (like X) -> Vecotr of Ranges
    let mut verti_rocks: BTreeMap<i32, Vec<Rock>> = BTreeMap::new();
    let mut horiz_rocks: BTreeMap<i32, Vec<Rock>> = BTreeMap::new();
    let mut highest_y = 0;

    while let Some(path) = read_path() {
        let mut x = None;
        let mut y = None;
        for pos in path {
            highest_y = std::cmp::max(highest_y, pos.1);

            match (x, y) {
                (None, None) => {
                    x = Some(pos.0);
                    y = Some(pos.1);
                }
                (Some(x_pos), Some(y_pos)) => {
                    if x_pos == pos.0 {
                        let rock = Rock::new(y_pos, pos.1);
                        let rocks = verti_rocks.entry(x_pos).or_default();
                        rocks.push(rock);

                        let rock = Rock::new(x_pos, x_pos);
                        let y_min = std::cmp::min(y_pos, pos.1);
                        let rocks = horiz_rocks.entry(y_min).or_default();
                        rocks.push(rock);
                    } else if y_pos == pos.1 {
                        let rock = Rock::new(x_pos, pos.0);
                        let rocks = horiz_rocks.entry(y_pos).or_default();
                        rocks.push(rock);
                    } else {
                        panic!("X != pos.0 or Y != pos.1");
                    }

                    x = Some(pos.0);
                    y = Some(pos.1);
                }
                (_, _) => panic!("Both X and Y should be initiated or set to None"),
            }
        }
    }

    let result1 = simulation_part1(&verti_rocks, &horiz_rocks);
    println!("{}", result1);

    let result2 = simulation_part2(&verti_rocks, &horiz_rocks, highest_y + 2);
    println!("{}", result2);
}
