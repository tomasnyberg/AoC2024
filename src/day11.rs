use std::io::{self, Read};

pub fn digit_count(mut x: i64) -> i32 {
    let mut result = 0;
    while x > 0 {
        result += 1;
        x /= 10;
    }
    result
}

pub fn one_iter(xs: &mut Vec<i64>) {
    for i in 0..xs.len() {
        if xs[i] == 0 {
            xs[i] = 1;
            continue;
        }
        let dc = digit_count(xs[i]);
        if dc % 2 == 0 {
            let mut new_num = 0;
            let mut pos = 1;
            for _ in 0..(dc / 2) {
                new_num += (xs[i] % 10) * pos;
                pos *= 10;
                xs[i] /= 10;
            }
            xs.push(new_num);
        } else {
            xs[i] *= 2024;
        }
    }
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input: Vec<i64> = input
        .trim()
        .split(' ')
        .map(|l| l.parse().unwrap())
        .collect();
    for _ in 0..25 {
        one_iter(&mut input);
    }
    println!("{:?}", input.len());
}
