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

fn item_priority(ch: u8) -> i32 {
    if ch.is_ascii_lowercase() {
        return (ch as u8 - 'a' as u8) as i32 + 1;
    }

    (ch as u8 - 'A' as u8) as i32 + 1 + 26
}

const NUM_OF_ITEMS: usize = 256;
const NON_OF_ELVES: i32 = -1;
const LAST_ELF: i32 = 2;

fn main() {
    let mut items_sum = 0;
    let mut badges_sum = 0;
    let mut elf_number = 0;
    let mut badge_tab = vec![NON_OF_ELVES; NUM_OF_ITEMS];

    while let Some(rucksack) = read_string() {
        let mut item_tab = vec![false; NUM_OF_ITEMS];
        let mut item_found = false;
        let half_size = rucksack.as_bytes().len() / 2;

        for (idx, ch) in rucksack.as_bytes().iter().enumerate() {
            if badge_tab[*ch as usize] == elf_number - 1 {
                badge_tab[*ch as usize] = elf_number;
            }

            if idx < half_size {
                item_tab[*ch as usize] = true;
            } else if idx >= half_size && item_tab[*ch as usize] && item_found == false {
                items_sum += item_priority(*ch);
                item_found = true;
            }
        }

        if elf_number == LAST_ELF {
            for idx in 0..badge_tab.len() {
                if badge_tab[idx] == LAST_ELF {
                    badges_sum += item_priority(idx as u8);
                    break;
                }
            }

            badge_tab = vec![NON_OF_ELVES; NUM_OF_ITEMS];
        }

        elf_number = (elf_number + 1) % 3;
    }

    println!("{}", items_sum);
    println!("{}", badges_sum);
}
