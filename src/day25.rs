use std::io::{self, Read};

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut locks: Vec<u32> = Vec::new();
    let mut keys: Vec<u32> = Vec::new();
    for p in parts.iter() {
        let bits = p.as_bytes()[6..35]
            .iter()
            .fold(0, |acc, &b| acc << 1 | (b == b'#') as u32);
        if p.starts_with('#') {
            locks.push(bits);
        } else {
            keys.push(bits);
        }
    }
    let mut result = 0;
    for lock in &locks {
        for key in &keys {
            result += (lock & key == 0) as u32;
        }
    }
    println!("{}", result);
}
