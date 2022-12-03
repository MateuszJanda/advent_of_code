// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;


fn read_string() -> Option<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            Some(stripped_line.to_string())
        }
    }
}

fn char_priority(ch1: u8) -> i32 {
    if ch1.is_ascii_lowercase() {
        return (ch1 as u8 - 'a' as u8) as i32 + 1;
    }

    (ch1 as u8 - 'A' as u8) as i32 + 1 + 26
}

const NUM_OF_CHARS: usize = 256;

fn main() {
    let mut priorities = 0;

    loop {
        match read_string() {
            None => break,
            Some(s) => {
                let mut char_tab = vec![false; NUM_OF_CHARS];
                let half_size = s.as_bytes().len() / 2;

                for ch in &s.as_bytes()[..half_size] {
                    char_tab[*ch as usize] = true;
                }

                for ch in &s.as_bytes()[half_size..] {
                    if char_tab[*ch as usize] {
                        priorities += char_priority(*ch);
                        break;
                    }
                }
            }
        }
    }

    println!("{}", priorities);
}
