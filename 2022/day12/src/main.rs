// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
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

fn find_node(graph: &Vec<Vec<u8>>, name: char) -> Option<Position> {
    for (y, row) in graph.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == name as u8 {
                return Some(Position { x, y });
            }
        }
    }

    None
}

fn is_edge(graph: &Vec<Vec<u8>>, node_a: &Position, node_b: &Position) -> bool {
    let val_a = graph[node_a.y][node_a.x];
    let val_b = graph[node_b.y][node_b.x];
    return (val_a == 'S' as u8 && val_b == 'a' as u8)
        || (val_b.is_ascii_lowercase() && val_b <= val_a)
        || (val_b == val_a + 1)
        || (val_a == 'z' as u8 && val_b == 'E' as u8)
}

fn build_node(
    graph: &Vec<Vec<u8>>,
    node: &Position,
    shift_y: i32,
    shift_x: i32,
) -> Option<Position> {
    let width = graph[0].len() as i32;
    let height = graph.len() as i32;

    let y = node.y as i32;
    let x = node.x as i32;

    if y + shift_y < 0 || x + shift_x < 0 || y + shift_y >= height || x + shift_x >= width {
        return None;
    }

    Some(Position {
        x: (x + shift_x) as usize,
        y: (y + shift_y) as usize,
    })
}

fn dijkstra(graph: &Vec<Vec<u8>>) -> i32 {
    let start_node = find_node(&graph, 'S').unwrap();

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
    let mut parent = vec![vec![Position { x: 0, y: 0 }; width]; height];

    let mut min_path_length = i32::MAX;
    while let Some(Reverse(pair)) = priority_queue.pop() {
        let node_a = pair.node;

        if visited[node_a.y][node_a.x] {
            continue;
        }

        visited[node_a.y][node_a.x] = true;

        out[node_a.y][node_a.x] = graph[node_a.y][node_a.x];

        if graph[node_a.y][node_a.x] == 'E' as u8 {
            min_path_length = pair.distance;
            break;
        }

        for (shift_y, shift_x) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            // Skip if out of border
            let node_b = match build_node(&graph, &node_a, shift_y, shift_x) {
                Some(n) => n,
                None => continue,
            };

            // Skip if no edge between nodes
            if !is_edge(&graph, &node_a, &node_b) {
                continue;
            }

            if distance[node_a.y][node_a.x] + 1 < distance[node_b.y][node_b.x] {
                distance[node_b.y][node_b.x] = distance[node_a.y][node_a.x] + 1;
                parent[node_b.y][node_b.x] = node_a.clone();
                priority_queue.push(Reverse(Pair {
                    node: node_b.clone(),
                    distance: distance[node_b.y][node_b.x],
                }));
            }
        }
    }

    let mut out = vec![vec![' ' as u8; width]; height];
    let mut n = find_node(graph, 'E').unwrap();
    loop {
        out[n.y][n.x] = graph[n.y][n.x];

        if graph[n.y][n.x] == 'S' as u8 {
            break;
        }

        n = parent[n.y][n.x].clone();
    }

    for line in out {
        println!(
            "{}",
            // line.iter().map(|val| val.to_string()).collect::<String>()
            String::from_utf8(line).unwrap()
        );
    }

    return min_path_length;
}

fn bfs(graph: &Vec<Vec<u8>>) -> i32 {
    let start_node = find_node(graph, 'S').unwrap();
    let mut queue = VecDeque::new();

    let width = graph[0].len();
    let height = graph.len();

    let mut distance = vec![vec![i32::MAX; width]; height];

    queue.push_back(Pair {
        node: start_node.clone(),
        distance: 0,
    });

    while let Some(pair) = queue.pop_front() {
        let node_a = pair.node;

        if pair.distance > distance[node_a.y][node_a.x] {
            continue;
        }
        distance[node_a.y][node_a.x] = pair.distance;

        for (shift_y, shift_x) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let node_b = match build_node(&graph, &node_a, shift_y, shift_x) {
                Some(n) => n,
                None => continue,
            };

            if !is_edge(&graph, &node_a, &node_b) {
                continue;
            }

            queue.push_back(Pair {
                node: node_b,
                distance: pair.distance + 1,
            });
        }
    }

    let end_node = find_node(graph, 'E').unwrap();
    distance[end_node.y][end_node.x]
}

// #algorithm: Dijkstra’s algorithm
fn main() {
    // Build graph
    let mut graph = vec![];
    while let Some(line) = read_string() {
        graph.push(line);
    }

    // let min_path_length = dijkstra(&graph);
    let min_path_length = bfs(&graph);

    println!("{}", min_path_length);
}
