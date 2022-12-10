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

fn signal_strength(cycle: i32, register: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        return cycle * register;
    }

    0
}

fn draw(cycle: i32, register: i32, buffer: &mut Vec<Vec<char>>) {
    // Cycle start from 1, and (x, y) position start from 0, so we must to subtract 1
    let y = (cycle - 1) / 40;
    let x = (cycle - 1) % 40;

    buffer[y as usize][x as usize] = match x == register - 1 || x == register || x == register + 1 {
        true => '#',
        false => '.',
    }
}

fn main() {
    let mut cycle = 0;
    let mut register = 1;
    let mut result = 0;
    let mut buffer = vec![vec!['.'; 40]; 6];

    while let Some(instr) = read_instructions() {
        match instr {
            Instruction::Addx(val) => {
                // Start first cycle
                cycle += 1;
                result += signal_strength(cycle, register);
                draw(cycle, register, &mut buffer);

                // Start second cycle
                cycle += 1;
                result += signal_strength(cycle, register);
                draw(cycle, register, &mut buffer);

                // Add in third cycle
                register += val;
            }
            Instruction::Noop => {
                cycle += 1;
                result += signal_strength(cycle, register);
                draw(cycle, register, &mut buffer);
            }
        }
    }

    println!("{}", result);

    for line in buffer {
        println!("{}", line.iter().collect::<String>());
    }
}
