use std::io::{self, Read};

pub fn safe_report(xs: Vec<i32>) -> i32 {
    let increasing: bool = xs.windows(2).all(|x| x[0] < x[1]);
    let decreasing: bool = xs.windows(2).all(|x| x[0] > x[1]);
    let max_d: bool = xs.windows(2).all(|x| (x[0] - x[1]).abs() <= 3);
    ((increasing || decreasing) && max_d) as i32
}
pub fn part_one(xss: &Vec<Vec<i32>>) -> i32 {
    xss.iter().map(|xs| safe_report(xs.to_vec())).sum()
}

pub fn part_two(xss: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    xss.iter().for_each(|xs| {
        for i in 0..xs.len() {
            let _xs_without: Vec<i32> = xs
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, x)| *x)
                .collect();
            if safe_report(_xs_without) == 1 {
                result += 1;
                break;
            }
        }
    });
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let xss: Vec<Vec<i32>> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!("{}", part_one(&xss));
    println!("{}", part_two(xss));
}
