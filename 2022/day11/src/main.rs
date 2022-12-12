// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn read_string() -> Option<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            Some(stripped_line.to_string())
        }
    }
}

fn read_monkey_number() -> Option<usize> {
    Some(
        read_string()?
            .replace("Monkey ", "")
            .replace(":", "")
            .parse::<usize>()
            .unwrap(),
    )
}

fn read_items() -> Option<Vec<i32>> {
    Some(
        read_string()?
            .replace("  Starting items: ", "")
            .split(", ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
}

fn read_operation() -> Option<(char, String)> {
    let words = read_string()?
        .replace("  Operation: new = old ", "")
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Some((words[0].as_bytes()[0] as char, words[1].clone()))
}

fn read_test() -> Option<i32> {
    Some(
        read_string()?
            .replace("  Test: divisible by ", "")
            .parse::<i32>()
            .unwrap(),
    )
}

fn read_monkey_num_true() -> Option<usize> {
    Some(
        read_string()?
            .replace("    If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap(),
    )
}

fn read_monkey_num_false() -> Option<usize> {
    Some(
        read_string()?
            .replace("    If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap(),
    )
}

#[derive(Clone, Debug)]
struct Monkey {
    _num: usize,
    items: Vec<i32>,
    operator: char,
    operator_val: String,
    test: i32,
    monkey_num_true: usize,
    monkey_num_false: usize,
}

fn read_monkey() -> Option<Monkey> {
    let _num = read_monkey_number()?;
    let items = read_items()?;
    let (operator, operator_val) = read_operation()?;
    let test = read_test()?;
    let monkey_num_true = read_monkey_num_true()?;
    let monkey_num_false = read_monkey_num_false()?;

    // Read empty line
    read_string();

    Some(Monkey {
        _num,
        items,
        operator,
        operator_val,
        test,
        monkey_num_true,
        monkey_num_false,
    })
}

fn get_value(operator_val: &String, old_value: &i32) -> i32 {
    match operator_val.as_str() {
        "old" => *old_value,
        _ => operator_val.parse::<i32>().unwrap(),
    }
}

const NUMBER_OF_ROUNDS: i32 = 20;

fn main() {
    let mut monkeys_curr = vec![];
    let mut monkey_business = vec![];

    while let Some(monkey) = read_monkey() {
        monkeys_curr.push(monkey);
        monkey_business.push(0);
    }

    for _ in 0..NUMBER_OF_ROUNDS {
        let mut monkeys_next = monkeys_curr.clone();
        for num in 0..monkeys_curr.len() {
            monkeys_next[num].items.clear();

            for idx in 0..monkeys_curr[num].items.len() {
                monkey_business[num] += 1;

                let old_value = monkeys_curr[num].items[idx];
                let mut new_value = match monkeys_curr[num].operator {
                    '+' => old_value + get_value(&monkeys_curr[num].operator_val, &old_value),
                    '*' => old_value * get_value(&monkeys_curr[num].operator_val, &old_value),
                    _ => panic!("Unsupported operator."),
                };

                new_value = new_value / 3;

                let monkey_num_true = monkeys_curr[num].monkey_num_true;
                let monkey_num_false = monkeys_curr[num].monkey_num_false;

                match new_value % monkeys_curr[num].test == 0 {
                    true => match monkey_num_true <= num {
                        true => monkeys_next[monkey_num_true].items.push(new_value),
                        false => monkeys_curr[monkey_num_true].items.push(new_value),
                    },
                    false => match monkey_num_false <= num {
                        true => monkeys_next[monkey_num_false].items.push(new_value),
                        false => monkeys_curr[monkey_num_false].items.push(new_value),
                    },
                }
            }
        }

        monkeys_curr = monkeys_next;
    }

    monkey_business.sort_by(|a, b| b.cmp(a));
    println!("{}", monkey_business[0] * monkey_business[1]);
}
