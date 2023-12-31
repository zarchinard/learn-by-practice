pub const EXAMPLE_ANSWER1: &str = "3";
pub const EXAMPLE_ANSWER2: &str = "1623178306";

fn read_input(input: String, encryption_key: usize) -> Vec<(usize, isize)> {
    let mut size = 0;
    let mut data: Vec<(usize, isize)> = Vec::new();
    for line in input.lines() {
        data.push((size, line.parse().unwrap()));
        size += 1;
    }
    data.iter_mut()
        .for_each(|e| e.1 = e.1 * encryption_key as isize);
    data
}

fn change_index<T>(arr: &mut [T], old_index: usize, new_index: usize) {
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
    }
}

fn mixin(data: &mut Vec<(usize, isize)>) {
    let size = data.len();
    for i in 0..size {
        let index = data.iter().position(|&e| e.0 == i).unwrap();
        let v = data[index].1;
        let new_index = (index as isize + v).rem_euclid((size - 1) as isize);
        change_index(data, index, new_index as usize);
    }
}

pub fn day20_1(input: String) -> String {
    let mut data = read_input(input, 1);
    let size = data.len();
    mixin(&mut data);
    let mut answer = 0;
    let index_zero = data.iter().position(|&e| e.1 == 0).unwrap();
    answer += data[(index_zero + 1000) % size].1;
    answer += data[(index_zero + 2000) % size].1;
    answer += data[(index_zero + 3000) % size].1;

    answer.to_string()
}

pub fn day20_2(input: String) -> String {
    let mut data = read_input(input, 811589153);
    let size = data.len();
    for _i in 0..10 {
        mixin(&mut data);
    }
    let mut answer = 0;
    let index_zero = data.iter().position(|&e| e.1 == 0).unwrap();
    answer += data[(index_zero + 1000) % size].1;
    answer += data[(index_zero + 2000) % size].1;
    answer += data[(index_zero + 3000) % size].1;

    answer.to_string()
}
