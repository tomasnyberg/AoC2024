use rayon::prelude::*;

fn part_one(xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    sum_results(true, xss, targets)
}

fn part_two(xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    sum_results(false, xss, targets)
}

fn sum_results(part_one: bool, xss: &[Vec<i64>], targets: &[i64]) -> i64 {
    targets
        .par_iter()
        .zip(xss.par_iter())
        .map(|(target, xs)| {
            if dfs(*target, 1, xs, xs[0], part_one) {
                return target;
            }
            &0
        })
        .sum::<i64>()
}

fn concat_vals(a: i64, b: i64) -> i64 {
    (a * 10i64.pow(b.ilog10() + 1)) + b
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

pub fn solve(input: String) {
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
