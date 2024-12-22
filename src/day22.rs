use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn evolve(mut secret: i64) -> i64 {
    secret ^= secret << 6 & 0xFFFFFF;
    secret ^= secret >> 5 & 0xFFFFFF;
    secret ^ secret << 11 & 0xFFFFFF
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut totals: HashMap<i64, i64> = HashMap::new();
    let mut part_one = 0;
    for num in nums {
        let mut curr = num;
        let mut prev_ones = curr % 10;
        let mut seen: HashSet<i64> = HashSet::new();
        let mut curr_key = 0;
        for i in 0..2000 {
            curr = evolve(curr);
            let ones = curr % 10;
            let diff = ones - prev_ones;
            curr_key <<= 5;
            curr_key += diff;
            curr_key &= (1 << 20) - 1;
            prev_ones = ones;
            if i > 3 && !seen.contains(&curr_key) {
                totals.insert(curr_key, *totals.get(&curr_key).unwrap_or(&0) + ones);
                seen.insert(curr_key);
            }
        }
        part_one += curr;
    }
    println!("{}", part_one);
    println!("{}", totals.values().max().unwrap());
}
