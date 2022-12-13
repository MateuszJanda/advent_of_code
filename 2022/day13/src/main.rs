// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::cmp::Ordering;
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

fn read_packets() -> Option<(String, String)> {
    let left = read_string()?;
    let right = read_string()?;

    // Read blank line
    read_string();

    Some((left, right))
}

fn split_packet(packet: &String) -> Vec<String> {
    let mut bracket_counter = 0;
    let mut start_value = None;
    let mut result1 = vec![];

    for (idx, ch) in packet.chars().enumerate() {
        match ch {
            '[' => {
                bracket_counter += 1;
                if bracket_counter > 1 && start_value.is_none() {
                    start_value = Some(idx);
                }
            }
            ']' => {
                if bracket_counter == 1 {
                    match start_value {
                        Some(start) => result1.push(packet[start..idx].to_string()),
                        None => result1.push("".to_string()),
                    };
                    start_value = None;
                }
                bracket_counter -= 1;
            }
            ',' => {
                if bracket_counter == 1 {
                    result1.push(packet[start_value.unwrap()..idx].to_string());
                    start_value = None;
                }
            }
            '0'..='9' => {
                if bracket_counter == 1 && start_value.is_none() {
                    start_value = Some(idx);
                }
            }
            _ => panic!("Unknown character"),
        }
    }

    result1
}

fn check_order(left: &String, right: &String) -> Ordering {
    let left_vec = split_packet(left);
    let right_vec = split_packet(right);

    let it = left_vec.iter().zip(right_vec.iter());
    for (l_str, r_str) in it {
        let result = match (l_str.parse::<i32>(), r_str.parse::<i32>()) {
            (Ok(l_val), Ok(r_val)) => l_val.cmp(&r_val),
            (Ok(l_val), Err(_)) => check_order(&("[".to_owned() + &l_val.to_string() + "]"), r_str),
            (Err(_), Ok(r_val)) => check_order(l_str, &("[".to_owned() + &r_val.to_string() + "]")),
            (Err(_), Err(_)) => check_order(l_str, r_str),
        };

        match result {
            Ordering::Equal => (),
            cmp => return cmp,
        };
    }

    left_vec.len().cmp(&right_vec.len())
}

fn main() {
    let mut all_packets = vec![];

    // Part 1
    let mut counter = 1;
    let mut result1 = 0;

    while let Some((left, right)) = read_packets() {
        match check_order(&left, &right) {
            Ordering::Less => result1 += counter,
            _ => (),
        }
        counter += 1;

        all_packets.push(left);
        all_packets.push(right)
    }

    // Part 2
    let divider1 = "[[2]]".to_string();
    let divider2 = "[[6]]".to_string();
    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());
    all_packets.sort_by(|a, b| check_order(a, b));

    let mut result2 = 1;
    for (idx, packet) in all_packets.iter().enumerate() {
        if packet == &divider1 || packet == &divider2 {
            result2 *= idx + 1;
        }
    }

    println!("{}", result1);
    println!("{}", result2);
}
