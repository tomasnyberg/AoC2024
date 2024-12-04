use std::collections::HashMap;
use std::io::{self, Read};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn find_xmas(matrix: &[Vec<char>], i: i32, j: i32, di: i32, dj: i32) -> i32 {
    for k in 0..4 {
        let oi = i + di * k;
        let oj = j + dj * k;
        if oi < 0 || oi >= matrix.len() as i32 || oj < 0 || oj >= matrix[0].len() as i32 {
            return 0;
        }
        if matrix[oi as usize][oj as usize] != XMAS[k as usize] {
            return 0;
        }
    }
    1
}

fn part_one() {}

fn part_two() {}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // read input into a 2x2 char matrix
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let dirs8 = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut result = 0;
    for i in 0..matrix.len() as i32 {
        for j in 0..matrix[0].len() as i32 {
            for (di, dj) in dirs8.iter() {
                result += find_xmas(&matrix, i, j, *di, *dj);
            }
        }
    }
    println!("{:?}", result);
}
