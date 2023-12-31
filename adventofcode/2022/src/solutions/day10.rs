use std::str;

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "13140";
pub const EXAMPLE_ANSWER2: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

struct Clock {
    pub value: i32,
    pub cycle: i32,
    pub signal_strength: i32,
    pub output: String,
}

fn build_clock() -> Clock {
    Clock {
        value: 1,
        cycle: 0,
        signal_strength: 0,
        output: String::new(),
    }
}

impl Clock {
    fn increment_cycle(&mut self, add_value: i32) {
        if self.cycle != 0 && self.cycle % 40 == 0 {
            self.output += "\n";
        }
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.signal_strength += self.cycle * self.value;
        }

        let position = (self.cycle - 1) % 40;
        let sprite_position = self.value;
        self.output += if (position - sprite_position).abs() < 2 {
            "#"
        } else {
            "."
        };

        self.value += add_value;
    }

    fn execute(&mut self, command: &str) -> () {
        let re_command = Regex::new(r"^addx (.+)$").unwrap();
        if command == "noop" {
            self.increment_cycle(0);
        } else {
            let caps = re_command.captures(&command).unwrap();
            let add_value: i32 = caps[1].parse().unwrap();
            self.increment_cycle(0);
            self.increment_cycle(add_value);
        }
    }
}
pub fn day10_1(input: String) -> String {
    let mut clock = build_clock();
    for line in input.lines() {
        clock.execute(line);
        if clock.cycle >= 220 {
            break;
        }
    }

    clock.signal_strength.to_string()
}

pub fn day10_2(input: String) -> String {
    let mut clock = build_clock();
    for line in input.lines() {
        clock.execute(line);
    }
    clock.output
}
