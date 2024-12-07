use std::io::{self, Read};

fn part_one(xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    targets
        .iter()
        .zip(xss.iter())
        .map(|(target, xs)| {
            if dfs(*target, 1, xs, xs[0], true) {
                return target;
            }
            &0
        })
        .sum::<i64>()
}

fn part_two(xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    targets
        .iter()
        .zip(xss.iter())
        .map(|(target, xs)| {
            if dfs(*target, 1, xs, xs[0], false) {
                return target;
            }
            &0
        })
        .sum::<i64>()
}

fn concat_vals(mut a: i64, b: i64) -> i64 {
    if b >= 100 {
        a *= 1000;
    } else if b >= 10 {
        a *= 100;
    } else {
        a *= 10;
    }
    a + b
}

fn dfs(target: i64, i: usize, matrix: &Vec<i64>, running: i64, part_one: bool) -> bool {
    if i == matrix.len() && running == target {
        return true;
    }
    if i == matrix.len() || running > target {
        return false;
    }
    let new_con_val = concat_vals(running, matrix[i]);
    let con = !part_one && dfs(target, i + 1, matrix, new_con_val, part_one);
    let add = dfs(target, i + 1, matrix, running + matrix[i], part_one);
    let mul = dfs(target, i + 1, matrix, running * matrix[i], part_one);
    con || add || mul
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let split: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            )
        })
        .collect();
    let xss: Vec<Vec<i64>> = split
        .iter()
        .map(|(_, s)| {
            s.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let targets: Vec<i64> = split
        .iter()
        .map(|(s, _)| s.parse::<i64>().unwrap())
        .collect();
    let part_one_result = part_one(&xss, &targets);
    let part_two_result = part_two(&xss, &targets);
    println!("{}", part_one_result);
    println!("{}", part_two_result);
}
