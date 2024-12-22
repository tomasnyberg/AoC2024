use std::io::{self, Read};

fn evolve(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut result = 0;
    for num in nums {
        let mut curr = num;
        for _ in 0..2000 {
            curr = evolve(curr);
        }
        result += curr;
    }
    println!("{}", result);
}
