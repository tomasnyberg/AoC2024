use std::io::{self, Read};

const MOVE_CHARS: [char; 4] = ['^', '>', 'v', '<'];
const MOVE_VECTORS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn _debug_print(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn score(matrix: &[Vec<char>]) -> i32 {
    let mut score = 0;
    (0..matrix.len()).for_each(|i| {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'O' {
                score += (i as i32 * 100) + j as i32;
            }
        }
    });
    score
}

fn find_start(matrix: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                return (x as i32, y as i32);
            }
        }
    }
    (0, 0)
}

fn part_one(matrix: &[Vec<char>], moves: &str) -> i32 {
    let (mut i, mut j) = find_start(matrix);
    let mut matrix: Vec<Vec<char>> = matrix.to_vec();
    for c in moves.chars() {
        let idx = MOVE_CHARS.iter().position(|&x| x == c).unwrap();
        let (di, dj) = MOVE_VECTORS[idx];
        let (mut ni, mut nj) = (i, j);
        let mut to_move: Vec<char> = vec!['@'];
        loop {
            ni += di;
            nj += dj;
            if matrix[ni as usize][nj as usize] == '#' {
                to_move.clear();
                break;
            }
            if matrix[ni as usize][nj as usize] == '.' {
                break;
            }
            to_move.push(matrix[ni as usize][nj as usize]);
        }
        if !to_move.is_empty() {
            matrix[i as usize][j as usize] = '.';
            matrix[(i + di) as usize][(j + dj) as usize] = '@';
            if to_move.len() > 1 {
                let last_i = i + to_move.len() as i32 * di;
                let last_j = j + to_move.len() as i32 * dj;
                matrix[last_i as usize][last_j as usize] = 'O';
            }
            i += di;
            j += dj;
        }
    }
    score(&matrix)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let matrix: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves = parts.next().unwrap();
    let moves = moves.replace('\n', "");
    let part_one = part_one(&matrix, &moves);
    println!("{}", part_one);
}
