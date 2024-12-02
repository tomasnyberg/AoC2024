use std::io::{self, Read};

pub fn part_one(xss: Vec<Vec<i32>>) -> i32 {
    xss.iter()
        .map(|xs| {
            let increasing: bool = xs.windows(2).all(|x| x[0] < x[1]);
            let decreasing: bool = xs.windows(2).all(|x| x[0] > x[1]);
            let max_d: bool = xs.windows(2).all(|x| (x[0] - x[1]).abs() <= 3);
            ((increasing || decreasing) && max_d) as i32
        })
        .sum()
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
    println!("{}", part_one(xss));
}
