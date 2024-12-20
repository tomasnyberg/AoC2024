use std::{
    collections::VecDeque,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

pub struct Pos {
    i: usize,
    j: usize,
}

pub fn bfs(grid: &[Vec<char>], s: &Pos, e: &Pos) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((s.i, s.j));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut steps = 0;
    loop {
        let n = queue.len();
        if n == 0 {
            panic!();
        }
        for _ in 0..n {
            let (i, j) = queue.pop_front().unwrap();
            if i == e.i && j == e.j {
                return steps;
            }
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            for (di, dj) in DIRS4.iter() {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0
                    && ni < grid.len() as i32
                    && nj >= 0
                    && nj < grid[0].len() as i32
                    && grid[ni as usize][nj as usize] != '#'
                {
                    queue.push_back((ni as usize, nj as usize));
                }
            }
        }
        steps += 1;
    }
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut i_s, mut j_s, mut i_e, mut j_e) = (0, 0, 0, 0);
    (0..grid.len()).for_each(|i| {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                i_s = i;
                j_s = j;
            } else if grid[i][j] == 'E' {
                i_e = i;
                j_e = j;
            }
        }
    });
    let s = Pos { i: i_s, j: j_s };
    let e = Pos { i: i_e, j: j_e };
    let base_steps = bfs(&grid, &s, &e);
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                grid[i][j] = '.';
                let steps = bfs(&grid, &s, &e);
                if base_steps - steps >= 100 {
                    result += 1;
                }
                grid[i][j] = '#';
            }
        }
    }
    println!("{}", result);
}
