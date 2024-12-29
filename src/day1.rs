use std::collections::HashMap;

fn part_one(a: &Vec<i32>, b: &Vec<i32>) {
    let result: i32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
    println!("{}", result);
}

fn part_two(a: Vec<i32>, b: Vec<i32>) {
    let mut hm = HashMap::<i32, i32>::new();
    b.iter().for_each(|x| {
        *hm.entry(*x).or_insert(0) += 1;
    });
    let result: i32 = a.iter().map(|x| x * hm.get(x).unwrap_or(&0)).sum();
    println!("{}", result);
}

pub fn solve(input: String) {
    let xs: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut a: Vec<i32> = xs.iter().step_by(2).map(|x| *x).collect();
    let mut b: Vec<i32> = xs.iter().skip(1).step_by(2).map(|x| *x).collect();
    a.sort();
    b.sort();
    part_one(&a, &b);
    part_two(a, b);
}
