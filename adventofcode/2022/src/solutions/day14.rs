use std::{collections::HashSet, str};

pub const EXAMPLE_ANSWER1: &str = "24";
pub const EXAMPLE_ANSWER2: &str = "93";

fn build_rock_data(input: String) -> HashSet<(u32, u32)> {
    fn trace_line(start: (u32, u32), end: (u32, u32)) -> Vec<(u32, u32)> {
        let mut results = Vec::new();
        let (sx, sy) = start;
        let (ex, ey) = end;
        if sx == ex {
            if sy <= ey {
                for i in sy..ey + 1 {
                    results.push((sx, i));
                }
            } else {
                for i in ey..sy + 1 {
                    results.push((sx, i));
                }
            }
        } else if sy == ey {
            if sx <= ex {
                for i in sx..ex + 1 {
                    results.push((i, sy));
                }
            } else {
                for i in ex..sx + 1 {
                    results.push((i, sy));
                }
            }
        }
        results
    }

    fn string_to_position(position: &str) -> (u32, u32) {
        let coords: Vec<u32> = position
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        (coords[0], coords[1])
    }

    let mut rocks = HashSet::<(u32, u32)>::new();
    for line in input.lines() {
        let rocks_input = line.split(" -> ").collect::<Vec<&str>>();
        for i in 0..rocks_input.len() - 1 {
            let start = string_to_position(rocks_input[i]);
            let end = string_to_position(rocks_input[i + 1]);
            for r in trace_line(start, end) {
                rocks.insert(r);
            }
        }
    }
    rocks
}

fn sand_drops(
    rocks: &HashSet<(u32, u32)>,
    sands: &HashSet<(u32, u32)>,
    max: u32,
) -> Option<(u32, u32)> {
    let (mut x, mut y) = (500, 0);
    for h in 1..max + 1 {
        if !rocks.contains(&(x, h)) && !sands.contains(&(x, h)) {
            y = h;
        } else if !rocks.contains(&(x - 1, h)) && !sands.contains(&(x - 1, h)) {
            x -= 1;
            y = h;
        } else if !rocks.contains(&(x + 1, h)) && !sands.contains(&(x + 1, h)) {
            x += 1;
            y = h;
        } else {
            return Some((x, y));
        }
    }
    None
}
fn sand_drops_2(rocks: &HashSet<(u32, u32)>, sands: &HashSet<(u32, u32)>, max: u32) -> (u32, u32) {
    let (mut x, mut y) = (500, 0);
    for h in 1..max + 2 {
        if !rocks.contains(&(x, h)) && !sands.contains(&(x, h)) {
            y = h;
        } else if !rocks.contains(&(x - 1, h)) && !sands.contains(&(x - 1, h)) {
            x -= 1;
            y = h;
        } else if !rocks.contains(&(x + 1, h)) && !sands.contains(&(x + 1, h)) {
            x += 1;
            y = h;
        } else {
            return (x, y);
        }
    }
    (x, y)
}

pub fn day14_1(input: String) -> String {
    let rocks = build_rock_data(input);
    let mut sands = HashSet::<(u32, u32)>::new();
    let max = rocks.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 + 1;

    let mut drop = sand_drops(&rocks, &sands, max);
    while drop.is_some() {
        sands.insert(drop.unwrap());
        drop = sand_drops(&rocks, &sands, max);
    }

    sands.len().to_string()
}
pub fn day14_2(input: String) -> String {
    let rocks = build_rock_data(input);
    let mut sands = HashSet::<(u32, u32)>::new();
    let max = rocks.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut drop = sand_drops_2(&rocks, &sands, max);
    while drop != (500, 0) {
        sands.insert(drop);
        drop = sand_drops_2(&rocks, &sands, max);
    }
    sands.insert(drop);
    sands.len().to_string()
}
