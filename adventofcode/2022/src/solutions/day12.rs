use std::str;

use petgraph::{algo::astar, data::Build, prelude::DiGraphMap, visit::EdgeRef};

pub const EXAMPLE_ANSWER1: &str = "31";
pub const EXAMPLE_ANSWER2: &str = "29";

fn build_graph(
    data: &Vec<Vec<char>>,
) -> (
    DiGraphMap<(usize, usize), usize>,
    (usize, usize),
    (usize, usize),
) {
    fn char_to_height(c: char) -> u32 {
        let mut answer = c.to_digit(36).unwrap();
        if c == 'S' {
            answer = 'a'.to_digit(36).unwrap();
        } else if c == 'E' {
            answer = 'z'.to_digit(36).unwrap();
        }
        answer
    }

    let mut graph = DiGraphMap::new();
    let line_number = data.len();
    let col_number = data[0].len();
    let mut start_node = (0, 0);
    let mut end_node = (0, 0);
    for i in 0..line_number {
        for j in 0..col_number {
            let node = (i, j);
            let value = data[i][j];
            let height = char_to_height(value);
            if value == 'S' {
                start_node = (i, j);
            } else if value == 'E' {
                end_node = (i, j);
            }
            if i < line_number - 1 {
                let right_node = (i + 1, j);
                let right_height = char_to_height(data[i + 1][j]);
                if right_height < height + 2 {
                    graph.update_edge(node, right_node, 1);
                }
                if height < right_height + 2 {
                    graph.update_edge(right_node, node, 1);
                }
            }
            if j < col_number - 1 {
                let down_node = (i, j + 1);
                let down_height = char_to_height(data[i][j + 1]);
                if down_height < height + 2 {
                    graph.update_edge(node, down_node, 1);
                }
                if height < down_height + 2 {
                    graph.update_edge(down_node, node, 1);
                }
            }
        }
    }
    (graph, start_node, end_node)
}

pub fn day12_1(input: String) -> String {
    let mut data = Vec::<Vec<char>>::new();
    for line in input.lines() {
        data.push(line.chars().collect());
    }
    let (graph, start, end) = build_graph(&data);
    let answer = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap()
    .0;
    answer.to_string()
}

pub fn day12_2(input: String) -> String {
    let mut data = Vec::<Vec<char>>::new();
    for line in input.lines() {
        data.push(line.chars().collect());
    }
    let (graph, start, end) = build_graph(&data);
    let mut answer = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap()
    .0;

    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'a' {
                let new_possible_distance = astar(
                    &graph,
                    (i, j),
                    |finish| finish == end,
                    |e| *e.weight(),
                    |_| 0,
                );
                if new_possible_distance.is_some() {
                    let new_distance = new_possible_distance.unwrap().0;
                    if new_distance < answer {
                        answer = new_distance;
                    }
                }
            }
        }
    }
    answer.to_string()
}
