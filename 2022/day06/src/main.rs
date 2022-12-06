// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn read_string() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.strip_suffix("\n").unwrap().to_string();
}

fn main() {
    let mut result1 = None;
    let mut result2 = None;
    let mut window1 = VecDeque::new();
    let mut window2 = VecDeque::new();

    let input = read_string();

    for (idx, ch) in input.chars().enumerate() {
        window1.push_back(ch);
        window2.push_back(ch);

        if result1.is_none() && window1.len() > 4 {
            window1.pop_front();

            let letters: HashSet<char> = window1.iter().cloned().collect();
            if letters.len() == 4 {
                result1 = Some(idx + 1);
            }
        }

        if result2.is_none() && window2.len() > 14 {
            window2.pop_front();

            let letters: HashSet<char> = window2.iter().cloned().collect();
            if result2.is_none() && letters.len() == 14 {
                result2 = Some(idx + 1);
            }
        }
    }

    println!("{}", result1.unwrap());
    println!("{}", result2.unwrap());
}
