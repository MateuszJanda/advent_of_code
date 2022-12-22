// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

#[derive(Clone, Debug)]
struct Blueprint {
    id: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: i32,
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
                ore_robot_cost: v[1],
                clay_robot_cost: v[2],
                obsidian_robot_cost: (v[3], v[4]),
                geode_robot_cost: v[5],
            })
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    ore_robots: i32,
    ore: i32,
}

impl State {
    fn new(ore_robots: i32, ore: i32) -> Self {
        State { ore_robots, ore }
    }
}

const MINUTES: usize = 24;

fn main() {
    while let Some(bluprint) = read_bluprint() {
        let mut dp = vec![];
        let empty_row = vec![State::new(0, 0); MINUTES + 1];
        dp.push(empty_row);

        let mut row = vec![State::new(1, 0)];
        for idx in 1..=MINUTES {
            row.push(State {
                ore_robots: 1,
                ore: row[idx - 1].ore + 1,
            });
        }
        dp.push(row);

        // println!("{:?}\n\n", dp);

        for y in 2..=MINUTES {
            let mut row = vec![State::new(1, 0)];
            let mut buy_ore_robot = true;
            for x in 1..=MINUTES {

                match buy_ore_robot {
                    true => {

                        if dp[y-1][x-1].ore >= bluprint.ore_robot_cost

                        false
                    },
                    false =>
                    {
                        let ore = dp[y][x - 1].ore + dp[y][x - 1].ore_robots;
                        let ore_robots = dp[y][x - 1].ore_robots;
                    }
                }

                ore_robots = dp[y - 1][x] + dp[y - 1][x]
                row[x] = State { ore_robots: row }
            }
            dp.push(row);
        }
    }
}
