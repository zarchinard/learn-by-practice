use core::panic;

pub const EXAMPLE_ANSWER1: &str = "15";
pub const EXAMPLE_ANSWER2: &str = "12";

fn score1(round: &str) -> u32 {
    let r: u32 = match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("no matching round {}", round),
    };
    r
}

fn score2(round: &str) -> u32 {
    let r: u32 = match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!("no matching round {}", round),
    };
    r
}
fn total_score(input: String, score: fn(&str) -> u32) -> String {
    let mut answer = 0;
    for line in input.lines() {
        answer = answer + score(line);
    }
    String::from(answer.to_string())
}
pub fn day2_1(input: String) -> String {
    total_score(input, score1)
}
pub fn day2_2(input: String) -> String {
    total_score(input, score2)
}
