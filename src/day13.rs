use std::io::{self, Read};

fn into_digits(s: &str) -> (i64, i64) {
    let parts: Vec<i64> = s
        .split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|x| x[2..].parse::<i64>().unwrap())
        .collect();
    (parts[0], parts[1])
}

pub fn bs(av: (i64, i64), bv: (i64, i64), prize: (i64, i64)) -> i64 {
    let mut low = 0;
    let mut high = 10000000000000;
    while low < high {
        let mid = low + (high - low) / 2;
        let (x, y) = ((prize.0 - mid * av.0) / bv.0, (prize.1 - mid * av.1) / bv.1);
        if (x < 0 || y < 0) || (av.0 < av.1 && x > y) || (av.0 > av.1 && x < y) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

pub fn find_combo(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> i64 {
    let low = bs(a, b, prize);
    let mut result = 0;
    for add in -100..100 {
        let new_low = low + add;
        let xremain = prize.0 - new_low * a.0;
        let yremain = prize.1 - new_low * a.1;
        if xremain % b.0 == 0
            && yremain % b.1 == 0
            && xremain / b.0 == yremain / b.1
            && xremain >= 0
            && yremain >= 0
        {
            let res = new_low * 3 + (xremain / b.0);
            if result == 0 {
                result = res;
            }
            result = result.min(res);
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut part_one = 0;
    let mut part_two = 0;
    input.split("\n\n").for_each(|parts| {
        let x: Vec<&str> = parts.split('\n').collect();
        let (a, b, prize) = (x[0], x[1], x[2]);
        let (a_x, a_y) = into_digits(a);
        let (b_x, b_y) = into_digits(b);
        let (mut prize_x, mut prize_y) = into_digits(prize);
        part_one += find_combo((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
        prize_x += 10000000000000;
        prize_y += 10000000000000;
        part_two += find_combo((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
    });
    println!("{}", part_one);
    println!("{}", part_two);
}
