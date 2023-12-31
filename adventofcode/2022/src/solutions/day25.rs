use core::panic;

pub const EXAMPLE_ANSWER1: &str = "2=-1=0";
pub const EXAMPLE_ANSWER2: &str = "12";

struct Snafu {
    data: Vec<i8>,
}

impl Snafu {
    fn from(s: &str) -> Snafu {
        let char_to_number = |c: char| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown value"),
        };
        let mut data = Vec::new();
        s.chars().for_each(|c| data.push(char_to_number(c)));
        data.reverse();
        Snafu { data }
    }

    fn to_string(&self) -> String {
        let number_to_char = |n: i8| match n {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("unknown value"),
        };
        self.data
            .iter()
            .map(|n| number_to_char(*n).to_string())
            .rev()
            .reduce(|acc, e| acc + &e)
            .unwrap()
    }

    fn add(&mut self, other: Snafu) {
        let d = &mut self.data;
        let o_d = other.data;
        let mut higher = 0;
        for i in 0..d.len().max(o_d.len()) {
            let v_d = if i < d.len() { d[i] } else { 0 };
            let v_od = if i < o_d.len() { o_d[i] } else { 0 };
            let mut sum = v_d + v_od + higher;
            higher = 0;
            if sum > 2 {
                higher += 1;
                sum -= 5;
            } else if sum < -2 {
                higher -= 1;
                sum += 5;
            }
            if i < d.len() {
                d[i] = sum;
            } else {
                d.push(sum);
            }
        }
        if higher > 0 {
            d.push(higher);
        }
    }
}
pub fn day25_1(input: String) -> String {
    let mut total = Snafu::from("0");
    for line in input.lines() {
        total.add(Snafu::from(line));
    }
    total.to_string()
}
pub fn day25_2(input: String) -> String {
    input
}
