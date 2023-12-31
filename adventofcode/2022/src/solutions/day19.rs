use std::collections::VecDeque;

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "33";
pub const EXAMPLE_ANSWER2: &str = "3472"; // 56*62

fn read_blueprints(input: String) -> Vec<(u32, u32, (u32, u32), (u32, u32))> {
    let re_blueprint = Regex::new(r"^Blueprint [0-9]+: Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.$").unwrap();
    let mut blueprints: Vec<(u32, u32, (u32, u32), (u32, u32))> = Vec::new();
    for line in input.lines() {
        let caps = re_blueprint.captures(line).unwrap();
        blueprints.push((
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
        ));
    }
    blueprints
}

struct State {
    pub ressources: Vec<u32>,
    pub robots: Vec<u32>,
    pub time: u32,
}

impl State {
    fn new(time: u32, ressources: Vec<u32>, robots: Vec<u32>) -> State {
        State {
            ressources,
            robots,
            time,
        }
    }
}
fn find_max(blueprint: &(u32, u32, (u32, u32), (u32, u32)), available_time: u32) -> usize {
    let mut max = 0;
    let mut stack: VecDeque<State> = VecDeque::new();
    let max_robots = (
        blueprint
            .0
            .max(blueprint.1)
            .max(blueprint.2 .0)
            .max(blueprint.3 .0),
        blueprint.2 .1,
        blueprint.3 .1,
    );
    stack.push_front(State::new(1, vec![1, 0, 0, 0], vec![1, 0, 0, 0]));
    while !stack.is_empty() {
        let node = stack.pop_front().unwrap();

        let mut max_possible_geodes = node.ressources[3];
        for t in 0..available_time - node.time {
            max_possible_geodes += node.robots[3] + t;
        }

        if node.time == available_time - 1 {
            max = std::cmp::max(max, node.ressources[3] + node.robots[3]);
        } else if max_possible_geodes <= max {
            //stop because can't fight best with perfect scenario
        } else {
            if node.robots[0] < max_robots.0 {
                // build ore robot
                let mut new_time = node.time;
                let mut ressources = node.ressources.clone();
                let mut robots = node.robots.clone();
                while ressources[0] < blueprint.0 {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                }
                if new_time < available_time {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                    robots[0] += 1;
                    ressources[0] -= blueprint.0;
                    stack.push_front(State::new(new_time, ressources, robots));
                }
            }

            if node.robots[1] < max_robots.1 {
                // build clay robot
                let mut new_time = node.time;
                let mut ressources = node.ressources.clone();
                let mut robots = node.robots.clone();
                while ressources[0] < blueprint.1 {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                }
                if new_time < available_time {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                    robots[1] += 1;
                    ressources[0] -= blueprint.1;
                    stack.push_front(State::new(new_time, ressources, robots));
                }
            }

            if node.robots[2] < max_robots.2 && node.robots[1] > 0 {
                // build obsidian robot
                let mut new_time = node.time;
                let mut ressources = node.ressources.clone();
                let mut robots = node.robots.clone();
                while ressources[0] < blueprint.2 .0 || ressources[1] < blueprint.2 .1 {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                }
                if new_time < available_time {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                    robots[2] += 1;
                    ressources[0] -= blueprint.2 .0;
                    ressources[1] -= blueprint.2 .1;
                    stack.push_front(State::new(new_time, ressources, robots));
                }
            }

            if node.robots[2] > 0 {
                // build geo robot
                let mut new_time = node.time;
                let mut ressources = node.ressources.clone();
                let mut robots = node.robots.clone();
                while ressources[0] < blueprint.3 .0 || ressources[2] < blueprint.3 .1 {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                }
                if new_time < available_time {
                    new_time += 1;
                    for i in 0..4 {
                        ressources[i] += robots[i];
                    }
                    robots[3] += 1;
                    ressources[0] -= blueprint.3 .0;
                    ressources[2] -= blueprint.3 .1;
                    stack.push_front(State::new(new_time, ressources, robots));
                }
            }
        }
    }

    max.try_into().unwrap()
}

pub fn day19_1(input: String) -> String {
    let blueprints = read_blueprints(input);
    let mut answer = 0;
    for i in 0..blueprints.len() {
        answer += (i + 1) * find_max(&blueprints[i], 24);
    }
    answer.to_string()
}

pub fn day19_2(input: String) -> String {
    let blueprints = read_blueprints(input);
    let mut answer = 1;
    for i in 0..blueprints.len().min(3) {
        answer *= find_max(&blueprints[i], 32);
    }
    answer.to_string()
}
