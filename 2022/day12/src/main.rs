// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn read_string() -> Option<Vec<u8>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let v: Vec<u8> = stripped_line.chars().map(|ch| ch as u8).collect();
            return Some(v);
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn find_start_node_node(graph: &Vec<Vec<u8>>) -> Option<Position> {
    for (y, row) in graph.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'S' as u8 {
                return Some(Position { x, y });
            }
        }
    }

    None
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
struct Pair {
    node: Position,
    distance: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

// #algorithm: Dijkstra’s algorithm
fn main() {
    // Build graph
    let mut graph = vec![];
    while let Some(line) = read_string() {
        graph.push(line);
    }

    let start_node = find_start_node_node(&graph).unwrap();

    let width = graph[0].len();
    let height = graph.len();
    let mut distance = vec![vec![i32::MAX; width]; height];
    distance[start_node.y][start_node.x] = 0;

    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Reverse(Pair {
        node: start_node.clone(),
        distance: distance[start_node.y][start_node.x],
    }));

    let mut visited = vec![vec![false; width]; height];

    let mut out = vec![vec![' ' as u8; width]; height];

    let mut min_path_length = i32::MAX;
    while let Some(Reverse(pair)) = priority_queue.pop() {
        let node_a = pair.node;
        // println!("{} {}", graph[node_a.y][node_a.x] as char, priority_queue.len());

        if visited[node_a.y][node_a.x] {
            continue;
        }

        visited[node_a.y][node_a.x] = true;

        out[node_a.y][node_a.x] = graph[node_a.y][node_a.x];

        if graph[node_a.y][node_a.x] == 'E' as u8 {
            min_path_length = std::cmp::min(min_path_length, pair.distance);
            continue;
        }

        for (shift_y, shift_x) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            // Skip if out of border
            if (shift_y < 0 && node_a.y == 0) || (shift_x < 0 && node_a.x == 0) {
                continue;
            }

            let node_b = Position {
                x: (node_a.x as i32 + shift_x) as usize,
                y: (node_a.y as i32 + shift_y) as usize,
            };

            // Skip if out of border
            if node_b.y >= height || node_b.x >= width {
                continue;
            }

            // Skip if no edge between nodes
            let val_a = graph[node_a.y][node_a.x];
            let val_b = graph[node_b.y][node_b.x];
            if !(val_a == 'S' as u8 && val_b == 'a' as u8)
                && !(val_b <= val_a)
                && !(val_b == val_a + 1)
                && !(val_a == 'z' as u8 && val_b == 'E' as u8)
            {
                continue;
            }

            if distance[node_a.y][node_a.x] + 1 < distance[node_b.y][node_b.x] {
                distance[node_b.y][node_b.x] = distance[node_a.y][node_a.x] + 1;
                priority_queue.push(Reverse(Pair {
                    node: node_b.clone(),
                    distance: distance[node_b.y][node_b.x],
                }));
            }
        }
    }

    for line in out {
        println!(
            "{}",
            // line.iter().map(|val| val.to_string()).collect::<String>()
            String::from_utf8(line).unwrap()
        );
    }

    println!("{}", min_path_length);
}
