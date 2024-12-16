use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn test_dijsktra(
    matrix: &[Vec<char>],
    i_s: usize,
    j_s: usize,
    costs: &Vec<Vec<Vec<i32>>>,
    max_c: i32,
) -> bool {
    let mut seen = vec![vec![vec![false; 4]; matrix[0].len()]; matrix.len()];
    let mut hq: BinaryHeap<Reverse<(i32, i32, i32, i32)>> = BinaryHeap::new();
    hq.push(Reverse((costs[i_s][j_s][0], i_s as i32, j_s as i32, 0_i32)));
    hq.push(Reverse((costs[i_s][j_s][1], i_s as i32, j_s as i32, 1_i32)));
    hq.push(Reverse((costs[i_s][j_s][2], i_s as i32, j_s as i32, 2_i32)));
    hq.push(Reverse((costs[i_s][j_s][3], i_s as i32, j_s as i32, 3_i32)));
    loop {
        let Reverse((cost, i, j, dir)) = hq.pop().unwrap();
        if cost > max_c {
            return false;
        }
        if i < 0 || i >= matrix.len() as i32 || j < 0 || j >= matrix[0].len() as i32 {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if matrix[i][j] == 'E' {
            return cost == max_c;
        }
        if seen[i][j][dir as usize] || matrix[i][j] == '#' {
            continue;
        }
        seen[i][j][dir as usize] = true;
        hq.push(Reverse((cost + 1000, i as i32, j as i32, (dir + 1) % 4)));
        hq.push(Reverse((
            cost + 1000,
            i as i32,
            j as i32,
            (dir + 4 - 1) % 4,
        )));
        let (di, dj) = DIRS4[dir as usize];
        hq.push(Reverse((cost + 1, i as i32 + di, j as i32 + dj, dir)));
    }
}

fn dijkstra(matrix: &[Vec<char>], i_s: usize, j_s: usize) -> i32 {
    let mut seen = vec![vec![vec![-1; 4]; matrix[0].len()]; matrix.len()];
    let mut hq: BinaryHeap<Reverse<(i32, i32, i32, i32)>> = BinaryHeap::new();
    hq.push(Reverse((0, i_s as i32, j_s as i32, 0_i32)));
    let mut max_c = -1;
    while !hq.is_empty() {
        let Reverse((cost, i, j, dir)) = hq.pop().unwrap();
        if i < 0 || i >= matrix.len() as i32 || j < 0 || j >= matrix[0].len() as i32 {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if seen[i][j][dir as usize] != -1 || matrix[i][j] == '#' {
            continue;
        }
        if matrix[i][j] == 'E' && max_c == -1 {
            max_c = cost;
        }
        seen[i][j][dir as usize] = cost;
        hq.push(Reverse((cost + 1000, i as i32, j as i32, (dir + 1) % 4)));
        hq.push(Reverse((
            cost + 1000,
            i as i32,
            j as i32,
            (dir + 4 - 1) % 4,
        )));
        let (di, dj) = DIRS4[dir as usize];
        hq.push(Reverse((cost + 1, i as i32 + di, j as i32 + dj, dir)));
    }
    println!("max c {}", max_c);
    let mut result = 0;
    (0..matrix.len()).for_each(|i| {
        println!("i {}", i);
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '#' && test_dijsktra(matrix, i, j, &seen, max_c) {
                result += 1;
            }
        }
    });
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut i_s, mut j_s) = (0, 0);
    (0..matrix.len()).for_each(|i| {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'S' {
                i_s = i;
                j_s = j;
                break;
            }
        }
    });
    println!("{}", dijkstra(&matrix, i_s, j_s));
}
