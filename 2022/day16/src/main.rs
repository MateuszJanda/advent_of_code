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
    graph_with_names: &HashMap<String, Vec<String>>,
    rates_with_names: &HashMap<String, i32>,
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

        // println!("rates_with_names {}", valve);
        let rate = rates_with_names[&valve] * (TIME_LIMIT - t);
        // println!(" Rate {} {} {}", valve, rate, t);
        if !open.contains(&valve) && rate > best_rate {
            best_rate = rate;
            time_passed = t;
            best_valve = valve.clone();
        }

        visited.insert(valve.clone());

        for v in graph_with_names[&valve].iter() {
            if !visited.contains(v) {
                queue.push_back((v.clone(), t + 1));
            }
        }
    }

    (best_valve, time_passed)
}

fn graph_with_inedexes(
    graph_with_names: &HashMap<String, Vec<String>>,
    name_to_idx: &HashMap<String, usize>,
) -> HashMap<usize, Vec<usize>> {
    graph_with_names
        .iter()
        .map(|(name, neighbours)| {
            (
                name_to_idx[name],
                neighbours
                    .iter()
                    .map(|neighbour| name_to_idx[neighbour])
                    .collect(),
            )
        })
        .collect()
}

fn rates_with_inedexes(
    rates_with_names: &HashMap<String, i32>,
    name_to_idx: &HashMap<String, usize>,
) -> HashMap<usize, i32> {
    rates_with_names
        .iter()
        .map(|(name, rate)| (name_to_idx[name], *rate))
        .collect()
}

// #algorithm: Floyd–Warshall algorithm
// Time complexity O(n^3)
fn shortest_paths(graph_with_names: &HashMap<usize, Vec<usize>>) -> Vec<Vec<i32>> {
    // Init matrix
    let mut distance = vec![vec![i32::MAX; graph_with_names.len()]; graph_with_names.len()];
    for (idx1, adjacents) in graph_with_names {
        for idx2 in adjacents {
            distance[*idx1][*idx2] = 1;
        }

        distance[*idx1][*idx1] = 0;
    }

    // Finding shortest paths
    for k in 0..graph_with_names.len() {
        for i in 0..graph_with_names.len() {
            for j in 0..graph_with_names.len() {
                distance[i][j] = std::cmp::min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }

    distance
}

const TIME_LIMIT: i32 = 30;

fn main() {
    let mut name_to_idx = HashMap::new();
    let mut graph_with_names = HashMap::new();
    let mut rates_with_names = HashMap::new();

    let mut idx = 0;
    while let Some((valve, rate, adjacent)) = read_valve() {
        name_to_idx.insert(valve.clone(), idx);
        graph_with_names.insert(valve.clone(), adjacent);
        rates_with_names.insert(valve, rate);

        idx += 1;
    }

    let graph = graph_with_inedexes(&graph_with_names, &name_to_idx);
    let rates = rates_with_inedexes(&rates_with_names, &name_to_idx);
}
