use std::{
    collections::HashMap,
    collections::HashSet,
    io::{self, Read},
};

fn mirror(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let di = b.0 - a.0;
    let dj = b.1 - a.1;
    ((a.0 + 2 * di, a.1 + 2 * dj), (b.0 - 2 * di, b.1 - 2 * dj))
}

fn resonance_mirror(a: (i32, i32), b: (i32, i32), n: i32, m: i32) -> HashSet<(i32, i32)> {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let di = b.0 - a.0;
    let dj = b.1 - a.1;
    let mut i = a.0;
    let mut j = a.1;
    while i >= 0 && j >= 0 && i < n && j < m {
        seen.insert((i, j));
        i += di;
        j += dj;
    }
    i = b.0;
    j = b.1;
    while i >= 0 && j >= 0 && i < n && j < m {
        seen.insert((i, j));
        i -= di;
        j -= dj;
    }
    seen
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
    let mut seen_p1: HashSet<(i32, i32)> = HashSet::new();
    let mut seen_p2: HashSet<(i32, i32)> = HashSet::new();
    for (_k, v) in locations.iter() {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (a, b) = mirror(v[i], v[j]);
                seen_p1.insert(a);
                seen_p1.insert(b);
                seen_p2.extend(resonance_mirror(v[i], v[j], n, m));
            }
        }
    }
    seen_p1.retain(|&(i, j)| i >= 0 && j >= 0 && i < n && j < m);
    println!("{:?}", seen_p1.len());
    println!("{:?}", seen_p2.len());
}
