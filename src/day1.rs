use std::collections::HashMap;
use std::io::{self, Read};

fn part_one(a: &Vec<i32>, b: &Vec<i32>) {
    let mut result = 0;
    for i in 0..a.len() {
        result += (a[i] - b[i]).abs();
    }
    println!("{}", result);
}

fn part_two(a: Vec<i32>, b: Vec<i32>) {
    let mut hm = HashMap::<i32, i32>::new();
    for i in 0..b.len() {
        *hm.entry(b[i]).or_insert(0) += 1;
    }
    let mut result = 0;
    for i in 0..a.len() {
        result += a[i] * hm.get(&a[i]).unwrap_or(&0);
    }
    println!("{}", result);
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
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
