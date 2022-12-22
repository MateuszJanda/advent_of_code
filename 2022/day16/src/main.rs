// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::collections::HashMap;
use std::collections::HashSet;
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

const INF: i32 = 1_000_000_000 + 5;
const TIME_LIMIT: i32 = 30;
const START_NODE: &str = "AA";

fn main() {
    let mut name_to_idx = HashMap::new();
    let mut graph_with_names = HashMap::new();
    let mut rates_with_names = HashMap::new();

    let mut idx = 0;
    let mut start_node = None;
    while let Some((valve, rate, adjacent)) = read_valve() {
        name_to_idx.insert(valve.clone(), idx);
        graph_with_names.insert(valve.clone(), adjacent);
        rates_with_names.insert(valve.clone(), rate);

        if valve == START_NODE {
            start_node = Some(idx);
        }

        idx += 1;
    }

    let graph = graph_with_inedexes(&graph_with_names, &name_to_idx);
    let rates = rates_with_inedexes(&rates_with_names, &name_to_idx);

    let mut dfs = Dfs::new(&graph, rates);
    dfs.search(start_node.unwrap());
    println!("{}", dfs.best_result.unwrap());
}

struct Dfs {
    distance: Vec<Vec<i32>>,
    rates: HashMap<usize, i32>,
    visited: HashSet<usize>,
    time: i32,
    scores: Vec<i32>,
    best_result: Option<i32>,
}

impl Dfs {
    fn new(graph: &HashMap<usize, Vec<usize>>, rates: HashMap<usize, i32>) -> Self {
        let mut visited = HashSet::new();
        for (node, value) in rates.iter() {
            if *value == 0 {
                visited.insert(*node);
            }
        }

        let mut dfs = Dfs {
            distance: vec![],
            rates: rates,
            visited: visited,
            time: TIME_LIMIT,
            scores: vec![],
            best_result: None,
        };

        dfs.shortest_paths(graph);
        dfs
    }

    /// #algorithm: Floyd-Warshall algorithm
    /// Time complexity O(n^3)
    fn shortest_paths(&mut self, graph: &HashMap<usize, Vec<usize>>) {
        self.distance = vec![vec![INF; graph.len()]; graph.len()];

        // Init matrix
        for (idx1, adjacents) in graph {
            for idx2 in adjacents {
                self.distance[*idx1][*idx2] = 1;
            }

            self.distance[*idx1][*idx1] = 0;
        }

        // Finding shortest paths
        for k in 0..graph.len() {
            for i in 0..graph.len() {
                for j in 0..graph.len() {
                    self.distance[i][j] = std::cmp::min(
                        self.distance[i][j],
                        self.distance[i][k] + self.distance[k][j],
                    );
                }
            }
        }
    }

    /// Theoretical better score - happy path.
    fn is_better_score(
        &self,
        current_node: usize,
        current_score: i32,
        mut current_time: i32,
    ) -> bool {
        if self.best_result.is_none() {
            return true;
        }

        let mut nodes = vec![];
        for node in 0..self.distance.len() {
            if !self.visited.contains(&node) && node != current_node {
                nodes.push(node);
            }
        }

        let mut result = self.scores.iter().sum::<i32>() + current_score;
        nodes.sort_by(|a, b| self.rates[b].cmp(&self.rates[a]));
        for node in nodes.iter() {
            current_time -= 2;
            result += self.rates[&node] * current_time;
        }

        if result < self.best_result.unwrap() {
            false;
        }

        true
    }

    fn search(&mut self, prev_node: usize) {
        for node in 0..self.distance.len() {
            if self.visited.contains(&node) {
                continue;
            }

            let time_to_activate = self.distance[prev_node][node] + 1;
            if self.time - time_to_activate <= 0 {
                continue;
            }

            let score = self.rates[&node] * (self.time - time_to_activate);
            if !self.is_better_score(node, score, self.time - time_to_activate) {
                continue;
            }

            self.time -= time_to_activate;
            self.scores.push(score);
            self.visited.insert(node);

            self.search(node);

            self.visited.remove(&node);
            self.scores.pop();
            self.time += time_to_activate;
        }

        if self.visited.len() == self.distance.len() {
            self.best_result = match self.best_result {
                None => Some(self.scores.iter().sum()),
                Some(result) => Some(std::cmp::max(result, self.scores.iter().sum())),
            }
        }
    }
}
