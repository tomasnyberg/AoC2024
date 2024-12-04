use std::io::{self, Read};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRS8: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIRS_UP_DOWN: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

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

fn find_x_mas(matrix: &[Vec<char>], i: i32, j: i32) -> i32 {
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    (DIRS_UP_DOWN
        .into_iter()
        .map(|((di1, dj1), (di2, dj2))| {
            let oi1 = i + di1;
            let oj1 = j + dj1;
            let oi2 = i + di2;
            let oj2 = j + dj2;
            if oi1 < 0 || oi1 >= n || oj1 < 0 || oj1 >= m {
                return 0;
            }
            if oi2 < 0 || oi2 >= n || oj2 < 0 || oj2 >= m {
                return 0;
            }
            let m = matrix[oi1 as usize][oj1 as usize] == 'M'
                || matrix[oi2 as usize][oj2 as usize] == 'M';
            let s = matrix[oi1 as usize][oj1 as usize] == 'S'
                || matrix[oi2 as usize][oj2 as usize] == 'S';
            (m && s) as i32
        })
        .sum::<i32>()
        == 2) as i32
}

fn part_one(matrix: &[Vec<char>]) -> i32 {
    let mut result = 0;
    for i in 0..matrix.len() as i32 {
        for j in 0..matrix[0].len() as i32 {
            for (di, dj) in DIRS8.iter() {
                result += find_xmas(matrix, i, j, *di, *dj);
            }
        }
    }
    result
}

fn part_two(matrix: &[Vec<char>]) -> i32 {
    let mut result = 0;
    for i in 0..matrix.len() as i32 {
        for j in 0..matrix[0].len() as i32 {
            if matrix[i as usize][j as usize] == 'A' {
                result += find_x_mas(matrix, i, j);
            }
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // read input into a 2x2 char matrix
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let part_one_result = part_one(&matrix);
    let part_two_result = part_two(&matrix);
    println!("{:?}", part_one_result);
    println!("{:?}", part_two_result);
}
