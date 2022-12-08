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

    fn score(&self, row: usize, col: usize, trees: &Vec<Vec<Tree>>) -> i32 {
        let val = trees[row][col].value;

        // Up
        let mut s1 = 0;
        for r in (0..row).rev() {
            s1 += 1;
            if val <= trees[r][col].value {
                break;
            }
        }

        // Left
        let mut s2 = 0;
        for c in (0..col).rev() {
            s2 += 1;
            if val <= trees[row][c].value {
                break;
            }
        }

        // Right
        let mut s3 = 0;
        for c in (col + 1)..trees[0].len() {
            s3 += 1;
            if val <= trees[row][c].value {
                break;
            }
        }

        // Down
        let mut s4 = 0;
        for r in (row + 1)..trees.len() {
            s4 += 1;
            if val <= trees[r][col].value {
                break;
            }
        }

        s1 * s2 * s3 * s4
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
    let mut best_score = 0;

    // Part 2: 99 * 99 * (98 + 98) == 1920996 ~= 1.9 * 10**6 steps, so brute force is fine
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            if trees[row][col].is_visible() {
                visible += 1;
            }

            best_score = std::cmp::max(best_score, trees[row][col].score(row, col, &trees));
        }
    }

    println!("{}", visible);
    println!("{}", best_score);
}
