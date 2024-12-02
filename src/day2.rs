use std::io::{self, Read};

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
    let mut result = 0;
    xss.iter().for_each(|xs| {
        let increasing: bool = xs.windows(2).all(|x| x[0] < x[1]);
        let decreasing: bool = xs.windows(2).all(|x| x[0] > x[1]);
        let max_d: bool = xs.windows(2).all(|x| (x[0] - x[1]).abs() <= 3);
        result += ((increasing || decreasing) && max_d) as i32;
    });
    println!("{}", result);
}
