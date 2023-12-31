use std::collections::HashMap;

use regex::Regex;
pub const EXAMPLE_ANSWER1: &str = "95437";
pub const EXAMPLE_ANSWER2: &str = "24933642";
#[derive(Debug)]
struct Directory {
    name: String,
    parent_name: String,
    files: Vec<(String, usize)>,
    total_size_with_children: usize,
}
impl Directory {
    fn files_size(&self) -> usize {
        self.files.iter().map(|(_n, s)| s).sum()
    }
}
fn build_directory(name: &str, parent_name: &str) -> Directory {
    Directory {
        name: name.to_string(),
        parent_name: parent_name.to_string(),
        files: Vec::new(),
        total_size_with_children: 0,
    }
}

fn build_tree(input: String) -> HashMap<String, Directory> {
    let mut tree = HashMap::new();
    let mut current_node_key = "None".to_string();
    let re_moving_directory = Regex::new(r"^\$ cd (.*)$").unwrap();
    let re_file_size_name = Regex::new(r"^([0-9]+) (.+)$").unwrap();
    for line in input.lines() {
        if line == "$ cd /" {
            tree.insert("/".to_string(), build_directory("/", "None"));
            current_node_key = "/".to_string();
        } else if line == "$ cd .." {
            let current_node_value = tree.get_mut(&current_node_key).unwrap();
            current_node_key = current_node_value.parent_name.clone();
        } else if re_moving_directory.is_match(line) {
            let current_node_value = tree.get_mut(&current_node_key).unwrap();
            let caps = re_moving_directory.captures(&line).unwrap();
            let child_name: String = caps[1].parse().unwrap();
            let child_full_name = current_node_value.name.to_owned() + &child_name + "/";
            tree.insert(
                child_full_name.clone(),
                build_directory(&child_full_name, &current_node_key),
            );
            current_node_key = child_full_name;
        } else if re_file_size_name.is_match(line) {
            let current_node_value = tree.get_mut(&current_node_key).unwrap();
            let caps = re_file_size_name.captures(&line).unwrap();
            let file_size: usize = caps[1].parse().unwrap();
            let file_name: String = caps[2].parse().unwrap();
            current_node_value.files.push((file_name, file_size));
        }
    }

    let mut keys: Vec<String> = tree.keys().map(|k| k.clone()).collect();
    keys.sort_by(|a, b| number_slash(b).cmp(&number_slash(a)));
    for k in keys {
        let node = tree.get_mut(&k).unwrap();
        let file_size = node.files_size();
        node.total_size_with_children += file_size;
        let total_size = node.total_size_with_children;
        let parent_name = node.parent_name.clone();
        if parent_name != "None" {
            let parent_node = tree.get_mut(&parent_name).unwrap();
            parent_node.total_size_with_children += total_size;
        }
    }

    tree
}

fn number_slash(s: &String) -> usize {
    let mut answer = 0;
    s.to_owned().chars().for_each(|c| {
        if c == '/' {
            answer += 1
        }
    });
    answer
}

pub fn day7_1(input: String) -> String {
    let tree = build_tree(input);
    let answer: usize = tree
        .values()
        .map(|n| n.total_size_with_children)
        .filter(|size| size <= &100000)
        .sum();
    answer.to_string()
}
pub fn day7_2(input: String) -> String {
    let tree = build_tree(input);
    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = tree.get("/").unwrap().total_size_with_children;
    let available_space = total_space - used_space;
    let mut answer = 0;
    if available_space < needed_space {
        let min_to_delete = needed_space - available_space;
        answer = tree
            .values()
            .map(|d| d.total_size_with_children)
            .filter(|s| s >= &min_to_delete)
            .min()
            .unwrap();
    }
    answer.to_string()
}
