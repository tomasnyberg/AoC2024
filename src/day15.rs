use std::collections::{HashMap, VecDeque};

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
            if matrix[i][j] == 'O' || matrix[i][j] == '[' {
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
                return (y as i32, x as i32);
            }
        }
    }
    (0, 0)
}

fn make_big_matrix(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut big_matrix: Vec<Vec<char>> = vec![vec!['.'; matrix[0].len() * 2]; matrix.len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'O' {
                big_matrix[i][j * 2] = '[';
                big_matrix[i][j * 2 + 1] = ']';
                continue;
            }
            big_matrix[i][j * 2] = c;
            big_matrix[i][j * 2 + 1] = c;
            if c == '@' {
                big_matrix[i][j * 2 + 1] = '.';
            }
        }
    }
    big_matrix
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

fn move_vertical(matrix: &mut [Vec<char>], i: i32, j: i32, dir: i32) -> bool {
    let mut seen: HashMap<(i32, i32), char> = HashMap::new();
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((i, j));
    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        if matrix[i as usize][j as usize] == '#' {
            seen.clear();
            break;
        }
        if matrix[i as usize][j as usize] == '.' {
            continue;
        }
        if seen.contains_key(&(i, j)) {
            continue;
        }
        seen.insert((i, j), matrix[i as usize][j as usize]);
        if matrix[i as usize][j as usize] == '[' {
            q.push_back((i, j + 1));
        }
        if matrix[i as usize][j as usize] == ']' {
            q.push_back((i, j - 1));
        }
        q.push_back((i + dir, j));
    }
    for (i, j) in seen.keys() {
        matrix[*i as usize][*j as usize] = '.';
    }
    for (i, j) in seen.keys() {
        matrix[(*i + dir) as usize][*j as usize] = seen[&(*i, *j)];
    }
    !seen.is_empty()
}

fn part_two(matrix: &[Vec<char>], moves: &str) -> i32 {
    let (mut i, mut j) = find_start(matrix);
    let mut matrix: Vec<Vec<char>> = matrix.to_vec();
    for c in moves.chars() {
        let idx = MOVE_CHARS.iter().position(|&x| x == c).unwrap();
        let (di, dj) = MOVE_VECTORS[idx];
        let (mut ni, mut nj) = (i, j);
        if idx == 1 || idx == 3 {
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
                ni += di;
                nj += dj;
                to_move.push(matrix[ni as usize][nj as usize]);
            }
            if !to_move.is_empty() {
                ni = i;
                nj = j;
                matrix[i as usize][j as usize] = '.';
                i += di;
                j += dj;
                for c in to_move {
                    ni += di;
                    nj += dj;
                    matrix[ni as usize][nj as usize] = c;
                }
            }
        } else {
            assert!(di != 0 && dj == 0);
            let moved = move_vertical(&mut matrix, i, j, di);
            if moved {
                i += di;
            }
        }
    }
    score(&matrix)
}

pub fn solve(input: String) {
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
    let p2matrix = make_big_matrix(&matrix);
    let part_two = part_two(&p2matrix, &moves);
    println!("{}", part_one);
    println!("{}", part_two);
}
