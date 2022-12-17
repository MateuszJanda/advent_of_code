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
struct Range {
    begin: i32,
    end: i32,
}

enum Dir {
    Forward,
    Backward,
}
impl Range {
    fn new(val1: i32, val2: i32) -> Self {
        Range {
            begin: std::cmp::min(val1, val2),
            end: std::cmp::max(val1, val2),
        }
    }

    fn is_contact(&self, pos: i32) -> bool {
        pos >= self.begin && pos <= self.end
    }
}

fn find_range_for_drop(
    obstacles: &BTreeMap<i32, Vec<Range>>,
    level: i32,
    dir: Dir,
    pos: i32,
) -> Option<&Range> {
    match dir {
        Dir::Backward => obstacles
            .range(..=level)
            .rev()
            .into_iter()
            .map(|(l, ranges)| (l, ranges))
            .flatten()
            .into_iter()
            .find(|range| range.is_contact(pos)),
        Dir::Forward => obstacles
            .range(level..)
            .into_iter()
            .map(|(_level, ranges)| ranges)
            .flatten()
            .into_iter()
            .find(|range| range.is_contact(pos)),
    }
}

fn main() {
    // X -> Vecotr of Ranges
    let mut verti_obstacles: BTreeMap<i32, Vec<Range>> = BTreeMap::new();
    // Y -> Vecotr of Ranges
    let mut horiz_obstacles: BTreeMap<i32, Vec<Range>> = BTreeMap::new();
    let mut sand_obstacle: HashSet<Range> = HashSet::new();

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
                        let range = Range::new(y_pos, pos.1);
                        let values = verti_obstacles.entry(x_pos).or_default();
                        values.push(range);
                    } else if y_pos == pos.1 {
                        let range = Range::new(x_pos, pos.0);
                        let values = horiz_obstacles.entry(y_pos).or_default();
                        values.push(range);
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

    // println!("{}", horiz_obstacles.len());
    // println!("{:?}", horiz_obstacles);

    if let Some(f) = find_range_for_drop(&horiz_obstacles, 0, Dir::Forward, 500) {
        println!("{:?}", f);
    } else {
        println!("false");
    }
}
