use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dijkstra(matrix: &[Vec<char>], i_s: usize, j_s: usize) -> i32 {
    let mut seen = vec![vec![vec![false; 4]; matrix[0].len()]; matrix.len()];
    let mut hq: BinaryHeap<Reverse<(i32, i32, i32, i32)>> = BinaryHeap::new();
    hq.push(Reverse((0, i_s as i32, j_s as i32, 0_i32)));
    loop {
        let Reverse((cost, i, j, dir)) = hq.pop().unwrap();
        if i < 0 || i >= matrix.len() as i32 || j < 0 || j >= matrix[0].len() as i32 {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if matrix[i][j] == 'E' {
            return cost;
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
