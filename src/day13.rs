use std::{
    collections::HashMap,
    io::{self, Read},
};

const BAD_VAL: i32 = 10000000;

fn into_digits(s: &str) -> (i32, i32) {
    let parts: Vec<i32> = s
        .split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|x| x[2..].parse::<i32>().unwrap())
        .collect();
    (parts[0], parts[1])
}

fn dp(
    target: (i32, i32),
    a: (i32, i32),
    b: (i32, i32),
    curr: (i32, i32),
    seen: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    if curr == target {
        return 0;
    }
    if seen.contains_key(&curr) {
        return seen[&curr];
    }
    if curr.0 > target.0 || curr.1 > target.1 {
        return BAD_VAL;
    }
    let take_a = 3 + dp(target, a, b, (curr.0 + a.0, curr.1 + a.1), seen);
    let take_b = 1 + dp(target, a, b, (curr.0 + b.0, curr.1 + b.1), seen);
    let result = BAD_VAL.min(take_a).min(take_b);
    seen.insert(curr, take_a.min(take_b));
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let res = input
        .split("\n\n")
        .map(|parts| {
            let x: Vec<&str> = parts.split('\n').collect();
            let (a, b, prize) = (x[0], x[1], x[2]);
            let (a_x, a_y) = into_digits(a);
            let (b_x, b_y) = into_digits(b);
            let (prize_x, prize_y) = into_digits(prize);
            let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
            let res = dp(
                (prize_x, prize_y),
                (a_x, a_y),
                (b_x, b_y),
                (0, 0),
                &mut seen,
            );
            if res < BAD_VAL {
                res
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("{}", res);
}
