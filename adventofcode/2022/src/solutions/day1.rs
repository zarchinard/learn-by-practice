pub const EXAMPLE_ANSWER1: &str = "24000";

pub fn day1_1(input: String) -> String {
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let more: u32 = line.parse().unwrap();
            current = current + more;
        }
    }
    String::from(max.to_string())
}

pub const EXAMPLE_ANSWER2: &str = "45000";

pub fn day1_2(input: String) -> String {
    let mut calories_per_elf: Vec<u32> = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            calories_per_elf.push(current);
            current = 0;
        } else {
            let more: u32 = line.parse().unwrap();
            current = current + more;
        }
    }
    calories_per_elf.push(current);
    calories_per_elf.sort();
    let mut sum_first_three = 0;
    for _number in 1..4 {
        sum_first_three = sum_first_three + calories_per_elf.pop().unwrap();
    }
    String::from(sum_first_three.to_string())
}
