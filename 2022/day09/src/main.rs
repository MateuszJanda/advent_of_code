// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::HashSet;
use std::io;

fn read_motions() -> Option<(char, i32)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let ch: Vec<String> = stripped_line
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect();

            Some((ch[0].as_bytes()[0] as char, ch[1].parse::<i32>().unwrap()))
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

fn main() {
    let mut positions = HashSet::<Position>::new();
    let mut head = Position::new(0, 0);

    while let Some((dir, steps)) = read_motions() {
        let p = Position::new(0, 0);
        positions.insert(p);
    }

    println!("{}", positions.len());
}
