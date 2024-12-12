use std::{
    collections::HashSet,
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

pub fn dfs(
    i: i32,
    j: i32,
    expected: char,
    visited: &mut Vec<Vec<bool>>,
    perimeter: &mut HashSet<(i32, i32, i32)>,
    grid: &Vec<Vec<char>>,
) -> i32 {
    visited[i as usize][j as usize] = true;
    1 + DIRS4
        .iter()
        .enumerate()
        .map(|(idx, (di, dj))| {
            let (ni, nj) = (i + di, j + dj);
            let a = ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32;
            let b = !a && grid[ni as usize][nj as usize] != expected;
            if a || b {
                perimeter.insert((i, j, idx as i32));
                return 0;
            } else if !visited[ni as usize][nj as usize] {
                return dfs(ni, nj, expected, visited, perimeter, grid);
            }
            0
        })
        .sum::<i32>()
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];
    let mut result_part_one = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if !visited[i][j] {
                let mut perimeter = HashSet::new();
                let node_c = dfs(
                    i as i32,
                    j as i32,
                    lines[i][j],
                    &mut visited,
                    &mut perimeter,
                    &lines,
                );
                result_part_one += perimeter.len() as i32 * node_c;
            }
        }
    }
    println!("{}", result_part_one);
}
