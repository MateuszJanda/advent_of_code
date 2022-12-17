// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::cmp::Ordering;
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

#[derive(PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    fn new(val1: i32, val2: i32) -> Self {
        Range {
            begin: std::cmp::min(val1, val2),
            end: std::cmp::max(val1, val2),
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.begin.cmp(&other.begin)
    }
}

fn main() {
    let mut verti_obstacle: BTreeMap<Range, HashSet<i32>> = BTreeMap::new();
    let mut horiz_obstacle: BTreeMap<Range, HashSet<i32>> = BTreeMap::new();

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
                        let values = verti_obstacle.entry(range).or_default();
                        values.insert(x_pos);
                    } else if y_pos == pos.1 {
                        let range = Range::new(x_pos, pos.0);
                        let values = horiz_obstacle.entry(range).or_default();
                        values.insert(y_pos);
                    } else {
                        panic!("X != pos.0 or Y != pos.1");
                    }

                    x = Some(pos.0);
                    y = Some(pos.1);
                }
                (_, _) => panic!("Both X and Y should be initiated or None"),
            }
            println!("{}:{} ", pos.0, pos.1);
        }
    }
}
