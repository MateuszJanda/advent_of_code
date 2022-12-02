// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn read_chars() -> Option<Vec<char>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let ch = stripped_line
                .split_ascii_whitespace()
                .map(|s| s.as_bytes()[0] as char)
                .collect::<Vec<_>>();
            Some(ch)
        }
    }
}

type Result<T> = std::result::Result<T, String>;

fn score_choice(ch2: char) -> Result<i32> {
    match ch2 {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err("Inccorect char".to_string()),
    }
}
fn score_result(ch1: char, ch2: char) -> i32 {
    match (ch1, ch2) {
        // Draw
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,

        // Win
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,

        // Lost
        _ => 0,
    }
}

fn main() {
    let mut result = 0;
    loop {
        match read_chars() {
            None => break,
            Some(v) => {
                let ch1 = v[0];
                let ch2 = v[1];

                result += score_choice(ch2).unwrap();
                result += score_result(ch1, ch2);
            }
        }
    }

    println!("{}", result);
}
