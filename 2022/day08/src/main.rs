// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn read_nums() -> Option<Vec<i32>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let result: Vec<i32> = stripped_line
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect();
            Some(result)
        }
    }
}

fn main() {
    println!("Hello world!");

    let mut height_top = vec![];
    let mut height_left = vec![];
    let mut row = 0;
    while let Some(nums) = read_nums() {
        for (col, val) in nums.iter().enumerate() {
            if row == 0 {
                height_top.push(val);
            } else {
                height_top[col] = std::cmp::max(height_top[col], val);
            }

            if col == 0 {
                height_left.push(val);
            } else {
                height_left[row] = std::cmp::max(height_left[col], val);
            }
        }

        row += 1;
    }
}
