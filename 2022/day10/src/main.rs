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

fn signal_strength(cycle: i32, memory: &mut Vec<i32>) -> i32 {
    if (cycle - 20) % 40 == 0 {
        return cycle * memory.iter().sum::<i32>();
    }

    0
}

fn main() {
    let mut cycle = 0;
    let mut memory = vec![1];
    let mut result = 0;

    while let Some(instr) = read_instructions() {
        match instr {
            Instruction::Addx(val) => {
                cycle += 1;
                result += signal_strength(cycle, &mut memory);

                cycle += 1;
                result += signal_strength(cycle, &mut memory);

                memory.push(val);
            }
            Instruction::Noop => {
                cycle += 1;
                result += signal_strength(cycle, &mut memory);
            }
        }
    }

    println!("{}", result);
}
