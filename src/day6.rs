use std::collections::HashSet;
use std::io::{self, Read};

const DIRS4: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const _DIRS_STYLE: [char; 4] = ['^', '>', 'v', '<'];

fn _debug_print(matrix: &[Vec<char>], t_i: i32, t_j: i32, dir_idx: usize) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if i == t_i as usize && j == t_j as usize {
                print!("{}", _DIRS_STYLE[dir_idx]);
                continue;
            }
            print!("{}", matrix[i][j]);
        }
        println!();
    }
    println!();
}

fn part_one(matrix: &[Vec<char>], mut t_i: i32, mut t_j: i32) -> usize {
    let mut dir_idx: usize = 0;
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    loop {
        visited.insert((t_i, t_j));
        let (di, dj) = DIRS4[dir_idx];
        let (oi, oj) = ((t_i + di), (t_j + dj));
        if oi < 0 || oi >= n || oj < 0 || oj >= m {
            break;
        }
        if matrix[oi as usize][oj as usize] == '#' {
            dir_idx = (dir_idx + 1) % 4;
            continue;
        }
        t_i = oi;
        t_j = oj;
    }
    visited.len()
}

fn find_loop(matrix: &[Vec<char>], mut t_i: i32, mut t_j: i32) -> bool {
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let mut visited_three: HashSet<(i32, i32, usize)> = HashSet::new();
    let mut dir_idx: usize = 0;
    loop {
        if visited_three.contains(&(t_i, t_j, dir_idx)) {
            return true;
        }
        visited_three.insert((t_i, t_j, dir_idx));
        let (di, dj) = DIRS4[dir_idx];
        let (oi, oj) = ((t_i + di), (t_j + dj));
        if oi < 0 || oi >= n || oj < 0 || oj >= m {
            break;
        }
        if matrix[oi as usize][oj as usize] == '#' {
            dir_idx = (dir_idx + 1) % 4;
            continue;
        }
        t_i = oi;
        t_j = oj;
    }
    false
}

fn part_two(matrix: &[Vec<char>], t_i: i32, t_j: i32) -> i32 {
    let mut matrix_copy: Vec<Vec<char>> = vec![];
    for row in matrix {
        matrix_copy.push(row.clone());
    }
    let mut result = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '#' || (i as i32 == t_i && j as i32 == t_j) {
                continue;
            }
            matrix_copy[i][j] = '#';
            if find_loop(&matrix_copy, t_i, t_j) {
                result += 1;
            }
            matrix_copy[i][j] = '.';
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let target = '^';
    let mut t_i = 0;
    let mut t_j = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == target {
                t_i = i as i32;
                t_j = j as i32;
            }
        }
    }
    let part_one_result = part_one(&matrix, t_i, t_j);
    let part_two_result = part_two(&matrix, t_i, t_j);
    println!("{}", part_one_result);
    println!("{}", part_two_result);
}
