pub const EXAMPLE_ANSWER1: &str = "64";
pub const EXAMPLE_ANSWER2: &str = "58";

fn manhattan_dist(a: (isize, isize, isize), b: (isize, isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

fn build_cubes(input: String) -> Vec<(isize, isize, isize)> {
    let mut cubes: Vec<(isize, isize, isize)> = Vec::new();
    for line in input.lines() {
        let array: Vec<isize> = line
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        cubes.push((array[0], array[1], array[2]));
    }
    cubes
}

pub fn day18_1(input: String) -> String {
    let cubes = build_cubes(input);
    let mut answer = cubes.len() * 6;
    for i in 0..cubes.len() - 1 {
        for j in i + 1..cubes.len() {
            if manhattan_dist(cubes[i], cubes[j]) == 1 {
                answer -= 2;
            }
        }
    }
    answer.to_string()
}

pub fn day18_2(input: String) -> String {
    let cubes = build_cubes(input);
    let (x, y, z) = optimums(&cubes);
    let mut steams: Vec<(isize, isize, isize)> = Vec::new();
    for i in x.0..x.1 + 1 {
        for j in y.0..y.1 + 1 {
            for k in z.0..z.1 + 1 {
                if !cubes.contains(&(i, j, k)) {
                    steams.push((i, j, k));
                }
            }
        }
    }

    let mut freed = true;
    while freed {
        freed = false;
        for i in (0..steams.len()).rev() {
            if is_free(&steams[i], &cubes, &steams) {
                steams.remove(i);
                freed = true;
            }
        }
    }
    let mut answer = cubes.len() * 6;
    for i in 0..cubes.len() {
        for j in i + 1..cubes.len() {
            if manhattan_dist(cubes[i], cubes[j]) == 1 {
                answer -= 2;
            }
        }
        for j in 0..steams.len() {
            if manhattan_dist(cubes[i], steams[j]) == 1 {
                answer -= 1;
            }
        }
    }

    answer.to_string()
}

fn is_free(
    steam: &(isize, isize, isize),
    cubes: &Vec<(isize, isize, isize)>,
    steams: &Vec<(isize, isize, isize)>,
) -> bool {
    let neighbours = vec![
        (steam.0 - 1, steam.1, steam.2),
        (steam.0 + 1, steam.1, steam.2),
        (steam.0, steam.1 - 1, steam.2),
        (steam.0, steam.1 + 1, steam.2),
        (steam.0, steam.1, steam.2 - 1),
        (steam.0, steam.1, steam.2 + 1),
    ];
    for n in neighbours.iter() {
        if !cubes.contains(n) && !steams.contains(n) {
            return true;
        }
    }
    false
}

fn optimums(
    cubes: &Vec<(isize, isize, isize)>,
) -> ((isize, isize), (isize, isize), (isize, isize)) {
    let c = cubes[0];
    let mut x = (c.0, c.0);
    let mut y = (c.1, c.1);
    let mut z = (c.2, c.2);
    for i in 1..cubes.len() {
        let c = cubes[i];
        if c.0 < x.0 {
            x.0 = c.0;
        }
        if c.0 > x.1 {
            x.1 = c.0;
        }
        if c.1 < y.0 {
            y.0 = c.1;
        }
        if c.1 > y.1 {
            y.1 = c.1;
        }
        if c.2 < z.0 {
            z.0 = c.2;
        }
        if c.2 > z.1 {
            z.1 = c.2;
        }
    }

    (x, y, z)
}
