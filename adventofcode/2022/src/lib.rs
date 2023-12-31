use std::error::Error;

use crate::utils::Puzzle;

pub mod solutions;
pub mod utils;

#[derive(Debug)]
pub struct Config {
    pub cookie: String,
    pub day: String,
    pub level: String,
    pub send_answer: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        let cookie = args[1].clone();
        let day = args[2].clone();
        let level = args[3].clone();

        let send_answer: bool = if args.len() >= 5 {
            args[4].clone().parse().unwrap()
        } else {
            false
        };

        Ok(Config {
            cookie,
            day,
            level,
            send_answer,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (example_answer, solve): (String, fn(String) -> String) =
        match (config.day.as_str(), config.level.as_str()) {
            ("1", "1") => (
                String::from(solutions::day1::EXAMPLE_ANSWER1),
                solutions::day1::day1_1,
            ),
            ("1", "2") => (
                String::from(solutions::day1::EXAMPLE_ANSWER2),
                solutions::day1::day1_2,
            ),
            ("2", "1") => (
                String::from(solutions::day2::EXAMPLE_ANSWER1),
                solutions::day2::day2_1,
            ),
            ("2", "2") => (
                String::from(solutions::day2::EXAMPLE_ANSWER2),
                solutions::day2::day2_2,
            ),
            ("3", "1") => (
                solutions::day3::EXAMPLE_ANSWER1.to_string(),
                solutions::day3::day3_1,
            ),

            ("3", "2") => (
                solutions::day3::EXAMPLE_ANSWER2.to_string(),
                solutions::day3::day3_2,
            ),
            ("4", "1") => (
                solutions::day4::EXAMPLE_ANSWER1.to_string(),
                solutions::day4::day4_1,
            ),
            ("4", "2") => (
                solutions::day4::EXAMPLE_ANSWER2.to_string(),
                solutions::day4::day4_2,
            ),
            ("5", "1") => (
                solutions::day5::EXAMPLE_ANSWER1.to_string(),
                solutions::day5::day5_1,
            ),
            ("5", "2") => (
                solutions::day5::EXAMPLE_ANSWER2.to_string(),
                solutions::day5::day5_2,
            ),
            ("6", "1") => (
                solutions::day6::EXAMPLE_ANSWER1.to_string(),
                solutions::day6::day6_1,
            ),
            ("6", "2") => (
                solutions::day6::EXAMPLE_ANSWER2.to_string(),
                solutions::day6::day6_2,
            ),
            ("7", "1") => (
                solutions::day7::EXAMPLE_ANSWER1.to_string(),
                solutions::day7::day7_1,
            ),
            ("7", "2") => (
                solutions::day7::EXAMPLE_ANSWER2.to_string(),
                solutions::day7::day7_2,
            ),
            ("8", "1") => (
                solutions::day8::EXAMPLE_ANSWER1.to_string(),
                solutions::day8::day8_1,
            ),
            ("8", "2") => (
                solutions::day8::EXAMPLE_ANSWER2.to_string(),
                solutions::day8::day8_2,
            ),
            ("9", "1") => (
                solutions::day9::EXAMPLE_ANSWER1.to_string(),
                solutions::day9::day9_1,
            ),
            ("9", "2") => (
                solutions::day9::EXAMPLE_ANSWER2.to_string(),
                solutions::day9::day9_2,
            ),
            ("10", "1") => (
                solutions::day10::EXAMPLE_ANSWER1.to_string(),
                solutions::day10::day10_1,
            ),
            ("10", "2") => (
                solutions::day10::EXAMPLE_ANSWER2.to_string(),
                solutions::day10::day10_2,
            ),
            ("11", "1") => (
                solutions::day11::EXAMPLE_ANSWER1.to_string(),
                solutions::day11::day11_1,
            ),
            ("11", "2") => (
                solutions::day11::EXAMPLE_ANSWER2.to_string(),
                solutions::day11::day11_2,
            ),
            ("12", "1") => (
                solutions::day12::EXAMPLE_ANSWER1.to_string(),
                solutions::day12::day12_1,
            ),
            ("12", "2") => (
                solutions::day12::EXAMPLE_ANSWER2.to_string(),
                solutions::day12::day12_2,
            ),
            ("13", "1") => (
                solutions::day13::EXAMPLE_ANSWER1.to_string(),
                solutions::day13::day13_1,
            ),
            ("13", "2") => (
                solutions::day13::EXAMPLE_ANSWER2.to_string(),
                solutions::day13::day13_2,
            ),
            ("14", "1") => (
                solutions::day14::EXAMPLE_ANSWER1.to_string(),
                solutions::day14::day14_1,
            ),
            ("14", "2") => (
                solutions::day14::EXAMPLE_ANSWER2.to_string(),
                solutions::day14::day14_2,
            ),
            ("15", "1") => (
                solutions::day15::EXAMPLE_ANSWER1.to_string(),
                solutions::day15::day15_1,
            ),
            ("15", "2") => (
                solutions::day15::EXAMPLE_ANSWER2.to_string(),
                solutions::day15::day15_2,
            ),
            ("16", "1") => (
                solutions::day16::EXAMPLE_ANSWER1.to_string(),
                solutions::day16::day16_1,
            ),
            ("16", "2") => (
                solutions::day16::EXAMPLE_ANSWER2.to_string(),
                solutions::day16::day16_2,
            ),
            ("17", "1") => (
                solutions::day17::EXAMPLE_ANSWER1.to_string(),
                solutions::day17::day17_1,
            ),
            ("17", "2") => (
                solutions::day17::EXAMPLE_ANSWER2.to_string(),
                solutions::day17::day17_2,
            ),
            ("18", "1") => (
                solutions::day18::EXAMPLE_ANSWER1.to_string(),
                solutions::day18::day18_1,
            ),
            ("18", "2") => (
                solutions::day18::EXAMPLE_ANSWER2.to_string(),
                solutions::day18::day18_2,
            ),
            ("19", "1") => (
                solutions::day19::EXAMPLE_ANSWER1.to_string(),
                solutions::day19::day19_1,
            ),
            ("19", "2") => (
                solutions::day19::EXAMPLE_ANSWER2.to_string(),
                solutions::day19::day19_2,
            ),
            ("20", "1") => (
                solutions::day20::EXAMPLE_ANSWER1.to_string(),
                solutions::day20::day20_1,
            ),
            ("20", "2") => (
                solutions::day20::EXAMPLE_ANSWER2.to_string(),
                solutions::day20::day20_2,
            ),
            ("21", "1") => (
                solutions::day21::EXAMPLE_ANSWER1.to_string(),
                solutions::day21::day21_1,
            ),
            ("21", "2") => (
                solutions::day21::EXAMPLE_ANSWER2.to_string(),
                solutions::day21::day21_2,
            ),
            ("22", "1") => (
                solutions::day22::EXAMPLE_ANSWER1.to_string(),
                solutions::day22::day22_1,
            ),
            ("22", "2") => (
                solutions::day22::EXAMPLE_ANSWER2.to_string(),
                solutions::day22::day22_2,
            ),
            ("23", "1") => (
                solutions::day23::EXAMPLE_ANSWER1.to_string(),
                solutions::day23::day23_1,
            ),
            ("23", "2") => (
                solutions::day23::EXAMPLE_ANSWER2.to_string(),
                solutions::day23::day23_2,
            ),
            ("24", "1") => (
                solutions::day24::EXAMPLE_ANSWER1.to_string(),
                solutions::day24::day24_1,
            ),
            ("24", "2") => (
                solutions::day24::EXAMPLE_ANSWER2.to_string(),
                solutions::day24::day24_2,
            ),
            ("25", "1") => (
                solutions::day25::EXAMPLE_ANSWER1.to_string(),
                solutions::day25::day25_1,
            ),
            ("25", "2") => (
                solutions::day25::EXAMPLE_ANSWER2.to_string(),
                solutions::day25::day25_2,
            ),
            _ => panic!("no matching puzzle"),
        };
    let puzzle = Puzzle {
        day: config.day,
        level: config.level,
        example_answer,
        solve,
    };
    puzzle.answer(&config.cookie, config.send_answer);
    Ok(())
}
