use std::{
    collections::VecDeque,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

fn bfs(grid: &[Vec<i32>], n: usize) -> i32 {
    let mut seen = vec![vec![false; n]; n];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((0, 0));
    let mut level = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            if i == n as i32 - 1 && j == n as i32 - 1 {
                return level;
            }
            if seen[i as usize][j as usize] {
                continue;
            }
            seen[i as usize][j as usize] = true;
            for (di, dj) in DIRS4.iter() {
                let (ni, nj) = (i + di, j + dj);
                if ni < 0 || ni >= n as i32 || nj < 0 || nj >= n as i32 {
                    continue;
                }
                if grid[ni as usize][nj as usize] == 0 {
                    q.push_back((ni, nj));
                }
            }
        }
        level += 1;
    }
    0
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut bytes: Vec<(i32, i32)> = Vec::new();
    let mut n: usize = 7;
    input.lines().for_each(|line| {
        let mut parts = line.split(',');
        let tuple = (
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        );
        if tuple.0 > 6 || tuple.1 > 6 {
            n = 71;
        }
        bytes.push(tuple);
    });
    let mut grid = vec![vec![0; n]; n];
    let end = if n == 7 { 12 } else { 1024 };
    bytes.iter().take(end).for_each(|(i, j)| {
        grid[*i as usize][*j as usize] = 1;
    });
    println!("{}", bfs(&grid, n));
    let mut low = 1024;
    let mut high = bytes.len() as i32;
    while low < high {
        let mid = low + (high - low) / 2;
        let mut new_grid = grid.clone();
        bytes.iter().take(mid as usize).for_each(|(i, j)| {
            new_grid[*i as usize][*j as usize] = 1;
        });
        if bfs(&new_grid, n) == 0 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let (a, b) = bytes[low as usize - 1];
    println!("{},{}", a, b);
}
