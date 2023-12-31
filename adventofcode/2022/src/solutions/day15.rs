use std::{cmp, collections::HashSet, str};

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "26";
pub const EXAMPLE_ANSWER2: &str = "56000011";
fn build_sensor_beacon(input: String) -> (Vec<(isize, isize)>, Vec<(isize, isize)>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    let re_input =
        Regex::new(r"^Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)$").unwrap();

    for line in input.lines() {
        let caps = re_input.captures(line).unwrap();

        sensors.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
        beacons.push((caps[3].parse().unwrap(), caps[4].parse().unwrap()));
    }

    (sensors, beacons)
}

fn intersection(a: (isize, isize), b: (isize, isize)) -> Option<(isize, isize)> {
    let (min_a, max_a) = a;
    let (min_b, max_b) = b;

    if max_a <= max_b && max_a >= min_b {
        return Some((cmp::max(min_a, min_b), max_a));
    }
    if max_b <= max_a && max_b >= min_a {
        return Some((cmp::max(min_a, min_b), max_b));
    }

    None
}

fn merge(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let (min_a, max_a) = a;
    let (min_b, max_b) = b;

    (cmp::min(min_a, min_b), cmp::max(max_a, max_b))
}

fn add_interval(intervals: &mut Vec<(isize, isize)>, a: (isize, isize)) -> () {
    for i in 0..intervals.len() {
        let (i_min, i_max) = intervals[i];
        if intersection((i_min, i_max), a).is_some() {
            intervals[i] = merge((i_min, i_max), a);
            return;
        }
    }
    intervals.push(a);
}

fn is_in_intervals(intervals: &Vec<(isize, isize)>, position: isize) -> bool {
    for i in intervals {
        if i.0 <= position && position <= i.1 {
            return true;
        }
    }
    false
}

fn find_non_beacon_position(
    y: isize,
    sensors: &Vec<(isize, isize)>,
    beacons: &Vec<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut intervals = Vec::new();
    for p in 0..sensors.len() {
        let (sx, sy) = sensors[p];
        let (bx, by) = beacons[p];
        let d = (bx - sx).abs() + (by - sy).abs();
        if d >= (sy - y).abs() {
            let range = d - (sy - y).abs();
            add_interval(&mut intervals, (sx - range, sx + range));
        }
    }
    if intervals.len() < 2 {
        return intervals;
    }
    intervals.sort_by_key(|i| i.1);
    let mut final_intervals = Vec::new();
    for i in (1..intervals.len()).rev() {
        let first = intervals[i - 1];
        let second = intervals[i];
        if intersection(first, second).is_some() {
            intervals[i - 1] = merge(first, second);
        } else {
            final_intervals.push(second);
        }
        intervals.remove(i);
    }
    final_intervals.push(intervals[0]);
    final_intervals
}

fn count_positions(
    y: isize,
    sensors: &Vec<(isize, isize)>,
    beacons: &Vec<(isize, isize)>,
) -> isize {
    let intervals = find_non_beacon_position(y, &sensors, &beacons);
    let mut answer = intervals.iter().map(|i| i.1 - i.0 + 1).sum();
    let mut excluded_positions = HashSet::new();
    for s in sensors {
        let (sx, sy) = s;
        if sy == &y && is_in_intervals(&intervals, *sx) {
            excluded_positions.insert(sx);
        }
    }
    for b in beacons {
        let (bx, by) = b;
        if by == &y && is_in_intervals(&intervals, *bx) {
            excluded_positions.insert(bx);
        }
    }
    for _ in excluded_positions {
        answer -= 1;
    }
    answer
}
pub fn day15_1(input: String) -> String {
    let (sensors, beacons) = build_sensor_beacon(input);
    let answer10 = count_positions(10, &sensors, &beacons);
    let answer2000000 = count_positions(2000000, &sensors, &beacons);
    println!("y=10 : {}", answer10);
    println!("y=2000000 : {}", answer2000000);

    answer10.to_string()
}

fn find_positions_in_range(
    y: isize,
    sensors: &Vec<(isize, isize)>,
    beacons: &Vec<(isize, isize)>,
    range: (isize, isize),
) -> Vec<(isize, isize)> {
    let mut intervals = find_non_beacon_position(y, &sensors, &beacons);
    intervals = intervals
        .iter()
        .filter(|i| intersection(**i, range).is_some())
        .map(|i| intersection(*i, range).unwrap())
        .collect();
    intervals
}

fn find_beacon(
    sensors: &Vec<(isize, isize)>,
    beacons: &Vec<(isize, isize)>,
    range: isize,
) -> String {
    for y in 0..range + 1 {
        let intervals = find_positions_in_range(y, sensors, beacons, (0, range));
        if intervals.iter().map(|i| i.1 - i.0 + 1).sum::<isize>() == range {
            println!("y={}", y);
            let mut x = 0;
            if intervals.len() == 1 && intervals[0].1 == range - 1 {
                x = range;
            } else if intervals.len() == 2 {
                if intervals[0].0 == 0 {
                    x = intervals[0].1 + 1;
                } else {
                    x = intervals[0].0 - 1;
                }
            }
            let answer_example = x * 4000000 + y;
            return answer_example.to_string();
        }
    }
    "error".to_string()
}

pub fn day15_2(input: String) -> String {
    let (sensors, beacons) = build_sensor_beacon(input);
    let example = find_beacon(&sensors, &beacons, 20);
    let true_answer = find_beacon(&sensors, &beacons, 4000000);
    println!("example {}, true answer {}", example, true_answer);
    example
}
