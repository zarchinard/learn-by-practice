use std::collections::{HashMap, HashSet};

pub const EXAMPLE_ANSWER1: &str = "110";
pub const EXAMPLE_ANSWER2: &str = "20";

fn read_input(input: String) -> HashSet<(isize, isize)> {
    let mut elves: HashSet<(isize, isize)> = HashSet::new();
    let mut x = 0;
    for line in input.lines() {
        for (y, char) in line.char_indices() {
            if char == '#' {
                elves.insert((x, y as isize));
            }
        }
        x += 1;
    }
    elves
}

fn round(elves: &mut HashSet<(isize, isize)>, start_direction: usize) -> bool {
    let has_neighbours = |elf: (isize, isize)| -> bool {
        let (x, y) = elf;
        !elves.is_disjoint(&HashSet::from([
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]))
    };
    let check_by_direction = |elf: (isize, isize), direction: usize| -> Option<(isize, isize)> {
        let (x, y) = elf;
        match direction {
            0 => {
                if elves.contains(&(x - 1, y - 1))
                    || elves.contains(&(x - 1, y))
                    || elves.contains(&(x - 1, y + 1))
                {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            1 => {
                if elves.contains(&(x + 1, y - 1))
                    || elves.contains(&(x + 1, y))
                    || elves.contains(&(x + 1, y + 1))
                {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            2 => {
                if elves.contains(&(x - 1, y - 1))
                    || elves.contains(&(x, y - 1))
                    || elves.contains(&(x + 1, y - 1))
                {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            3 => {
                if elves.contains(&(x - 1, y + 1))
                    || elves.contains(&(x, y + 1))
                    || elves.contains(&(x + 1, y + 1))
                {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            _ => None,
        }
    };

    let mut propositions: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for elf in elves.iter() {
        if has_neighbours(*elf) {
            for i in 0..4 {
                if let Some(proposition) = check_by_direction(*elf, (start_direction + i) % 4) {
                    if propositions.contains_key(&proposition) {
                        propositions.get_mut(&proposition).unwrap().push(*elf);
                    } else {
                        propositions.insert(proposition, vec![*elf]);
                    }
                    break;
                }
            }
        }
    }
    for (p, from) in propositions.iter() {
        if from.len() == 1 {
            elves.remove(&from[0]);
            elves.insert(*p);
        }
    }
    propositions.iter().filter(|p| p.1.len() == 1).count() == 0
}

pub fn day23_1(input: String) -> String {
    let mut elves = read_input(input);
    for i in 0..10 {
        round(&mut elves, i % 4);
    }
    let min_x = elves.iter().map(|e| e.0).min().unwrap();
    let max_x = elves.iter().map(|e| e.0).max().unwrap();
    let min_y = elves.iter().map(|e| e.1).min().unwrap();
    let max_y = elves.iter().map(|e| e.1).max().unwrap();
    let empty_number = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as isize;
    empty_number.to_string()
}
pub fn day23_2(input: String) -> String {
    let mut elves = read_input(input);
    let mut i = 0;
    while i < 10000 {
        if round(&mut elves, i % 4) {
            break;
        }
        i += 1;
    }
    (i + 1).to_string()
}
