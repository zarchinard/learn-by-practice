use std::collections::HashSet;

pub const EXAMPLE_ANSWER1: &str = "21";
pub const EXAMPLE_ANSWER2: &str = "8";

fn build_grid(input: String) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let new_line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(new_line);
    }
    grid
}

fn count_visibles(grid: Vec<Vec<u32>>) -> HashSet<(usize, usize, u32)> {
    let line_numbers = grid.len();
    let col_numbers = grid[0].len();

    let mut visibles = HashSet::<(usize, usize, u32)>::new();

    for i in 0..line_numbers {
        let mut max = grid[i][0];
        visibles.insert((i, 0, max));
        for j in 1..col_numbers {
            let v = grid[i][j];
            if v > max {
                visibles.insert((i, j, v));
                max = v;
            }
        }
        max = grid[i][col_numbers - 1];
        visibles.insert((i, col_numbers - 1, max));
        for j in (0..col_numbers - 1).rev() {
            let v = grid[i][j];
            if v > max {
                visibles.insert((i, j, v));
                max = v;
            }
        }
    }
    for j in 0..col_numbers {
        let mut max = grid[0][j];
        visibles.insert((0, j, max));
        for i in 1..line_numbers {
            let v = grid[i][j];
            if v > max {
                visibles.insert((i, j, v));
                max = v;
            }
        }
        max = grid[line_numbers - 1][j];
        visibles.insert((line_numbers - 1, j, max));
        for i in (0..line_numbers - 1).rev() {
            let v = grid[i][j];
            if v > max {
                visibles.insert((i, j, v));
                max = v;
            }
        }
    }
    visibles
}

pub fn day8_1(input: String) -> String {
    let grid = build_grid(input);
    let visibles = count_visibles(grid);
    visibles.len().to_string()
}

pub fn day8_2(input: String) -> String {
    let grid = build_grid(input);
    let line_number = grid.len();
    let col_number = grid[0].len();

    fn count_trees(size: u32, view: Vec<u32>) -> u32 {
        let mut count: u32 = 0;
        for tree in view {
            count += 1;
            if tree >= size {
                break;
            }
        }
        count
    }

    let scenic_score = |i: usize, j: usize| -> u32 {
        let mut result: u32 = 0;
        if i != 0 && j != 0 {
            let size = grid[i][j];
            let mut up_view = Vec::new();
            for k in (0..i).rev() {
                up_view.push(grid[k][j]);
            }
            let mut down_view = Vec::new();
            for k in i + 1..line_number {
                down_view.push(grid[k][j]);
            }
            let mut left_view = Vec::new();
            for k in (0..j).rev() {
                left_view.push(grid[i][k]);
            }
            let mut right_view = Vec::new();
            for k in j + 1..col_number {
                right_view.push(grid[i][k]);
            }
            result = count_trees(size, up_view)
                * count_trees(size, down_view)
                * count_trees(size, left_view)
                * count_trees(size, right_view);
        }
        result
    };

    let mut max = 0;
    for i in 0..line_number {
        for j in 0..col_number {
            let score = scenic_score(i, j);
            if score > max {
                max = score;
            }
        }
    }

    max.to_string()
}
