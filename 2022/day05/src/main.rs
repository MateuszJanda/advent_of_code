// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::VecDeque;
use std::io;

const NUM_OF_STACKS: usize = 9;

fn read_levels() -> Option<Vec<Option<char>>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let mut level = vec![];
            for i in 0..NUM_OF_STACKS {
                let idx = i * 4 + 1;
                if idx >= stripped_line.as_bytes().len() {
                    level.push(None);
                    continue;
                }

                let value = stripped_line.as_bytes()[idx] as char;
                if value == '1' {
                    return Some(vec![]);
                }

                if value != ' ' {
                    level.push(Some(value))
                } else {
                    level.push(None);
                }
            }

            Some(level)
        }
    }
}

fn read_moves() -> Option<(usize, usize, usize)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let result = stripped_line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(' ')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            Some((result[0], result[1], result[2]))
        }
    }
}

fn main() {
    let mut stacks = vec![VecDeque::<char>::new(); NUM_OF_STACKS];

    while let Some(level) = read_levels() {
        if level.is_empty() {
            continue;
        }

        for (idx, ch) in level.iter().enumerate() {
            match ch {
                None => (),
                Some(c) => stacks[idx].push_front(*c),
            }
        }
    }

    while let Some((ammount, from, to)) = read_moves() {
        let from = from - 1;
        let to = to - 1;

        for _ in 0..ammount {
            let val = stacks[from].pop_back().unwrap();
            stacks[to].push_back(val);
        }
    }

    let mut result = String::new();
    for stack in &stacks {
        if !stack.is_empty() {
            result.push(*stack.back().unwrap())
        }
    }

    println!("{}", result);
}
