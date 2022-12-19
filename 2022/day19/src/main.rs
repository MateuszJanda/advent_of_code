// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

struct Blueprint {
    id: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: (i32, i32),
    geode_robot: i32,
}

fn read_bluprint() -> Option<Blueprint> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let v = stripped_line
                .replace("Blueprint ", "")
                .replace(": Each ore robot costs ", " ")
                .replace(" ore. Each clay robot costs ", " ")
                .replace(" ore. Each obsidian robot costs ", " ")
                .replace(" ore and ", " ")
                .replace(" clay. Each geode robot costs ", " ")
                .replace(" obsidian.", "")
                .split(" ")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i32>>();

            Some(Blueprint {
                id: v[0],
                ore_robot: v[1],
                clay_robot: v[2],
                obsidian_robot: (v[3], v[4]),
                geode_robot: v[5],
            })
        }
    }
}

fn main() {
    while let Some(_) = read_bluprint() {}
}
