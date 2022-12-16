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
    mut time: i32,
    graph: &HashMap<String, Vec<String>>,
    rates: &HashMap<String, i32>,
    visited: &HashSet<String>,
) -> (String, i32) {
    let mut queue = VecDeque::new();

    time -= 1;
    queue.push_back((start_valve.clone(), time));

    let mut best_time = 0;
    let mut best_rate = 0;
    let mut best_valve = "".to_string();

    while let Some((valve, t)) = queue.pop_front() {
        if visited.contains(&valve) || t - 1 == 0 {
            continue;
        }

        let rate = rates[&valve] * (t - 1);
        if rate > best_rate {
            best_rate = rate;
            best_time = t;
            best_valve = valve.clone();
        }

        for v in graph[&valve].iter() {
            queue.push_back((v.clone(), t - 1));
        }
    }

    (best_valve, best_time)
}

fn main() {
    let mut graph = HashMap::new();
    let mut rates = HashMap::new();
    let mut visited = HashSet::new();
    while let Some((valve, rate, adjacent)) = read_valve() {
        graph.insert(valve.clone(), adjacent);
        rates.insert(valve.clone(), rate);

        if rate == 0 {
            visited.insert(valve);
        }
    }

    let num_of_valves = graph.len();
    let mut start_valve = "AA".to_string();
    let mut time = 30;

    loop {
        (start_valve, time) = bfs(&start_valve, time, &graph, &rates, &visited);
        visited.insert(start_valve.clone());

        if time == 0 || visited.len() == num_of_valves {
            break;
        }
    }
}
