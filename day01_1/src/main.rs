// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn main() {
    let mut max_calories = 0;
    let mut efl_calories = 0;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let stripped_line = match line.strip_suffix("\n") {
            None => break,
            Some(s) => s,
        };
        if stripped_line.is_empty() {
            max_calories = std::cmp::max(max_calories, efl_calories);
            efl_calories = 0;
        } else {
            let num: i32 = stripped_line.parse().unwrap();
            efl_calories += num;
        }
    }

    println!("Max calories: {}", max_calories);
}
