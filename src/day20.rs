use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

pub struct Pos {
    i: usize,
    j: usize,
}

pub fn find_cheats(
    from_start: &HashMap<(usize, usize), i32>,
    from_end: &HashMap<(usize, usize), i32>,
    e: &Pos,
    n: i32,
    skips_allowed: i32,
) -> i32 {
    let end_steps = from_start[&(e.i, e.j)];
    let threshold = if n > 50 { 100 } else { 50 };
    from_start
        .par_iter()
        .map(|(&(i, j), &_v)| {
            let mut local_result = 0;
            let steps = from_start[&(i, j)];
            for di in -20..21_i32 {
                for dj in (di.abs() - 20)..(21 - di.abs()) {
                    let (oi, oj) = (i as i32 + di, j as i32 + dj);
                    if from_end.contains_key(&(oi as usize, oj as usize)) {
                        let d = di.abs() + dj.abs();
                        if d > skips_allowed {
                            continue;
                        }
                        let total = steps + from_end[&(oi as usize, oj as usize)] + d;
                        let saved = end_steps - total;
                        if saved >= threshold {
                            local_result += 1;
                        }
                    }
                }
            }
            local_result
        })
        .sum()
}

pub fn bfs(grid: &[Vec<char>], s: &Pos, e: &Pos) -> (i32, i32) {
    let mut from_start: HashMap<(usize, usize), i32> = HashMap::new();
    let mut from_end: HashMap<(usize, usize), i32> = HashMap::new();
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    for iter in 0..2 {
        let mut steps = 0;
        let mut queue = VecDeque::new();
        queue.push_back(if iter == 0 { (s.i, s.j) } else { (e.i, e.j) });
        let map = if iter == 0 {
            &mut from_start
        } else {
            &mut from_end
        };
        while !queue.is_empty() {
            let qlen = queue.len();
            for _ in 0..qlen {
                let (i, j) = queue.pop_front().unwrap();
                if map.contains_key(&(i, j)) {
                    continue;
                }
                map.insert((i, j), steps);
                for (di, dj) in DIRS4.iter() {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if (0..n).contains(&ni)
                        && (0..m).contains(&nj)
                        && grid[ni as usize][nj as usize] != '#'
                    {
                        queue.push_back((ni as usize, nj as usize));
                    }
                }
            }
            steps += 1;
        }
    }
    let part_one = find_cheats(&from_start, &from_end, e, n, 2);
    let part_two = find_cheats(&from_start, &from_end, e, n, 20);
    (part_one, part_two)
}

pub fn solve(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut i_s, mut j_s, mut i_e, mut j_e) = (0, 0, 0, 0);
    (0..grid.len()).for_each(|i| {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                i_s = i;
                j_s = j;
            } else if grid[i][j] == 'E' {
                i_e = i;
                j_e = j;
            }
        }
    });
    let s = Pos { i: i_s, j: j_s };
    let e = Pos { i: i_e, j: j_e };
    let (part_one, part_two) = bfs(&grid, &s, &e);
    println!("{}", part_one);
    println!("{}", part_two);
}
