use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

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
    let mut result = 0;
    for (i, j) in from_start.keys() {
        let steps = from_start[&(*i, *j)];
        for (oi, oj) in from_end.keys() {
            let d = (*oi as i32 - *i as i32).abs() + (*oj as i32 - *j as i32).abs();
            if d > skips_allowed {
                continue;
            }
            let total = steps + from_end[&(*oi, *oj)] + d;
            let saved = end_steps - total;
            if saved >= threshold {
                result += 1;
            }
        }
    }
    result
}

pub fn bfs(grid: &[Vec<char>], s: &Pos, e: &Pos) -> (i32, i32) {
    let mut from_start: HashMap<(usize, usize), i32> = HashMap::new();
    let mut from_end: HashMap<(usize, usize), i32> = HashMap::new();
    for iter in 0..2 {
        let mut steps = 0;
        let mut queue = VecDeque::new();
        if iter == 0 {
            queue.push_back((s.i, s.j));
        } else {
            queue.push_back((e.i, e.j));
        }
        let map = if iter == 0 {
            &mut from_start
        } else {
            &mut from_end
        };
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (i, j) = queue.pop_front().unwrap();
                if map.contains_key(&(i, j)) {
                    continue;
                }
                map.insert((i, j), steps);
                for (di, dj) in DIRS4.iter() {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if ni >= 0
                        && ni < grid.len() as i32
                        && nj >= 0
                        && nj < grid[0].len() as i32
                        && grid[ni as usize][nj as usize] != '#'
                    {
                        queue.push_back((ni as usize, nj as usize));
                    }
                }
            }
            steps += 1;
        }
    }
    let part_one = find_cheats(&from_start, &from_end, e, grid.len() as i32, 2);
    let part_two = find_cheats(&from_start, &from_end, e, grid.len() as i32, 20);
    (part_one, part_two)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
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
