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

fn rate_element(ch2: char) -> Result<i32> {
    match ch2 {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err("Inccorect char".to_string()),
    }
}

fn rate_result(ch1: char, ch2: char) -> i32 {
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

fn choose_element(ch1: char, strategy: char) -> Result<char> {
    match strategy {
        // Lose
        'X' => match ch1 {
            'A' => Ok('Z'),
            'B' => Ok('X'),
            'C' => Ok('Y'),
            _ => Err("Inccorect char".to_string()),
        },

        // Draw
        'Y' => match ch1 {
            'A' => Ok('X'),
            'B' => Ok('Y'),
            'C' => Ok('Z'),
            _ => Err("Inccorect char".to_string()),
        },

        // Win
        'Z' => match ch1 {
            'A' => Ok('Y'),
            'B' => Ok('Z'),
            'C' => Ok('X'),
            _ => Err("Inccorect char".to_string()),
        },
        _ => Err("Inccorect char".to_string()),
    }
}

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;

    loop {
        match read_chars() {
            None => break,
            Some(v) => {
                let ch1 = v[0];
                let ch2 = v[1];

                result1 += rate_element(ch2).unwrap();
                result1 += rate_result(ch1, ch2);

                result2 += rate_element(choose_element(ch1, ch2).unwrap()).unwrap();
                result2 += rate_result(ch1, choose_element(ch1, ch2).unwrap());
            }
        }
    }

    println!("{}", result1);
    println!("{}", result2);
}
