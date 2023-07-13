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
    println!("{:?}", config);
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
