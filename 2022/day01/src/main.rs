// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn main() {
    let mut calories = 0;
    let mut elf_calories = vec![];

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let stripped_line = match line.strip_suffix("\n") {
            None => {
                elf_calories.push(calories);
                break;
            }
            Some(s) => s,
        };
        if stripped_line.is_empty() {
            elf_calories.push(calories);
            calories = 0;
        } else {
            let num: i32 = stripped_line.parse().unwrap();
            calories += num;
        }
    }

    elf_calories.sort();
    elf_calories.reverse();

    let mut total_of_three = 0;
    for i in 0..3 {
        total_of_three += elf_calories[i];
    }

    // First solution
    println!("{}", elf_calories[0]);
    // Second solution
    println!("{}", total_of_three);
}
