// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;

fn read_valve() -> Option<(String, i32, Vec<String>)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let s = stripped_line
                .replace("valves", "valve")
                .replace("tunnels", "tunnel")
                .replace("leads", "lead")
                .replace(",", "")
                .replace("Valve ", "")
                .replace("has flow rate=", "")
                .replace("; tunnel lead to valve", "")
                .split(" ")
                .map(|chunk| chunk.to_string())
                .collect::<Vec<String>>();

            Some((
                s[0].clone(),
                s[1].parse().unwrap(),
                Vec::from_iter(s[2..].iter().cloned()),
            ))
        }
    }
}

fn main() {
    let mut neighborhood = HashMap::new();

    while let Some((valve, rate, adjacent)) = read_valve() {
        println!("{} {} {:?}", valve, rate, adjacent);

        neighborhood.insert(valve, adjacent);
    }

    for (v1, adj1) in neighborhood.iter() {
        for a1 in adj1.iter() {
            let mut bi_dir = false;
            for v2 in neighborhood[a1].iter() {
                if v1 == v2 {
                    bi_dir = true;
                    break;
                }
            }

            if !bi_dir {
                println!("No bidirectional {} {}", v1, a1);
            }
        }
    }

    println!("Ok")
}
