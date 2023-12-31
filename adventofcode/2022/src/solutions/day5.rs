pub const EXAMPLE_ANSWER1: &str = "CMZ";
pub const EXAMPLE_ANSWER2: &str = "MCD";
use regex::Regex;

pub fn day5_1(input: String) -> String {
    day5(input, false)
}

pub fn day5_2(input: String) -> String {
    day5(input, true)
}

fn day5(input: String, multiple: bool) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line.contains("[") {
            building_stacks(line, &mut stacks);
        } else if line.contains("move") {
            moving_stacks(line, &mut stacks, multiple);
        }
    }
    stacks
        .into_iter()
        .map(|stack| stack.first().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn building_stacks(crates: &str, stacks: &mut Vec<Vec<char>>) -> () {
    for (i, c) in crates.char_indices() {
        if i % 4 == 1 && c != ' ' {
            let stack_index = i / 4;
            if stack_index >= stacks.len() {
                stacks.resize_with(stack_index + 1, || Vec::new());
            }
            stacks[stack_index].push(c);
        }
    }
}

fn moving_stacks(the_move: &str, stacks: &mut Vec<Vec<char>>, multiple: bool) -> () {
    let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    let caps = re.captures(the_move).unwrap();
    let (number_crates, from, to): (usize, usize, usize) = (
        caps[1].parse().unwrap(),
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
    );
    let mut moved_crates: Vec<char> = if multiple {
        stacks[from - 1].drain(0..number_crates).collect()
    } else {
        stacks[from - 1].drain(0..number_crates).rev().collect()
    };

    moved_crates.append(&mut stacks[to - 1]);
    stacks[to - 1] = moved_crates;
}
