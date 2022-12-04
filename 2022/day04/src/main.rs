// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn read_nums() -> Option<(i32, i32, i32, i32)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let nums = stripped_line
                .split(|c| c == ',' || c == '-')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Some((nums[0], nums[1], nums[2], nums[3]))
        }
    }
}

fn main() {
    let mut counter = 0;
    while let Some((a1, a2, b1, b2)) = read_nums() {
        if (b1 >= a1 && b2 <= a2) || (a1 >= b1 && a2 <= b2) {
            counter += 1;
        }
    }

    println!("{}", counter);
}
