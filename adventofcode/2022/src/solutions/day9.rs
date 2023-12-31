use std::collections::HashSet;
use std::str;

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "13";
pub const EXAMPLE_ANSWER2: &str = "1";

struct Position {
    pub x: i32,
    pub y: i32,
}

fn build_position(x: i32, y: i32) -> Position {
    Position { x, y }
}

impl Position {
    fn move_to(&mut self, direction: char) -> () {
        match direction {
            'R' => self.x += 1,
            'L' => self.x -= 1,
            'U' => self.y += 1,
            'D' => self.y -= 1,
            _ => (),
        }
    }

    fn follow_head(&mut self, (x, y): (i32, i32)) -> () {
        if (self.x - x).abs() > 1 || (self.y - y).abs() > 1 {
            self.x += (x - self.x).signum();
            self.y += (y - self.y).signum();
        }
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn day9_1(input: String) -> String {
    let re_indication = Regex::new(r"^([UDRL]) ([0-9]+)$").unwrap();
    let mut head = build_position(0, 0);
    let mut tail = build_position(0, 0);

    let mut tail_positions = HashSet::<(i32, i32)>::new();
    tail_positions.insert(tail.get_position());
    for line in input.lines() {
        let caps = re_indication.captures(&line).unwrap();
        let direction: char = caps[1].parse().unwrap();
        let number = caps[2].parse().unwrap();
        for _ in 0..number {
            head.move_to(direction);
            tail.follow_head(head.get_position());
            tail_positions.insert(tail.get_position());
        }
    }

    tail_positions.len().to_string()
}
pub fn day9_2(input: String) -> String {
    let re_indication = Regex::new(r"^([UDRL]) ([0-9]+)$").unwrap();
    let mut knots: Vec<Position> = Vec::new();
    for _ in 0..10 {
        knots.push(build_position(0, 0));
    }

    let mut tail_positions = HashSet::<(i32, i32)>::new();
    tail_positions.insert(knots[9].get_position());
    for line in input.lines() {
        let caps = re_indication.captures(&line).unwrap();
        let direction: char = caps[1].parse().unwrap();
        let number = caps[2].parse().unwrap();
        for _ in 0..number {
            knots[0].move_to(direction);
            for k in 1..10 {
                let position_to_follow = knots[k - 1].get_position();
                knots[k].follow_head(position_to_follow);
            }
            tail_positions.insert(knots[9].get_position());
        }
    }

    tail_positions.len().to_string()
}
