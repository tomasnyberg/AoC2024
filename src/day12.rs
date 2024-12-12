use std::io::{self, Read};

const DIRS4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

pub fn dfs(
    i: i32,
    j: i32,
    expected: char,
    visited: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>,
) -> (i32, i32) {
    if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
        return (1, 0);
    }
    if visited[i as usize][j as usize] {
        return if grid[i as usize][j as usize] == expected {
            (0, 0)
        } else {
            (1, 0)
        };
    }
    if grid[i as usize][j as usize] != expected {
        return (1, 0);
    }
    visited[i as usize][j as usize] = true;
    let (perim, area) = DIRS4
        .iter()
        .map(|(di, dj)| dfs(i + di, j + dj, expected, visited, grid))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    (perim, area + 1)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];
    let mut result = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if !visited[i][j] {
                let (perim, area) = dfs(i as i32, j as i32, lines[i][j], &mut visited, &lines);
                result += perim * area;
            }
        }
    }
    println!("{}", result);
}
