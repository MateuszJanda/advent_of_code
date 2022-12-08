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
    let mut trees = vec![];
    let mut mark = vec![];
    let mut visible = 0;

    let mut height_top = vec![];
    let mut height_left = vec![];

    let mut row = 0;
    while let Some(nums) = read_nums() {
        mark.push(vec![false; nums.len()]);

        for (col, val) in nums.iter().enumerate() {
            if row == 0 {
                height_top.push(-1);
                visible += 1;
                mark[row][col] = true;
            } else if *val > height_top[col] {
                visible += 1;
                mark[row][col] = true;
            }

            if col == 0 {
                height_left.push(-1);
                if mark[row][col] == false {
                    visible += 1;
                    mark[row][col] = true;
                }
            } else if mark[row][col] == false && *val > height_top[row] {
                visible += 1;
                mark[row][col] = true;
            }

            height_top[col] = std::cmp::max(height_top[col], *val);
            height_left[row] = std::cmp::max(height_left[row], *val);
        }

        trees.push(nums);
        row += 1;
    }

    let mut height_right = vec![-1; trees.len()];
    let mut height_bottom = vec![-1; trees[0].len()];

    for row in (0..trees.len()).rev() {
        for col in (0..trees[0].len()).rev() {
            let val = trees[row][col];

            if mark[row][col] == false && val > height_right[row] {
                visible += 1;
                mark[row][col] = true;
            } else if mark[row][col] == false && val > height_bottom[col] {
                visible += 1;
                mark[row][col] = true;
            }

            height_bottom[col] = std::cmp::max(height_bottom[col], val);
            height_right[row] = std::cmp::max(height_right[row], val);
        }
    }

    println!("{}", visible);
}
