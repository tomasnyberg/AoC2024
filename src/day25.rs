use std::io::{self, Read};

fn check(key: Vec<usize>, lock: Vec<usize>) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    parts.iter().for_each(|part| {
        let mut lock: Vec<Vec<char>> = Vec::new();
        part.lines().for_each(|line| {
            let mut lock_line: Vec<char> = Vec::new();
            line.chars().for_each(|c| {
                lock_line.push(c);
            });
            lock.push(lock_line);
        });
        grids.push(lock);
    });

    let mut locks: Vec<Vec<usize>> = Vec::new();
    for grid in grids.iter() {
        if grid[0][0] == '#' {
            let mut lock: Vec<usize> = Vec::new();
            for c in 0..grid[0].len() {
                let mut count = 0;
                for r in 1..grid.len() {
                    if grid[r][c] != '#' {
                        break;
                    }
                    count += 1;
                }
                lock.push(count);
            }
            locks.push(lock);
        }
    }
    let mut result = 0;
    for grid in grids.iter() {
        if grid[0][0] == '.' {
            let mut key: Vec<usize> = Vec::new();
            for c in 0..grid[0].len() {
                let mut count = 0;
                for r in (1..grid.len() - 1).rev() {
                    if grid[r][c] != '#' {
                        break;
                    }
                    count += 1;
                }
                key.push(count);
            }
            for lock in locks.iter() {
                if check(key.clone(), lock.clone()) {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
