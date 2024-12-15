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

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let mut matrix: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves = parts.next().unwrap();
    let moves = moves.replace('\n', "");
    let mut i = 0;
    let mut j = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                i = x as i32;
                j = y as i32;
                break;
            }
        }
    }
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
        println!("Move: {}", c);
        println!("To move: {:?}", to_move);
        _debug_print(&matrix);
    }
}
