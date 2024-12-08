use std::{
    collections::HashMap,
    collections::HashSet,
    io::{self, Read},
};

fn part_one() {}

fn part_two() {}

fn mirror(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let di = b.0 - a.0;
    let dj = b.1 - a.1;
    let c = (a.0 + 2 * di, a.1 + 2 * dj);
    let d = (b.0 - 2 * di, b.1 - 2 * dj);
    (c, d)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let split: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n = split.len() as i32;
    let m = split[0].len() as i32;
    let mut locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..split.len() {
        for j in 0..split[0].len() {
            if split[i][j] != '.' {
                locations
                    .entry(split[i][j])
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }
    }
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for (_k, v) in locations.iter() {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (a, b) = mirror(v[i], v[j]);
                seen.insert(a);
                seen.insert(b);
            }
        }
    }
    seen.retain(|&(i, j)| i >= 0 && j >= 0 && i < n && j < m);
    println!("{:?}", seen.len());
}
