// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

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
    while let Some((valve, rate, neighbors)) = read_valve() {
        println!("{} {} {:?}", valve, rate, neighbors);
    }
}
