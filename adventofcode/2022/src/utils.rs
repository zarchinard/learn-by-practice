use reqwest::{self, header};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn read_input_to_string(folder: &str, day: &str, cookie: &str) -> String {
    let file_path = format!("{}/{}.txt", folder, day);
    if folder == "inputs" && !Path::new(&file_path).exists() {
        println!("Downloading day {} input...", day);
        retrieve_input(day, cookie);
    }
    fs::read_to_string(file_path).unwrap().to_string()
}

fn retrieve_input(day: &str, cookie: &str) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(cookie).unwrap(),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    let res = client.get(&url).send().unwrap();

    let path = format!("inputs/{}.txt", day);
    let mut output = File::create(path).unwrap();
    write!(output, "{}", res.text().unwrap()).ok();
}

fn send_answer(day: &str, level: &str, cookie: &str, answer: String) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(cookie).unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("https://adventofcode.com/2022/day/{}/answer", day);
    let body = format!("level={}&answer={}", level, answer);
    client.post(url).body(body).send().unwrap();
}

pub struct Puzzle {
    pub day: String,
    pub level: String,
    pub example_answer: String,
    pub solve: fn(String) -> String,
}

impl Puzzle {
    fn test(&self) -> bool {
        let example_input = read_input_to_string("examples", &self.day, "no need");
        let solve = self.solve;
        solve(example_input) == self.example_answer
    }

    pub fn answer(&self, cookie: &str, send: bool) {
        if self.test() {
            let input = read_input_to_string("inputs", &self.day, cookie);
            let solve = self.solve;
            let answer = solve(input);
            if send {
                println!("sending answer {}", answer);
                send_answer(&self.day, &self.level, cookie, answer);
            } else {
                println!("{}", answer);
            }
        } else {
            println!("test failed");
        }
    }
}
