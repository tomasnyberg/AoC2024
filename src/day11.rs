use std::io::{self, Read};

pub fn digit_count(mut x: i64) -> i32 {
    let mut result = 0;
    while x > 0 {
        result += 1;
        x /= 10;
    }
    result
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
        for i in 0..input.len() {
            if input[i] == 0 {
                input[i] = 1;
                continue;
            }
            let dc = digit_count(input[i]);
            if dc % 2 == 0 {
                let mut new_num = 0;
                let mut pos = 1;
                for _ in 0..(dc / 2) {
                    new_num += (input[i] % 10) * pos;
                    pos *= 10;
                    input[i] /= 10;
                }
                input.push(new_num);
            } else {
                input[i] *= 2024;
            }
        }
    }
    println!("{:?}", input.len());
}
