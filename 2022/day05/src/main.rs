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
                match value {
                    '1' => return Some(vec![]),
                    ' ' => level.push(None),
                    _ => level.push(Some(value)),
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
    let mut stacsk1 = vec![VecDeque::<char>::new(); NUM_OF_STACKS];
    let mut stacks2 = vec![VecDeque::<char>::new(); NUM_OF_STACKS];

    while let Some(level) = read_levels() {
        if level.is_empty() {
            continue;
        }

        for (idx, ch) in level.iter().enumerate() {
            match ch {
                None => (),
                Some(c) => {
                    stacsk1[idx].push_front(*c);
                    stacks2[idx].push_front(*c);
                }
            }
        }
    }

    while let Some((ammount, from, to)) = read_moves() {
        let from = from - 1;
        let to = to - 1;

        let mut tmp_stack = VecDeque::new();
        for _ in 0..ammount {
            let val1 = stacsk1[from].pop_back().unwrap();
            stacsk1[to].push_back(val1);

            let val2 = stacks2[from].pop_back().unwrap();
            tmp_stack.push_front(val2);
        }
        stacks2[to].append(&mut tmp_stack);
    }

    let result1: String = stacsk1
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.back().unwrap())
        .collect();

    let result2: String = stacks2
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.back().unwrap())
        .collect();

    println!("{}", result1);
    println!("{}", result2);
}
