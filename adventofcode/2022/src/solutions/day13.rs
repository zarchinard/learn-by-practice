use std::{cmp::Ordering, str};

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "13";
pub const EXAMPLE_ANSWER2: &str = "140";
fn read_pairs(input: String) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut pair: Vec<String> = Vec::new();

    for line in input.lines() {
        if line == "" && pair.len() == 2 {
            pairs.push((pair[0].to_owned(), pair[1].to_owned()));
            pair.clear();
        } else {
            pair.push(line.to_owned());
        }
    }

    pairs
}

// return true if a <= b
fn compare(a: String, b: String, mut a_rest: Vec<String>, mut b_rest: Vec<String>) -> bool {
    if !a.starts_with("[") && !b.starts_with("[") {
        let int_a: u32 = a.parse().unwrap();
        let int_b: u32 = b.parse().unwrap();
        if int_a != int_b {
            return int_a < int_b;
        }
    } else if !a.starts_with("[") && b.starts_with("[") {
        let a_array = "[".to_string() + &a + "]";
        return compare(a_array.to_owned(), b, a_rest, b_rest);
    } else if a.starts_with("[") && !b.starts_with("[") {
        let b_array = "[".to_string() + &b + "]";
        return compare(a, b_array.to_owned(), a_rest, b_rest);
    } else if a == "[]" && b != "[]" {
        return true;
    } else if a != "[]" && b == "[]" {
        return false;
    } else if a != "[]" && b != "[]" {
        let (new_a, add_rest_a) = extract_first_element(a);
        let (new_b, add_rest_b) = extract_first_element(b);
        a_rest.push(add_rest_a);
        b_rest.push(add_rest_b);
        return compare(new_a, new_b, a_rest, b_rest);
    }
    let new_a = a_rest.pop().unwrap();
    let new_b = b_rest.pop().unwrap();
    compare(new_a, new_b, a_rest, b_rest)
}

fn extract_first_element(input: String) -> (String, String) {
    let re_array: Regex = Regex::new(r"^\[(.+)\]$").unwrap();
    let caps = re_array.captures(&input).unwrap();
    let mut inside_string = caps[1].parse().unwrap();
    let first_comma_pos = find_first_comma(&inside_string);
    if first_comma_pos.is_none() {
        return (inside_string, "[]".to_string());
    } else {
        let second_part = inside_string.split_off(first_comma_pos.unwrap());
        return (
            inside_string,
            "[".to_owned() + second_part.strip_prefix(",").unwrap() + "]",
        );
    }
}

fn find_first_comma(input: &String) -> Option<usize> {
    let mut opened = 0;
    for (pos, c) in input.char_indices() {
        if opened == 0 && c == ',' {
            return Some(pos);
        }
        if c == '[' {
            opened += 1;
        } else if c == ']' {
            opened -= 1;
        }
    }
    None
}

pub fn day13_1(input: String) -> String {
    let pairs = read_pairs(input);
    let results: Vec<bool> = pairs
        .iter()
        .map(|(a, b)| compare(a.to_owned(), b.to_owned(), Vec::new(), Vec::new()))
        .collect();
    let mut sum = 0;
    for (i, e) in results.iter().enumerate() {
        if *e {
            sum += i + 1;
        }
    }
    sum.to_string()
}
pub fn day13_2(input: String) -> String {
    let mut paquets = Vec::<String>::new();
    for line in input.lines() {
        if line != "" {
            paquets.push(line.to_owned());
        }
    }
    paquets.push("[[2]]".to_owned());
    paquets.push("[[6]]".to_owned());
    paquets.sort_by(
        |a, b| match compare(a.to_owned(), b.to_owned(), Vec::new(), Vec::new()) {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
    );
    let mut answer = 1;
    for (pos, e) in paquets.iter().enumerate() {
        if e == "[[2]]" || e == "[[6]]" {
            answer *= pos + 1;
        }
    }
    answer.to_string()
}
