// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

enum Instruction {
    Addx(i32),
    Noop,
}

fn read_instructions() -> Option<Instruction> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let words: Vec<String> = stripped_line
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect();

            match words[0].as_str() {
                "addx" => Some(Instruction::Addx(words[1].parse().unwrap())),
                "noop" => Some(Instruction::Noop),
                _ => None,
            }
        }
    }
}



fn main() {
    while let Some(instr) = read_instructions() {

    }
}
