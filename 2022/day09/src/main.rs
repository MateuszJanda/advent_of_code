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

#[derive(Eq, Hash, PartialEq, Clone)]
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
}

fn move_right(
    head: &mut Position,
    tail: &mut Position,
    steps: i32,
    positions: &mut HashSet<Position>,
) {
    let mut old_head = Position::new(head.x, head.y);
    for _ in 0..steps {
        head.x += 1;

        if !head.is_close(&tail) {
            *tail = old_head;
            positions.insert(tail.clone());
        }

        old_head = head.clone();
    }
}

fn move_left(
    head: &mut Position,
    tail: &mut Position,
    steps: i32,
    positions: &mut HashSet<Position>,
) {
    let mut old_head = Position::new(head.x, head.y);
    for _ in 0..steps {
        head.x -= 1;

        if !head.is_close(&tail) {
            *tail = old_head;
            positions.insert(tail.clone());
        }

        old_head = head.clone();
    }
}

fn move_up(
    head: &mut Position,
    tail: &mut Position,
    steps: i32,
    positions: &mut HashSet<Position>,
) {
    let mut old_head = Position::new(head.x, head.y);
    for _ in 0..steps {
        head.y += 1;

        if !head.is_close(&tail) {
            *tail = old_head;
            positions.insert(tail.clone());
        }

        old_head = head.clone();
    }
}

fn move_down(
    head: &mut Position,
    tail: &mut Position,
    steps: i32,
    positions: &mut HashSet<Position>,
) {
    let mut old_head = Position::new(head.x, head.y);
    for _ in 0..steps {
        head.y -= 1;

        if !head.is_close(&tail) {
            *tail = old_head;
            positions.insert(tail.clone());
        }

        old_head = head.clone();
    }
}

fn main() {
    let mut positions = HashSet::<Position>::new();
    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);

    positions.insert(tail.clone());

    while let Some((dir, steps)) = read_motions() {
        match dir {
            'R' => move_right(&mut head, &mut tail, steps, &mut positions),
            'L' => move_left(&mut head, &mut tail, steps, &mut positions),
            'U' => move_up(&mut head, &mut tail, steps, &mut positions),
            'D' => move_down(&mut head, &mut tail, steps, &mut positions),
            _ => panic!("Unknown direction"),
        }
    }

    println!("{}", positions.len());
}
