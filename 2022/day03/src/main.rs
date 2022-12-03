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
const NON_OF_ELVES: i32 = -1;

fn main() {
    let mut priorities = 0;
    let mut badge_priorities = 0;
    let mut elf_number = 0;
    let mut badge_tab = vec![NON_OF_ELVES; NUM_OF_CHARS];

    loop {
        match read_string() {
            None => break,
            Some(s) => {
                let mut char_tab = vec![false; NUM_OF_CHARS];

                for ch in s.as_bytes() {
                    if badge_tab[*ch as usize] == elf_number - 1 {
                        badge_tab[*ch as usize] = elf_number;
                    }
                }

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

                if elf_number == 2 {
                    for idx in 0..NUM_OF_CHARS {
                        if badge_tab[idx] == 2 {
                            badge_priorities += char_priority(idx as u8);
                        }
                    }

                    badge_tab = vec![NON_OF_ELVES; NUM_OF_CHARS];
                }

                elf_number = (elf_number + 1) % 3;
            }
        }
    }

    println!("{}", priorities);
    println!("{}", badge_priorities);
}
