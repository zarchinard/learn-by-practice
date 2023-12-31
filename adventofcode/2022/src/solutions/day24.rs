use std::collections::{HashMap, HashSet};

pub const EXAMPLE_ANSWER1: &str = "18";
pub const EXAMPLE_ANSWER2: &str = "54";

fn read_input(input: String) -> HashMap<(usize, usize), char> {
    let mut valley: HashMap<(usize, usize), char> = HashMap::new();
    let mut x = 0;
    for line in input.lines() {
        for (indice, char) in line.char_indices() {
            valley.insert((x, indice), char);
        }
        x += 1
    }
    valley
}

fn schrodingers_elf(
    valley: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    end: (usize, usize),
    start_time: usize,
) -> usize {
    let valley_width = valley.keys().map(|k| k.1).max().unwrap() - 1;
    let valley_height = valley.keys().map(|k| k.0).max().unwrap() - 1;
    let blizzard = |position: (usize, usize), time: usize| -> bool {
        let (x, y) = position;
        valley
            .get(&(
                (x + valley_height - 1 - (time % valley_height)) % valley_height + 1,
                y,
            ))
            .unwrap()
            == &'v'
            || valley
                .get(&((x + valley_height - 1 + time) % valley_height + 1, y))
                .unwrap()
                == &'^'
            || valley
                .get(&(
                    x,
                    (y + valley_width - 1 - (time % valley_width)) % valley_width + 1,
                ))
                .unwrap()
                == &'>'
            || valley
                .get(&((x, (y + valley_width - 1 + time) % valley_width + 1)))
                .unwrap()
                == &'<'
    };

    let mut time = start_time;
    let mut elves: HashSet<(usize, usize)> = HashSet::from([start]);

    while !elves.contains(&end) {
        let living_elves: Vec<(usize, usize)> = elves
            .iter()
            .filter(|e| !blizzard(**e, time) && valley.get(e).unwrap() != &'#')
            .map(|e| e.clone())
            .collect();
        elves.clear();
        for elf in living_elves {
            let (x, y) = elf;
            elves.insert((x, y));
            elves.insert((if x == 0 { 0 } else { x - 1 }, y));
            elves.insert(((x + 1).min(valley_height + 1), y));
            elves.insert((x, y - 1));
            elves.insert((x, y + 1));
        }
        time += 1;
    }
    time - start_time
}

pub fn day24_1(input: String) -> String {
    let valley = read_input(input);
    schrodingers_elf(
        &valley,
        (0, 1),
        (
            valley.keys().map(|k| k.0).max().unwrap(),
            valley.keys().map(|k| k.1).max().unwrap() - 1,
        ),
        0,
    )
    .to_string()
}
pub fn day24_2(input: String) -> String {
    let valley = read_input(input);
    let start = (0, 1);
    let end = (
        valley.keys().map(|k| k.0).max().unwrap(),
        valley.keys().map(|k| k.1).max().unwrap() - 1,
    );
    let mut total_time = 0;
    let first = schrodingers_elf(&valley, start, end, total_time);
    total_time += first;
    let second = schrodingers_elf(&valley, end, start, total_time);
    total_time += second;
    let third = schrodingers_elf(&valley, start, end, total_time);
    total_time += third;
    total_time.to_string()
}
