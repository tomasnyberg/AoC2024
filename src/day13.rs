use std::{
    collections::HashMap,
    io::{self, Read},
};

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

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (gcd, x1, y1) = extended_gcd(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (gcd, x, y)
}

fn positive_extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (gcd, mut k, mut t) = extended_gcd(a, b);

    let b_step = b / gcd;
    let a_step = a / gcd;

    while k < 0 {
        k += b_step;
        t -= a_step;
    }
    while t < 0 {
        k -= b_step;
        t += a_step;
    }
    (gcd, k, t)
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
            let (mut prize_x, mut prize_y) = into_digits(prize);
            prize_x += 10000000000000;
            prize_y += 10000000000000;
            let x_works = (prize_x % gcd(a_x, b_x)) == 0;
            let y_works = (prize_y % gcd(a_y, b_y)) == 0;
            if !(x_works && y_works) {
                return 0;
            }
            let low = bs((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
            //let xremain = prize_x - low * a_x;
            //let yremain = prize_y - low * a_y;
            for add in -25..25 {
                let new_low = low + add;
                let new_xremain = prize_x - new_low * a_x;
                let new_yremain = prize_y - new_low * a_y;
                if new_xremain % b_x == 0
                    && new_yremain % b_y == 0
                    && new_xremain / b_x == new_yremain / b_y
                {
                    return new_low * 3 + (new_xremain / b_x);
                }
            }
            0
        })
        .sum::<i64>();
    println!("{}", res);
}
