use std::{
    collections::VecDeque,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

pub fn bfs(matrix: &[Vec<i8>], i: usize, j: usize) -> i32 {
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    deque.push_back((i, j));
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut result = 0;
    while !deque.is_empty() {
        let size = deque.len();
        for _ in 0..size {
            let (i, j) = deque.pop_front().unwrap();
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            let x: i8 = matrix[i][j];
            if x == 9 {
                result += 1;
                continue;
            }
            for (di, dj) in DIRS4.iter() {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || ni >= matrix.len() as i32 || nj < 0 || nj >= matrix[0].len() as i32 {
                    continue;
                }
                if matrix[ni as usize][nj as usize] - x != 1 {
                    continue;
                }
                deque.push_back((ni as usize, nj as usize));
            }
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let matrix: Vec<Vec<i8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i8 - '0' as i8).collect())
        .collect();
    let mut result = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                result += bfs(&matrix, i, j);
            }
        }
    }
    println!("{}", result);
}
