use std::str;

use regex::Regex;

pub const EXAMPLE_ANSWER1: &str = "10605";
pub const EXAMPLE_ANSWER2: &str = "2713310158";
#[derive(Debug)]
struct Monkey {
    pub items: Vec<usize>,
    pub worry_up: (bool, usize), // true=add, false=times, usize=0 means old
    pub divisible_by: usize,
    pub yes: usize,
    pub no: usize,
    pub inspections: usize,
}

impl Monkey {
    fn new(
        items: &Vec<usize>,
        worry_up: (bool, usize),
        divisible_by: usize,
        yes: usize,
        no: usize,
    ) -> Monkey {
        Monkey {
            items: items.clone(),
            worry_up,
            divisible_by,
            yes,
            no,
            inspections: 0,
        }
    }
    fn inspect_item(&self, item: &usize, with_relief: usize) -> (usize, usize) {
        let worry_up = match self.worry_up {
            (true, 0) => item + item,
            (false, 0) => item * item,
            (true, x) => item + x,
            (false, x) => item * x,
        };
        let new_worry = if with_relief == 0 {
            worry_up / 3
        } else {
            worry_up % with_relief
        };
        let throw_to = if new_worry % self.divisible_by == 0 {
            self.yes
        } else {
            self.no
        };
        (throw_to, new_worry)
    }
    fn inspect_items(&mut self, with_relief: usize) -> Vec<(usize, usize)> {
        let changes: Vec<(usize, usize)> = self
            .items
            .iter()
            .map(|item| self.inspect_item(item, with_relief))
            .collect();

        self.items.clear();
        self.inspections += changes.len();
        changes
    }
}

fn build_monkeys(input: String) -> Vec<Monkey> {
    let re_items = Regex::new(r"^  Starting items: (.+)$").unwrap();
    let re_operation = Regex::new(r"^  Operation: new = old ([\+\*]) (.+)$").unwrap();
    let re_divisible = Regex::new(r"^  Test: divisible by (.+)$").unwrap();
    let re_yes = Regex::new(r"^    If true: throw to monkey (.+)$").unwrap();
    let re_no = Regex::new(r"^    If false: throw to monkey (.+)$").unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: Vec<usize> = Vec::new();
    let mut worry_up = (true, 0);
    let mut divisible = 0;
    let mut yes = 0;
    let mut no;
    for line in input.lines() {
        if re_items.is_match(&line) {
            let caps = re_items.captures(&line).unwrap();
            let string_items: String = caps[1].parse().unwrap();
            items = string_items
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();
        } else if re_operation.is_match(&line) {
            let caps = re_operation.captures(&line).unwrap();
            let operator: String = caps[1].parse().unwrap();
            let operation_value: String = caps[2].parse().unwrap();
            let boolean_value = if operator == "+" { true } else { false };
            let value_value = if operation_value == "old" {
                0
            } else {
                operation_value.parse().unwrap()
            };
            worry_up = (boolean_value, value_value);
        } else if re_divisible.is_match(&line) {
            let caps = re_divisible.captures(&line).unwrap();
            divisible = caps[1].parse().unwrap();
        } else if re_yes.is_match(&line) {
            let caps = re_yes.captures(&line).unwrap();
            yes = caps[1].parse().unwrap();
        } else if re_no.is_match(&line) {
            let caps = re_no.captures(&line).unwrap();
            no = caps[1].parse().unwrap();
            let monkey = Monkey::new(&items, worry_up, divisible, yes, no);
            monkeys.push(monkey);
        }
    }

    monkeys
}

fn day11(input: String, relief: usize, turn_number: usize) -> String {
    let mut monkeys = build_monkeys(input);
    let with_relief = if relief == 0 {
        relief
    } else {
        monkeys
            .iter()
            .map(|m| m.divisible_by)
            .reduce(|acc, e| acc * e)
            .unwrap()
    };
    for _ in 0..turn_number {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let changes = monkey.inspect_items(with_relief);
            for (throw_to, throw_value) in changes {
                monkeys[throw_to].items.push(throw_value);
            }
        }
    }
    let mut inspection_numbers: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspection_numbers.sort();
    let ordered: Vec<&usize> = inspection_numbers.iter().rev().collect();

    (ordered[0] * ordered[1]).to_string()
}
pub fn day11_1(input: String) -> String {
    day11(input, 0, 20)
}

pub fn day11_2(input: String) -> String {
    day11(input, 1, 10000)
}
