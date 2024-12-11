use std::{
    collections::HashMap,
    io::{self, Read},
};

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

pub fn number_five_iters(x: i64) -> HashMap<i64, i64> {
    let mut xs = vec![x];
    for _ in 0..5 {
        one_iter(&mut xs);
    }
    let mut result = HashMap::new();
    for x in xs {
        *result.entry(x).or_insert(0) += 1;
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
    let mut known_combos: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    let mut counts: HashMap<i64, i64> = input.iter().map(|x| (*x, 1)).collect();
    for _ in 0..(75 / 5) {
        let mut new_counts: HashMap<i64, i64> = HashMap::new();
        for (k, v) in &counts {
            if !known_combos.contains_key(k) {
                known_combos.insert(*k, number_five_iters(*k));
            }
            for (k2, v2) in known_combos.get(k).unwrap() {
                *new_counts.entry(*k2).or_insert(0) += v * v2;
            }
        }
        counts = new_counts;
    }
    println!("{}", counts.values().sum::<i64>());
}
