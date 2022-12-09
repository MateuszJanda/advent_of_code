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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn is_close(&self, other: &Position) -> bool {
        let x_delta = (self.x - other.x).abs();
        let y_delta = (self.y - other.y).abs();

        (x_delta == 0 || x_delta == 1) && (y_delta == 0 || y_delta == 1)
    }

    fn is_on_cross(&self, other: &Position) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn is_vertical_gap(&self, other: &Position) -> bool {
        (self.y - other.y).abs() > 1
    }

    fn is_hirizontal_gap(&self, other: &Position) -> bool {
        (self.x - other.x).abs() > 1
    }
}

fn move_tail(head: &Position, tail: &mut Position) {
    if head.is_on_cross(&tail) && !head.is_close(&tail) {
        match head.x == tail.x {
            true => match head.y < tail.y {
                true => tail.y -= 1,
                false => tail.y += 1,
            },
            // Case when, head.y == tail.y
            false => match head.x < tail.x {
                true => tail.x -= 1,
                false => tail.x += 1,
            },
        }
    } else if head.is_vertical_gap(&tail) {
        match head.y < tail.y {
            true => tail.y -= 1,
            false => tail.y += 1,
        }

        tail.x = head.x
    } else if head.is_hirizontal_gap(&tail) {
        match head.x < tail.x {
            true => tail.x -= 1,
            false => tail.x += 1,
        }

        tail.y = head.y
    }
}

fn main() {
    let mut positions = HashSet::<Position>::new();
    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);

    positions.insert(tail.clone());

    while let Some((dir, steps)) = read_motions() {
        for _ in 0..steps {
            match dir {
                'R' => head.x += 1,
                'L' => head.x -= 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _ => panic!("Unknown direction"),
            }
            move_tail(&head, &mut tail);
            positions.insert(tail.clone());
        }
    }

    println!("{}", positions.len());
}
