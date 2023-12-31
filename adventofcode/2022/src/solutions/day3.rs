pub const EXAMPLE_ANSWER1: &str = "157";

fn item_score(item: char) -> usize {
    let lower_alphabet = "abcdefghijklmnopqrstuvwxyz";
    lower_alphabet
        .chars()
        .chain(lower_alphabet.to_uppercase().chars())
        .position(|c| c == item)
        .unwrap()
        + 1
}

pub fn day3_1(input: String) -> String {
    let mut answer = 0;
    for line in input.lines() {
        let size = line.len();
        let (compartment_1, compartment_2) = line.split_at(size / 2);
        for c in compartment_1.chars() {
            if compartment_2.contains(c) {
                answer = answer + item_score(c);
                break;
            }
        }
    }

    answer.to_string()
}

pub const EXAMPLE_ANSWER2: &str = "70";

pub fn day3_2(input: String) -> String {
    fn find_badge(group: &mut Vec<&str>, answer: &mut usize) -> () {
        let one = group[0];
        let two = group[1];
        let three = group[2];

        for c in one.chars() {
            if two.contains(c) && three.contains(c) {
                *answer = *answer + item_score(c);
                break;
            }
        }

        group.clear();
    }
    let mut answer = 0;
    let mut group: Vec<&str> = Vec::new();
    for line in input.lines() {
        if group.len() == 3 {
            find_badge(&mut group, &mut answer);
        }
        group.push(line);
    }

    find_badge(&mut group, &mut answer);
    answer.to_string()
}
