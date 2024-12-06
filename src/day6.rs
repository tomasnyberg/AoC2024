use rayon::prelude::*;
use std::collections::HashMap;
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

fn part_one(matrix: &[Vec<char>], t_i: i32, t_j: i32) -> HashSet<(i32, i32)> {
    let result = find_loop(matrix, t_i, t_j, -1, -1, true, &HashMap::new());
    let mut visited_two: HashSet<(i32, i32)> = HashSet::new();
    for (i, j, _) in result.1 {
        visited_two.insert((i, j));
    }
    visited_two
}

fn insert_all(
    result: &mut HashMap<(i32, i32, usize), (i32, i32, usize)>,
    i: i32,
    j: i32,
    dir_idx: usize,
    curr: &mut Vec<(i32, i32)>,
) {
    curr.iter().for_each(|&(ci, cj)| {
        result.insert((ci, cj, dir_idx), (i, j, (dir_idx + 1) % 4));
    });
    curr.clear();
}

fn create_jump_table(matrix: &[Vec<char>]) -> HashMap<(i32, i32, usize), (i32, i32, usize)> {
    let mut result: HashMap<(i32, i32, usize), (i32, i32, usize)> = HashMap::new();
    let mut curr: Vec<(i32, i32)> = vec![];
    // >
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '#' {
                insert_all(&mut result, i as i32, j as i32 - 1, 1, &mut curr);
            } else {
                curr.push((i as i32, j as i32));
            }
        }
        insert_all(&mut result, -1, -1, 5, &mut curr);
    }
    // <
    for i in 0..matrix.len() {
        for j in (0..matrix[0].len()).rev() {
            if matrix[i][j] == '#' {
                insert_all(&mut result, i as i32, j as i32 + 1, 3, &mut curr);
            } else {
                curr.push((i as i32, j as i32));
            }
        }
        insert_all(&mut result, -1, -1, 5, &mut curr);
    }
    // v
    for j in 0..matrix[0].len() {
        (0..matrix.len()).for_each(|i| {
            if matrix[i][j] == '#' {
                insert_all(&mut result, i as i32 - 1, j as i32, 2, &mut curr);
            } else {
                curr.push((i as i32, j as i32));
            }
        });
        insert_all(&mut result, -1, -1, 5, &mut curr);
    }
    // ^
    for j in 0..matrix[0].len() {
        (0..matrix.len()).rev().for_each(|i| {
            if matrix[i][j] == '#' {
                insert_all(&mut result, i as i32 + 1, j as i32, 0, &mut curr);
            } else {
                curr.push((i as i32, j as i32));
            }
        });
        insert_all(&mut result, -1, -1, 5, &mut curr);
    }
    result
}

fn find_loop(
    matrix: &[Vec<char>],
    mut t_i: i32,
    mut t_j: i32,
    obs_i: i32,
    obs_j: i32,
    part_one: bool,
    jump_table: &HashMap<(i32, i32, usize), (i32, i32, usize)>,
) -> (bool, HashSet<(i32, i32, usize)>) {
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let mut visited_three: HashSet<(i32, i32, usize)> = HashSet::new();
    let mut dir_idx: usize = 0;
    loop {
        if !part_one && t_i != obs_i && t_j != obs_j {
            let next_pos = jump_table.get(&(t_i, t_j, dir_idx)).unwrap_or(&(-1, -1, 0));
            if next_pos.0 == -1 {
                return (false, visited_three);
            }
            t_i = next_pos.0;
            t_j = next_pos.1;
            dir_idx = next_pos.2;
        }
        if visited_three.contains(&(t_i, t_j, dir_idx)) {
            return (true, visited_three);
        }
        visited_three.insert((t_i, t_j, dir_idx));
        let (di, dj) = DIRS4[dir_idx];
        let (oi, oj) = ((t_i + di), (t_j + dj));
        if oi < 0 || oi >= n || oj < 0 || oj >= m {
            break;
        }
        if matrix[oi as usize][oj as usize] == '#' || obs_i == oi && obs_j == oj {
            dir_idx = (dir_idx + 1) % 4;
            continue;
        }
        t_i = oi;
        t_j = oj;
    }
    (false, visited_three)
}

fn part_two(
    matrix: &[Vec<char>],
    t_i: i32,
    t_j: i32,
    part_one_visited: HashSet<(i32, i32)>,
) -> i32 {
    let mut matrix_copy: Vec<Vec<char>> = vec![];
    for row in matrix {
        matrix_copy.push(row.clone());
    }
    let jump_table = create_jump_table(matrix);
    part_one_visited
        .par_iter()
        .map(|&(i, j)| {
            let result = find_loop(matrix, t_i, t_j, i, j, false, &jump_table);
            if result.0 {
                1
            } else {
                0
            }
        })
        .sum()
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
    let part_one_visited = part_one(&matrix, t_i, t_j);
    let part_one_result = part_one_visited.len();
    let part_two_result = part_two(&matrix, t_i, t_j, part_one_visited);
    println!("{}", part_one_result);
    println!("{}", part_two_result);
}
