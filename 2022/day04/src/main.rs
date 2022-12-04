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
    let mut counter1 = 0;
    let mut counter2 = 0;

    while let Some((a1, a2, b1, b2)) = read_nums() {
        if (b1 >= a1 && b2 <= a2) || (a1 >= b1 && a2 <= b2) {
            counter1 += 1;
        }

        let size_a = a2 - a1;
        let size_b = b2 - b1;
        let size_total = std::cmp::max(a2, b2) - std::cmp::min(a1, b1);
        if size_total <= size_a + size_b {
            counter2 += 1;
        }
    }

    println!("{}", counter1);
    println!("{}", counter2);
}
