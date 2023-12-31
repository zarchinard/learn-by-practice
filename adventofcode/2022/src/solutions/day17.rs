use std::collections::HashSet;

pub const EXAMPLE_ANSWER1: &str = "3068";
pub const EXAMPLE_ANSWER2: &str = "1514285714288";

#[derive(Copy, Clone)]
enum RockType {
    Hbar,
    Plus,
    L,
    Vbar,
    Square,
}

enum RockMove {
    Left,
    Right,
    Down,
}

struct Rock {
    positions: Vec<(usize, usize)>,
}

impl Rock {
    fn new(rock_type: RockType, height: usize) -> Rock {
        let y = height + 4;
        let positions = match rock_type {
            RockType::Hbar => vec![(3, y), (4, y), (5, y), (6, y)],
            RockType::Plus => vec![(4, y), (3, y + 1), (4, y + 1), (5, y + 1), (4, y + 2)],
            RockType::L => vec![(3, y), (4, y), (5, y), (5, y + 1), (5, y + 2)],
            RockType::Vbar => vec![(3, y), (3, y + 1), (3, y + 2), (3, y + 3)],
            RockType::Square => vec![(3, y), (4, y), (3, y + 1), (4, y + 1)],
        };
        Rock { positions }
    }

    fn moves(&mut self, rock_move: RockMove, data: &mut HashSet<(usize, usize)>) -> bool {
        let mut new_positions: Vec<(usize, usize)> = Vec::new();
        let positions = &self.positions;
        for i in 0..positions.len() {
            let p = positions[i];
            let new_p = match rock_move {
                RockMove::Left => (p.0 - 1, p.1),
                RockMove::Right => (p.0 + 1, p.1),
                RockMove::Down => (p.0, p.1 - 1),
            };
            new_positions.push(new_p);
        }

        if new_positions
            .iter()
            .filter(|p| p.0 < 1 || p.0 > 7 || p.1 < 1 || data.contains(p))
            .count()
            > 0
        {
            match rock_move {
                RockMove::Down => {
                    positions.iter().for_each(|p| {
                        data.insert(*p);
                    });
                    return false;
                }
                _ => (),
            }
        } else {
            self.positions = new_positions;
        }
        true
    }
}

fn day17(
    input: String,
    part: fn(Vec<RockType>, &mut HashSet<(usize, usize)>, Vec<char>) -> String,
) -> String {
    let mut data: HashSet<(usize, usize)> = HashSet::new();
    let jets: Vec<char> = input.lines().next().unwrap().chars().collect();
    let rock_types: Vec<RockType> = vec![
        RockType::Hbar,
        RockType::Plus,
        RockType::L,
        RockType::Vbar,
        RockType::Square,
    ];
    part(rock_types, &mut data, jets)
}

pub fn day17_1(input: String) -> String {
    day17(input, part_one)
}

fn part_one(
    rock_types: Vec<RockType>,
    data: &mut HashSet<(usize, usize)>,
    jets: Vec<char>,
) -> String {
    let jets_number = jets.len();
    let mut j = 0;
    for i in 0..2022 {
        let mut rock = Rock::new(rock_types[i % 5], data_height(data));
        let mut falling = true;

        while falling {
            let rock_move = if jets[j % jets_number] == '<' {
                RockMove::Left
            } else {
                RockMove::Right
            };
            rock.moves(rock_move, data);
            falling = rock.moves(RockMove::Down, data);
            j += 1;
        }
    }

    data_height(data).to_string()
}

fn part_two(
    rock_types: Vec<RockType>,
    data: &mut HashSet<(usize, usize)>,
    jets: Vec<char>,
) -> String {
    let mut states: Vec<(usize, usize, usize)> = Vec::new();
    states.push((0, 0, 0));
    let jets_number = jets.len();
    let mut j = 0;
    let mut r = 0;
    let mut answer = None;
    while answer == None {
        let mut rock = Rock::new(rock_types[r % 5], data_height(data));

        let mut falling = true;
        while falling {
            let rock_move_char = jets[j % jets_number];
            j += 1;
            let mut rock_move = RockMove::Right;
            if rock_move_char == '<' {
                rock_move = RockMove::Left
            }
            rock.moves(rock_move, data);
            falling = rock.moves(RockMove::Down, data);
        }
        r += 1;
        states.push((r % 5, j % jets_number, data_height(data)));
        answer = find_answer(&states);
    }
    answer.unwrap().to_string()
}

fn data_height(data: &mut HashSet<(usize, usize)>) -> usize {
    data.iter().map(|p| p.1).max().unwrap_or(0)
}

fn find_answer(states: &Vec<(usize, usize, usize)>) -> Option<usize> {
    let i = states.len() - 1;
    let last = states[i];
    for old_i in 0..states.len() - 1 {
        let s = states[old_i];
        if s.0 == last.0 && s.1 == last.1 && (1000000000000 - i) % (i - old_i) == 0 {
            let total = last.2 + (last.2 - s.2) * (1000000000000 - i) / (i - old_i);
            return Some(total);
        }
    }
    None
}

pub fn day17_2(input: String) -> String {
    day17(input, part_two)
}
