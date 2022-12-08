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

struct Tree {
    value: i32,
    height_top: i32,
    height_bottom: i32,
    height_right: i32,
    height_left: i32,
}

impl Tree {
    fn new(value: i32) -> Self {
        Tree {
            value,
            height_top: -1,
            height_bottom: -1,
            height_right: -1,
            height_left: -1,
        }
    }

    fn is_visible(&self) -> bool {
        self.value > self.height_top
            || self.value > self.height_bottom
            || self.value > self.height_right
            || self.value > self.height_left
    }
}

fn main() {
    let mut trees = vec![];

    let mut height_top = vec![];
    let mut height_left = vec![];

    let mut row = 0;
    while let Some(nums) = read_nums() {
        let mut tree_row = vec![];
        for (col, val) in nums.iter().enumerate() {
            if row == 0 {
                height_top.push(-1);
            }

            if col == 0 {
                height_left.push(-1);
            }

            let mut tree = Tree::new(*val);
            tree.height_top = height_top[col];
            tree.height_left = height_left[row];
            tree_row.push(tree);

            height_top[col] = std::cmp::max(height_top[col], *val);
            height_left[row] = std::cmp::max(height_left[row], *val);
        }

        trees.push(tree_row);
        row += 1;
    }

    let mut height_right = vec![-1; trees.len()];
    let mut height_bottom = vec![-1; trees[0].len()];

    for row in (0..trees.len()).rev() {
        for col in (0..trees[0].len()).rev() {
            let mut tree = &mut trees[row][col];

            tree.height_bottom = height_bottom[col];
            tree.height_right = height_right[row];

            height_bottom[col] = std::cmp::max(height_bottom[col], tree.value);
            height_right[row] = std::cmp::max(height_right[row], tree.value);
        }
    }

    let mut visible = 0;
    for row in (0..trees.len()).rev() {
        for col in (0..trees[0].len()).rev() {
            if trees[row][col].is_visible() {
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}


