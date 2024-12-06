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

fn part_one() {}

fn part_two() {}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut dir_idx: usize = 0;
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
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    loop {
        //debug_print(&matrix, t_i, t_j, dir_idx);
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
    println!("{}", visited.len());
}
