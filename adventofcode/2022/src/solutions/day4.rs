use regex::Regex;
pub const EXAMPLE_ANSWER1: &str = "2";
pub const EXAMPLE_ANSWER2: &str = "4";

fn find_numbers(line: String) -> (u32, u32, u32, u32) {
    let re = Regex::new(r"^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)$").unwrap();
    let caps = re.captures(&line).unwrap();
    (
        caps[1].parse().unwrap(),
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
        caps[4].parse().unwrap(),
    )
}

fn fully_contains((min1, max1, min2, max2): (u32, u32, u32, u32)) -> bool {
    (min1 >= min2 && max1 <= max2) || (min1 <= min2 && max1 >= max2)
}

fn overlaps((min1, max1, min2, max2): (u32, u32, u32, u32)) -> bool {
    fully_contains((min1, max1, min2, max2))
        || (min1 >= min2 && min1 <= max2)
        || (max1 >= min2 && max1 <= max2)
}
fn day4(input: String, criteria: fn((u32, u32, u32, u32)) -> bool) -> String {
    let mut answer = 0;
    for line in input.lines() {
        let intervals = find_numbers(line.to_string());

        if criteria(intervals) {
            answer += 1;
        }
    }
    answer.to_string()
}

pub fn day4_1(input: String) -> String {
    day4(input, fully_contains)
}

pub fn day4_2(input: String) -> String {
    day4(input, overlaps)
}
