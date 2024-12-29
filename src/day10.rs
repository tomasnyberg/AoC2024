use std::collections::VecDeque;

const DIRS4: [(i8, i8); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

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
                let (ni, nj) = (i as i8 + di, j as i8 + dj);
                if ni < 0 || ni >= matrix.len() as i8 || nj < 0 || nj >= matrix[0].len() as i8 {
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

pub fn dfs(matrix: &[Vec<i8>], i: i8, j: i8) -> i16 {
    let x = matrix[i as usize][j as usize];
    if x == 9 {
        return 1;
    }
    let mut result = 0;
    for (di, dj) in DIRS4.iter() {
        let (ni, nj) = (i + di, j + dj);
        if ni < 0 || ni >= matrix.len() as i8 || nj < 0 || nj >= matrix[0].len() as i8 {
            continue;
        }
        if matrix[ni as usize][nj as usize] - x != 1 {
            continue;
        }
        result += dfs(matrix, ni, nj);
    }
    result
}

pub fn solve(input: String) {
    let matrix: Vec<Vec<i8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i8 - '0' as i8).collect())
        .collect();
    let mut result_part_one = 0;
    let mut result_part_two = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                result_part_one += bfs(&matrix, i, j);
                result_part_two += dfs(&matrix, i as i8, j as i8);
            }
        }
    }
    println!("{}", result_part_one);
    println!("{}", result_part_two);
}
