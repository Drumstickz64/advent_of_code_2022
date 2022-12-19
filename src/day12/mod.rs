use itertools::Itertools;
use petgraph::prelude::*;

use petgraph::algo::dijkstra;

use crate::common::string::alphabet_score;

pub fn solve_part_one(input: String) -> String {
    let mut graph = DiGraph::new();
    let mut start_node = NodeIndex::default();
    let mut finish_node = NodeIndex::default();
    let matrix: Vec<Vec<NodeIndex>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    let node = graph.add_node(ch);
                    match ch {
                        'S' => start_node = node,
                        'E' => finish_node = node,
                        _ => (),
                    }
                    node
                })
                .collect()
        })
        .collect();

    let h = matrix.len();
    let w = matrix[0].len();
    for (x, y) in (0..w).cartesian_product(0..h) {
        let node = matrix[y][x];
        let ch = graph[node];

        let mut adjacent_indeces = Vec::new();
        if x > 0 {
            adjacent_indeces.push((x - 1, y));
        }
        if y > 0 {
            adjacent_indeces.push((x, y - 1));
        }
        if x < w - 1 {
            adjacent_indeces.push((x + 1, y));
        }
        if y < h - 1 {
            adjacent_indeces.push((x, y + 1));
        }

        for (adj_x, adj_y) in adjacent_indeces {
            let adjacent_node = matrix[adj_y][adj_x];
            let adjacent_ch = graph[adjacent_node];
            if !graph.contains_edge(node, adjacent_node)
                && char_to_height(ch) + 1 >= char_to_height(adjacent_ch)
            {
                graph.add_edge(node, adjacent_node, ());
            }
        }
    }

    let distances = dijkstra(&graph, start_node, Some(finish_node), |_| 1);
    distances[&finish_node].to_string()
}

pub fn solve_part_two(input: String) -> String {
    let mut graph = DiGraph::new();
    let mut finish_node = NodeIndex::default();
    let matrix: Vec<Vec<NodeIndex>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    let node = graph.add_node(ch);
                    if ch == 'E' {
                        finish_node = node;
                    }
                    node
                })
                .collect()
        })
        .collect();

    let h = matrix.len();
    let w = matrix[0].len();
    for (x, y) in (0..w).cartesian_product(0..h) {
        let node = matrix[y][x];
        let ch = graph[node];

        let mut adjacent_indeces = Vec::new();
        if x > 0 {
            adjacent_indeces.push((x - 1, y));
        }
        if y > 0 {
            adjacent_indeces.push((x, y - 1));
        }
        if x < w - 1 {
            adjacent_indeces.push((x + 1, y));
        }
        if y < h - 1 {
            adjacent_indeces.push((x, y + 1));
        }

        for (adj_x, adj_y) in adjacent_indeces {
            let adjacent_node = matrix[adj_y][adj_x];
            let adjacent_ch = graph[adjacent_node];
            if !graph.contains_edge(node, adjacent_node)
                && char_to_height(ch) + 1 >= char_to_height(adjacent_ch)
            {
                graph.add_edge(node, adjacent_node, ());
            }
        }
    }

    graph
        .node_indices()
        .map(|node| (node, graph[node]))
        .filter(|(_, ch)| char_to_height(*ch) == 1)
        .filter_map(|(node, _)| {
            let distances = dijkstra(&graph, node, Some(finish_node), |_| 1);
            distances.get(&finish_node).copied()
        })
        .min()
        .unwrap()
        .to_string()
}

fn char_to_height(ch: char) -> u32 {
    match ch {
        'S' => 1,
        'E' => 26,
        ch => alphabet_score(ch),
    }
}
