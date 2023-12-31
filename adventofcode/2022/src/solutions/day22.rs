use core::panic;
use regex::Regex;
use std::collections::HashMap;

pub const EXAMPLE_ANSWER1: &str = "6032";
pub const EXAMPLE_ANSWER2: &str = "5031";

fn read_input<'a>(input: &'a String) -> (HashMap<(usize, usize), char>, Vec<&'a str>) {
    let mut board: HashMap<(usize, usize), char> = HashMap::new();
    let mut instructions: Vec<&str> = Vec::new();
    let mut x = 1;
    for line in input.lines() {
        if line.contains(".") || line.contains("#") {
            for (y, value) in line.char_indices() {
                if value != ' ' {
                    board.insert((x, y + 1), value);
                }
            }
            x += 1
        } else if line.contains("R") || line.contains("L") {
            let re_instructions = Regex::new(r"([0-9]+)([R|L]*)").unwrap();
            for (_, [number, dir]) in re_instructions.captures_iter(line).map(|c| c.extract()) {
                instructions.push(number);
                if !dir.is_empty() {
                    instructions.push(dir);
                }
            }
        }
    }
    (board, instructions)
}

fn executes(
    board: &HashMap<(usize, usize), char>,
    instructions: &Vec<&str>,
    find_next: &dyn Fn(&(usize, usize, usize)) -> (usize, usize, usize),
) -> String {
    // ['>', 'v', '<', '^']
    let start_y = board
        .iter()
        .filter(|(k, v)| **v == '.' && k.0 == 1)
        .map(|(k, _v)| k.1)
        .min()
        .unwrap();
    let mut cursor: (usize, usize, usize) = (1, start_y, 0);
    for i in instructions {
        if *i == "R" {
            cursor.2 = (cursor.2 + 1) % 4;
        } else if *i == "L" {
            cursor.2 = (cursor.2 + 4 - 1) % 4;
        } else {
            let step_number = (*i).parse::<usize>().unwrap();
            for _ in 0..step_number {
                let next_cursor = find_next(&cursor);
                if board.get(&(next_cursor.0, next_cursor.1)).unwrap() == &'.' {
                    cursor = next_cursor;
                } else {
                    break;
                }
            }
        }
    }
    (cursor.0 * 1000 + cursor.1 * 4 + cursor.2).to_string()
}

pub fn day22_1(input: String) -> String {
    let (board, instructions) = read_input(&input);
    let find_next = |cursor: &(usize, usize, usize)| {
        let mut x = cursor.0;
        let mut y = cursor.1;
        let facing = cursor.2;
        match facing {
            0 => {
                if board.contains_key(&(x, y + 1)) {
                    y += 1;
                } else {
                    y = board
                        .keys()
                        .filter(|k| k.0 == x)
                        .map(|k| k.1)
                        .min()
                        .unwrap();
                }
            }
            1 => {
                if board.contains_key(&(x + 1, y)) {
                    x += 1;
                } else {
                    x = board
                        .keys()
                        .filter(|k| k.1 == y)
                        .map(|k| k.0)
                        .min()
                        .unwrap();
                }
            }
            2 => {
                if board.contains_key(&(x, y - 1)) {
                    y -= 1;
                } else {
                    y = board
                        .keys()
                        .filter(|k| k.0 == x)
                        .map(|k| k.1)
                        .max()
                        .unwrap();
                }
            }
            3 => {
                if board.contains_key(&(x - 1, y)) {
                    x -= 1;
                } else {
                    x = board
                        .keys()
                        .filter(|k| k.1 == y)
                        .map(|k| k.0)
                        .max()
                        .unwrap();
                }
            }
            _ => {
                panic!("facing out of range")
            }
        }
        (x, y, facing)
    };
    executes(&board, &instructions, &find_next)
}

#[derive(Debug)]
struct Face {
    _id: usize,
    pub size: usize,
    pub top_left: (usize, usize),
    pub facings: Vec<Option<(usize, usize)>>,
}

