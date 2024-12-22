use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    let mut totals: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    let mut part_one = 0;
    for num in nums {
        let mut curr = num;
        let mut prev_ones = curr % 10;
        let mut dq: VecDeque<i64> = VecDeque::new();
        let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        let mut curr_key = (-11, -11, -11, -11);
        for _ in 0..2000 {
            curr = evolve(curr);
            let ones = curr % 10;
            let diff = ones - prev_ones;
            prev_ones = ones;
            dq.push_back(diff);
            if dq.len() > 4 {
                dq.pop_front();
            }
            if dq.len() == 4 {
                curr_key = (dq[0], dq[1], dq[2], dq[3]);
            }
            if curr_key != (-11, -11, -11, -11) && !seen.contains(&curr_key) {
                totals.insert(curr_key, *totals.get(&curr_key).unwrap_or(&0) + ones);
                seen.insert(curr_key);
            }
        }
        part_one += curr;
    }
    println!("{}", part_one);
    println!("{}", totals.values().max().unwrap());
}
