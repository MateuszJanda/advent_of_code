// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn read_chars() -> Option<Vec<u8>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            let v: Vec<u8> = stripped_line.chars().map(|ch| ch as u8).collect();
            Some(v)
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

fn is_edge_up(graph: &Vec<Vec<u8>>, node_a: &Position, node_b: &Position) -> bool {
    let val_a = match graph[node_a.y][node_a.x] as char {
        'S' => 'a' as i8,
        'E' => 'z' as i8,
        _ => graph[node_a.y][node_a.x] as i8,
    };

    let val_b = match graph[node_b.y][node_b.x] as char {
        'S' => 'a' as i8,
        'E' => 'z' as i8,
        _ => graph[node_b.y][node_b.x] as i8,
    };

    (val_b - val_a) <= 1
}

fn is_edge_down(graph: &Vec<Vec<u8>>, node_a: &Position, node_b: &Position) -> bool {
    let val_a = match graph[node_a.y][node_a.x] as char {
        'S' => 'a' as i8,
        'E' => 'z' as i8,
        _ => graph[node_a.y][node_a.x] as i8,
    };

    let val_b = match graph[node_b.y][node_b.x] as char {
        'S' => 'a' as i8,
        'E' => 'z' as i8,
        _ => graph[node_b.y][node_b.x] as i8,
    };

    (val_a - val_b) <= 1
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

fn dijkstra(
    graph: &Vec<Vec<u8>>,
    start_char: char,
    end_char: char,
    is_edge: &dyn Fn(&Vec<Vec<u8>>, &Position, &Position) -> bool,
) -> i32 {
    let start_node = find_node(&graph, start_char).unwrap();

    let width = graph[0].len();
    let height = graph.len();
    let mut min_distance = i32::MAX;

    let mut distance = vec![vec![i32::MAX; width]; height];
    distance[start_node.y][start_node.x] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Pair {
        node: start_node.clone(),
        distance: distance[start_node.y][start_node.x],
    }));

    while let Some(Reverse(pair)) = priority_queue.pop() {
        let node_a = pair.node;

        if graph[node_a.y][node_a.x] == end_char as u8 {
            min_distance = std::cmp::min(min_distance, pair.distance);
            // We can't break if there is multiple correct nodes
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
                priority_queue.push(Reverse(Pair {
                    node: node_b.clone(),
                    distance: distance[node_b.y][node_b.x],
                }));
            }
        }
    }

    return min_distance;
}

// #algorithm: Dijkstra’s algorithm
fn main() {
    let mut graph = vec![];
    while let Some(line) = read_chars() {
        graph.push(line);
    }

    let min_distance1 = dijkstra(&graph, 'S', 'E', &is_edge_up);
    let min_distance2 = dijkstra(&graph, 'E', 'a', &is_edge_down);

    println!("{}", min_distance1);
    println!("{}", min_distance2);
}