impl Face {
    fn new(id: usize, size: usize, top_left: (usize, usize)) -> Face {
        Face {
            _id: id,
            size,
            top_left,
            facings: vec![None, None, None, None],
        }
    }
    fn contains(&self, position: (usize, usize)) -> bool {
        let (top_x, top_y) = self.top_left;
        position.0 >= top_x
            && position.0 < top_x + self.size
            && position.1 >= top_y
            && position.1 < top_y + self.size
    }
    fn relative_coords(&self, coords: (usize, usize)) -> (usize, usize) {
        (coords.0 - self.top_left.0, coords.1 - self.top_left.1)
    }
    fn absolute_coords(&self, relative_coords: (usize, usize)) -> (usize, usize) {
        (
            &self.top_left.0 + relative_coords.0,
            &self.top_left.1 + relative_coords.1,
        )
    }
}
fn cube_swaps(board: &HashMap<(usize, usize), char>) -> Vec<Face> {
    // find cube faces
    let mut faces: Vec<Face> = Vec::new();
    let size = ((board.len() / 6) as f64).sqrt() as usize;
    let mut face_id = 1;
    'find_faces: for x in 0..5 {
        let mut all_y: Vec<usize> = board
            .keys()
            .filter(|k| k.0 == x * size + 1)
            .map(|k| k.1)
            .collect();
        all_y.sort();
        if all_y.len() > 0 {
            let min_y = all_y[0];
            for i in 0..all_y.len() / size {
                faces.push(Face::new(face_id, size, (x * size + 1, min_y + i * size)));
                if face_id == 6 {
                    break 'find_faces;
                }
                face_id += 1;
            }
        }
    }
    // direct relationships between faces
    for i in 1..7 {
        let (x, y) = faces[i - 1].top_left;
        for j in 1..7 {
            if j != i {
                let other_face = &faces[j - 1];
                let (other_x, other_y) = other_face.top_left;
                if x == other_x {
                    if y + size == other_y {
                        faces[i - 1].facings[0] = Some((j, 0));
                    } else if y == other_y + size {
                        faces[i - 1].facings[2] = Some((j, 2));
                    }
                } else if y == other_y {
                    if x + size == other_x {
                        faces[i - 1].facings[1] = Some((j, 1));
                    } else if other_x + size == x {
                        faces[i - 1].facings[3] = Some((j, 3));
                    }
                }
            }
        }
    }
    // indirect relationships...
    let mut added = true;
    while added {
        added = false;
        for i in 1..7 {
            let facings = faces[i - 1].facings.clone();
            let mut compute_facing = |facing: usize, from: usize| {
                if faces[i - 1].facings[facing] == None {
                    if let Some((intermediate_id, intermediate_facing)) = facings[from] {
                        let mut final_facing = intermediate_facing;
                        let gap = (intermediate_facing + 4 - from) % 4;
                        let facing_to_find = (facing + gap) % 4;
                        if let Some((target_id, target_facing)) =
                            faces[intermediate_id - 1].facings[facing_to_find]
                        {
                            let gap = (target_facing + 4 - facing_to_find) % 4;
                            final_facing = (final_facing + gap) % 4;
                            faces[i - 1].facings[facing] = Some((target_id, final_facing));
                            added = true;
                        }
                    }
                }
            };
            if facings[3] == None && (facings[2] != None || facings[0] != None) {
                compute_facing(3, 2);
                compute_facing(3, 0);
            }
            if facings[1] == None && (facings[2] != None || facings[0] != None) {
                compute_facing(1, 2);
                compute_facing(1, 0);
            }
            if facings[2] == None && (facings[3] != None || facings[1] != None) {
                compute_facing(2, 3);
                compute_facing(2, 1);
            }
            if facings[0] == None && (facings[3] != None || facings[1] != None) {
                compute_facing(0, 3);
                compute_facing(0, 1);
            }
        }
    }
    faces
}

fn cube_next(faces: &Vec<Face>, cursor: &(usize, usize, usize)) -> (usize, usize, usize) {
    let (old_x, old_y, old_facing) = *cursor;
    let current_face = faces.iter().find(|f| f.contains((old_x, old_y))).unwrap();
    let size = current_face.size;
    let relative_coords = current_face.relative_coords((old_x, old_y));
    if let Some((target_face_id, facing)) = current_face.facings[old_facing] {
        let rotate_coords = match (facing + 4 - old_facing) % 4 {
            0 => {
                if facing % 2 == 0 {
                    (relative_coords.0, size - 1 - relative_coords.1)
                } else {
                    (size - 1 - relative_coords.0, relative_coords.1)
                }
            }
            1 => match facing {
                0 => (relative_coords.1, 0),
                1 => (0, size - 1 - relative_coords.0),
                2 => (relative_coords.1, size - 1),
                3 => (size - 1, size - 1 - relative_coords.0),
                _ => panic!(""),
            },
            2 => {
                if facing % 2 == 0 {
                    (size - 1 - relative_coords.0, relative_coords.1)
                } else {
                    (relative_coords.0, size - 1 - relative_coords.1)
                }
            }
            3 => match facing {
                0 => (size - 1 - relative_coords.1, 0),
                1 => (0, relative_coords.0),
                2 => (size - 1 - relative_coords.1, size - 1),
                3 => (size - 1, relative_coords.0),
                _ => panic!(""),
            },
            _ => panic!("facing > 3"),
        };
        let (x, y) = faces[target_face_id - 1].absolute_coords(rotate_coords);
        return (x, y, facing);
    } else {
        panic!("None facing");
    }
}

pub fn day22_2(input: String) -> String {
    let (board, instructions) = read_input(&input);
    let faces = cube_swaps(&board);
    let find_next = |cursor: &(usize, usize, usize)| {
        let (x, y, facing) = *cursor;
        match facing {
            0 => {
                if board.contains_key(&(x, y + 1)) {
                    (x, y + 1, facing)
                } else {
                    cube_next(&faces, &cursor)
                }
            }
            1 => {
                if board.contains_key(&(x + 1, y)) {
                    (x + 1, y, facing)
                } else {
                    cube_next(&faces, &cursor)
                }
            }
            2 => {
                if board.contains_key(&(x, y - 1)) {
                    (x, y - 1, facing)
                } else {
                    cube_next(&faces, &cursor)
                }
            }
            3 => {
                if board.contains_key(&(x - 1, y)) {
                    (x - 1, y, facing)
                } else {
                    cube_next(&faces, &cursor)
                }
            }
            _ => {
                panic!("facing out of range")
            }
        }
    };
    executes(&board, &instructions, &find_next)
}
