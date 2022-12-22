// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::clone;
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

        // println!("rates {}", valve);
        let rate = rates[&valve] * (TIME_LIMIT - t);
        // println!(" Rate {} {} {}", valve, rate, t);
        if !open.contains(&valve) && rate > best_rate {
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

// fn create_name_map(input: &<String, Vec<String>>) ->  HashMap<String, usize> {
//     let mut names = graph
//         .iter()
//         .map(|(name, _)| name.clone())
//         .collect::<Vec<String>>();
//     names.sort();

//     let mut names = graph
//         .iter()
//         .map(|(name, _)| name.clone())
//         .collect::<Vec<String>>();
//     names.sort();

//     let names_vec = names
//         .iter()
//         .enumerate()
//         .map(|(idx, name)| (name.clone(), idx))
//         .collect::<Vec<(String, usize)>>();
//     let names_map: HashMap<String, usize> = HashMap::from_iter(names_vec.into_iter());

//     names_map
// }

// #algorithm: Floyd–Warshall algorithm
// Time complexity O(n^3)
fn shortest_paths(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<i32>> {
    // let names_map = create_name_map(graph);

    // Init matrix
    let mut distance = vec![vec![i32::MAX; graph.len()]; graph.len()];
    for (name, adjacents) in graph {
        let idx1 = names_map[name];

        for adj in adjacents {
            let idx2 = names_map[adj];
            distance[idx1][idx2] = 1;
        }

        distance[idx1][idx1] = 0;
    }

    // Finding shortest paths
    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                distance[i][j] = std::cmp::min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }

    distance
}

const TIME_LIMIT: i32 = 30;

fn main() {
    let mut name_to_idx = HashMap::new();
    let mut graph = HashMap::new();
    let mut rates = HashMap::new();

    let mut idx = 0;
    while let Some((valve, rate, adjacent)) = read_valve() {
        name_to_idx.insert(valve, idx);
        graph.insert(idx, adjacent);
        rates.insert(idx, rate);

        idx += 1;
    }
    // let name_to_idx = create_name_map(intut_values);

    // map.insert("AA".to_string(), "BB".to_string());
    // map.insert("BB".to_string(), "CC".to_string());
    // map.insert("CC".to_string(), "DD".to_string());

    // let base = vec!["AA".to_string(), "BB".to_string(), "CC".to_string()];
    // let mut current = vec!["AA".to_string(), "BB".to_string(), "CC".to_string()];

    // current = permutation(map, base, current);
    // println!("{:?}", current);

    // let num_of_valves = graph.len();
    // println!("num_of_valves {}", open.len());
    // let mut start_valve = "AA".to_string();
    // let mut time_passed = 1;

    // // Greedy
    // while time_passed <= TIME_LIMIT && open.len() != num_of_valves {
    //     (start_valve, time_passed) = bfs(&start_valve, time_passed, &graph, &rates, &open);
    //     open.insert(start_valve.clone());

    //     println!("Open {} {}", start_valve, time_passed);
    // }
}
