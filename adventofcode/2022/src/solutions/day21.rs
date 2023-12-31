use regex::Regex;
use rust_decimal::prelude::*;
use std::collections::HashMap;

pub const EXAMPLE_ANSWER1: &str = "152";
pub const EXAMPLE_ANSWER2: &str = "301";

fn read_input(input: String) -> HashMap<String, String> {
    let re_monkey = Regex::new(r"^([a-z]{4}): (.*)$").unwrap();
    let mut input_map: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        let caps = re_monkey.captures(line).unwrap();
        input_map.insert(caps[1].parse().unwrap(), caps[2].parse().unwrap());
    }
    input_map
}

fn root_value(input_map: &HashMap<String, String>) -> Decimal {
    let mut clear_map: HashMap<String, Decimal> = HashMap::new();
    let re_number = Regex::new(r"^[0-9]+$").unwrap();
    let re_operation = Regex::new(r"^([a-z]{4}) ([\\+-\\*/]{1}) ([a-z]{4})$").unwrap();
    while !clear_map.contains_key("root") {
        for key in input_map.keys() {
            if !clear_map.contains_key(key) {
                let v = input_map.get(key).unwrap();
                if re_number.is_match(v) {
                    clear_map.insert(key.to_owned(), v.parse().unwrap());
                } else {
                    // println!("{}", v);
                    let caps = re_operation.captures(v).unwrap();
                    let a: String = caps[1].parse().unwrap();
                    let op: String = caps[2].parse().unwrap();
                    let b: String = caps[3].parse().unwrap();
                    if clear_map.contains_key(&a) && clear_map.contains_key(&b) {
                        match op.as_str() {
                            "+" => {
                                clear_map.insert(
                                    key.to_owned(),
                                    clear_map.get(&a).unwrap() + clear_map.get(&b).unwrap(),
                                );
                            }
                            "-" => {
                                clear_map.insert(
                                    key.to_owned(),
                                    clear_map.get(&a).unwrap() - clear_map.get(&b).unwrap(),
                                );
                            }
                            "*" => {
                                clear_map.insert(
                                    key.to_owned(),
                                    clear_map.get(&a).unwrap() * clear_map.get(&b).unwrap(),
                                );
                            }
                            "/" => {
                                clear_map.insert(
                                    key.to_owned(),
                                    clear_map.get(&a).unwrap() / clear_map.get(&b).unwrap(),
                                );
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    *clear_map.get(&"root".to_string()).unwrap()
}

pub fn day21_1(input: String) -> String {
    let input_map = read_input(input);
    root_value(&input_map).to_string()
}

pub fn day21_2(input: String) -> String {
    let mut input_map = read_input(input);
    let mut v = input_map.get("root").unwrap().to_owned();
    v.replace_range(5..6, "-");
    input_map.insert("root".to_string(), v);

    // humn = x, root(x) = 0 = Ax + B, x = -B/A, root(0) = B, root(1) = A + B, x = root(0) / (root(0)-root(1))
    input_map.insert("humn".to_string(), "0".to_string());
    let root_zero = root_value(&input_map).clone();
    input_map.insert("humn".to_string(), "1".to_string());
    let root_one = root_value(&input_map).clone();
    (root_zero / (root_zero - root_one)).round().to_string()
}
