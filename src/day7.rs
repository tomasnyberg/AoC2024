use std::io::{self, Read};

fn part_one(xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    targets
        .iter()
        .zip(xss.iter())
        .map(|(target, xs)| {
            if dfs(*target, 1, xs, xs[0]) {
                return target;
            }
            &0
        })
        .sum::<i64>()
}

fn part_two() {}

fn dfs(target: i64, i: usize, matrix: &Vec<i64>, running: i64) -> bool {
    if i == matrix.len() && running == target {
        return true;
    }
    if i == matrix.len() || running > target {
        return false;
    }
    let add = dfs(target, i + 1, matrix, running + matrix[i]);
    let mul = dfs(target, i + 1, matrix, running * matrix[i]);
    add || mul
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
    println!("{}", part_one_result);
}
