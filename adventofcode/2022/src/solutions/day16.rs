use rand::Rng;
use std::{collections::HashMap, str};

use petgraph::{algo::floyd_warshall, data::Build, prelude::UnGraphMap};

use regex::Regex;
pub const EXAMPLE_ANSWER1: &str = "1651";
pub const EXAMPLE_ANSWER2: &str = "1707";

pub fn day16_1(input: String) -> String {
    day16(input, 1, 30)
}

pub fn day16_2(input: String) -> String {
    day16(input, 2, 26)
}

fn day16(input: String, way_numbers: usize, given_time: usize) -> String {
    let valves = input_to_valves(input);
    let graph = valves_to_graph(&valves);
    let mut valves_with_positive_flow: HashMap<&String, usize> = HashMap::new();

    valves.iter().filter(|v| v.1 > 0).for_each(|v| {
        valves_with_positive_flow.insert(&v.0, v.1);
    });

    annealing(&graph, &valves_with_positive_flow, way_numbers, given_time).to_string()
}

// annealing is great
fn annealing(
    graph: &UnGraphMap<&str, usize>,
    valves: &HashMap<&String, usize>,
    way_numbers: usize,
    given_time: usize,
) -> usize {
    let mut first_way = vec!["AA".to_string()];
    valves
        .keys()
        .filter(|k| k.as_str() != "AA")
        .for_each(|k| first_way.push(k.to_string()));

    let mut init_ways = vec![first_way];
    for _n in 1..way_numbers {
        init_ways.push(vec!["AA".to_string()]);
    }

    let mut old_way = init_ways;
    let mut new_way = swap_valves(&old_way);
    let mut temperature: f64 = 500.0;
    let mut max = 0;

    while temperature > 0.01 {
        for _i in 0..500 {
            let pressure_old = pressure(&graph, &old_way, valves, given_time);
            let pressure_new = pressure(&graph, &new_way, valves, given_time);
            max = std::cmp::max(max, pressure_old);
            if pressure_old < pressure_new {
                old_way = new_way;
            } else {
                let r: f64 = rand::thread_rng().gen();
                let exp: f64 =
                    ((pressure_new as f64 - pressure_old as f64) / temperature as f64).exp();
                if r < exp {
                    old_way = new_way;
                }
            }
            new_way = swap_valves(&old_way);
        }
        if temperature > 1.0 {
            temperature -= 1 as f64;
        } else {
            temperature = temperature / 1.1 as f64;
        }
    }
    max
}

fn swap_valves(ways: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_ways = Vec::new();
    if ways.len() == 1 {
        let way = &ways[0];
        let mut new_way = Vec::new();
        let n = way.len();
        let r1 = rand::thread_rng().gen_range(1..=n - 1);
        let mut r2 = rand::thread_rng().gen_range(1..=n - 1);
        while r2 == r1 {
            r2 = rand::thread_rng().gen_range(1..=n - 1);
        }
        for i in 0..n {
            if i == r1 {
                new_way.push(way[r2].to_owned());
            } else if i == r2 {
                new_way.push(way[r1].to_owned());
            } else {
                new_way.push(way[i].to_owned());
            }
        }
        new_ways.push(new_way);
    } else if ways.len() == 2 {
        let way_a = &ways[0];
        let way_b = &ways[1];
        let mut new_way_a = vec!["AA".to_string()];
        let mut new_way_b = vec!["AA".to_string()];
        let mut positive_valves = Vec::new();
        for i in 1..way_a.len() {
            positive_valves.push(way_a[i].to_owned());
        }
        for i in 1..way_b.len() {
            positive_valves.push(way_b[i].to_owned());
        }
        let n = positive_valves.len();
        let r1 = rand::thread_rng().gen_range(0..=n - 1);
        let mut r2 = rand::thread_rng().gen_range(0..=n - 1);
        while r2 == r1 {
            r2 = rand::thread_rng().gen_range(0..=n - 1);
        }
        let r_size = rand::thread_rng().gen_range(n / 4..=n - 1 - n / 4);
        for i in 0..r_size {
            if i == r1 {
                new_way_a.push(positive_valves[r2].to_owned());
            } else if i == r2 {
                new_way_a.push(positive_valves[r1].to_owned());
            } else {
                new_way_a.push(positive_valves[i].to_owned());
            }
        }
        for i in r_size..n {
            if i == r1 {
                new_way_b.push(positive_valves[r2].to_owned());
            } else if i == r2 {
                new_way_b.push(positive_valves[r1].to_owned());
            } else {
                new_way_b.push(positive_valves[i].to_owned());
            }
        }
        new_ways.push(new_way_a);
        new_ways.push(new_way_b);
    }
    new_ways
}

fn pressure(
    graph: &UnGraphMap<&str, usize>,
    ways: &Vec<Vec<String>>,
    valves: &HashMap<&String, usize>,
    given_time: usize,
) -> usize {
    let mut answer = 0;

    for way in ways.iter() {
        let mut remain_time = given_time;
        for i in 1..way.len() {
            let time_to_remove = graph.edge_weight(&way[i - 1], &way[i]).unwrap();
            if &remain_time <= time_to_remove {
                break;
            }
            remain_time -= time_to_remove;
            answer += remain_time * valves.get(&way[i].to_string()).unwrap();
        }
    }
    answer
}

fn input_to_valves(input: String) -> Vec<(String, usize, String)> {
    let re_valve_info = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnel[s]? lead[s]? to valve[s]? (.*)$",
    )
    .unwrap();
    let mut valves = Vec::<(String, usize, String)>::new();
    for line in input.lines() {
        let caps = re_valve_info.captures(line).unwrap();
        let valve_name: String = caps[1].parse().unwrap();
        let flow_rate: usize = caps[2].parse().unwrap();
        let other_valves: String = caps[3].parse().unwrap();

        valves.push((valve_name, flow_rate, other_valves));
    }
    valves
}

fn valves_to_graph(valves: &Vec<(String, usize, String)>) -> UnGraphMap<&str, usize> {
    let mut big_graph: UnGraphMap<&str, usize> = UnGraphMap::new();
    for v in valves.iter() {
        let others = v.2.split(", ");
        for o in others {
            big_graph.update_edge(&v.0.as_str(), o, 1);
        }
    }

    let real_valves: Vec<(String, usize)> = valves
        .iter()
        .filter(|v| v.1 > 0 || v.0 == "AA")
        .map(|v| (v.0.to_string(), v.1))
        .collect();
    let real_valves_names: Vec<String> = real_valves.iter().map(|v| v.0.to_string()).collect();
    let mut real_graph: UnGraphMap<&str, usize> = UnGraphMap::new();
    let distances = floyd_warshall(&big_graph, |_e| 1).unwrap();
    for (edge, dist) in distances.iter() {
        if real_valves_names.contains(&edge.0.to_string())
            && real_valves_names.contains(&edge.1.to_string())
            && dist > &0
        {
            let a = edge.0.clone();
            let b = edge.1.clone();
            real_graph.update_edge(a, b, dist + 1);
        }
    }
    real_graph
}
