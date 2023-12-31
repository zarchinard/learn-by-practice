pub const EXAMPLE_ANSWER1: &str = "7";
pub const EXAMPLE_ANSWER2: &str = "19";

fn day6(input: String, last_n: usize) -> String {
    let mut last_chars: Vec<char> = Vec::with_capacity(last_n - 1);
    let mut answer = "not found".to_string();
    for (position, value) in input.char_indices() {
        if last_chars.len() < last_n - 1 {
            last_chars.push(value);
        } else {
            let mut copy = last_chars.clone();
            copy.sort();
            copy.dedup();
            if !last_chars.contains(&value) && copy.len() == last_n - 1 {
                answer = (position + 1).to_string();
                break;
            } else {
                last_chars.remove(0);
                last_chars.push(value);
            }
        }
    }
    answer
}
pub fn day6_1(input: String) -> String {
    day6(input, 4)
}
pub fn day6_2(input: String) -> String {
    day6(input, 14)
}
