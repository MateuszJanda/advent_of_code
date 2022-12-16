// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

fn bfs(
    start_valve: &String,
    mut time_passed: i32,
    graph: &HashMap<String, Vec<String>>,
    rates: &HashMap<String, i32>,
    open: &HashSet<String>,
) -> (String, i32) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_valve.clone(), time_passed));

    let mut best_rate = 0;
    let mut best_valve = "".to_string();

    while let Some((valve, t)) = queue.pop_front() {
        if t >= TIME_LIMIT {
            break;
        }

        let rate = rates[&valve] * (TIME_LIMIT - t);
        if !open.contains(&valve) && rate > best_rate {
            println!(" Rate {} {} {}", valve, rate, t);
            best_rate = rate;
            time_passed = t;
            best_valve = valve.clone();
        }

        visited.insert(valve.clone());

        for v in graph[&valve].iter() {
            if !visited.contains(v) {
                queue.push_back((v.clone(), t + 1));
            }
        }
    }

    (best_valve, time_passed)
}

const TIME_LIMIT: i32 = 30;

fn main() {
    let mut graph = HashMap::new();
    let mut rates = HashMap::new();
    let mut open = HashSet::new();
    while let Some((valve, rate, adjacent)) = read_valve() {
        graph.insert(valve.clone(), adjacent);
        rates.insert(valve.clone(), rate);

        if rate == 0 {
            open.insert(valve);
        }
    }

    let num_of_valves = graph.len();
    let mut start_valve = "AA".to_string();
    let mut time_passed = 1;

    // Greedy
    while time_passed <= TIME_LIMIT && open.len() != num_of_valves {
        (start_valve, time_passed) = bfs(&start_valve, time_passed, &graph, &rates, &open);
        open.insert(start_valve.clone());

        println!("Open {} {}", start_valve, time_passed);
    }
}
