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
    let mut result = 0;
    let mut window = VecDeque::new();

    let input = read_string();

    for (idx, ch) in input.chars().enumerate() {
        window.push_back(ch);

        if window.len() > 4 {
            window.pop_front();

            let letters: HashSet<char> = window.iter().cloned().collect();
            if letters.len() == 4 {
                result = idx + 1;
                break;
            }
        }
    }

    println!("{}", result);
}
