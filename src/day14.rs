use std::io::{self, Read};

const DIRS8: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn _debug(robots: &[(i32, i32, i32, i32)], n: i32, m: i32) {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
    robots.iter().for_each(|&(x, y, _, _)| {
        grid[y as usize][x as usize] += 1;
    });
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

pub fn parse_part(s: &str) -> (i32, i32) {
    let mut parts_iter = s
        .split('=')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap());
    (parts_iter.next().unwrap(), parts_iter.next().unwrap())
}

pub fn score(robots: &[(i32, i32, i32, i32)], n: i32, m: i32) -> i32 {
    let mut scores: Vec<i32> = vec![0; 4];
    robots.iter().for_each(|&(x, y, _, _)| {
        let mut idx: i32 = 0;
        if x == m / 2 || y == n / 2 {
            return;
        }
        if x > m / 2 {
            idx += 1;
        }
        if y > n / 2 {
            idx += 2;
        }
        scores[idx as usize] += 1;
    });
    scores.iter().cloned().reduce(|a, b| (a * b)).unwrap_or(0)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();
    let n = 103;
    let m = 101;
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        let (x, y): (i32, i32) = parse_part(parts[0]);
        let (dx, dy): (i32, i32) = parse_part(parts[1]);
        robots.push((x, y, dx, dy));
    });
    let mut seen: Vec<Vec<usize>> = vec![vec![0; m as usize]; n as usize];
    for iter in 0..10000 {
        let mut new_robots: Vec<(i32, i32, i32, i32)> = Vec::new();
        let mut components = 0;
        for &(x, y, dx, dy) in robots.iter() {
            let (mut nx, mut ny) = (x + dx, y + dy);
            if nx < 0 {
                nx += m;
            }
            if ny < 0 {
                ny += n;
            }
            nx %= m;
            ny %= n;
            new_robots.push((nx, ny, dx, dy));
            if iter != 0 {
                seen[y as usize][x as usize] -= 1;
            }
            seen[ny as usize][nx as usize] += 1;
            if seen[ny as usize][nx as usize] != 1 {
                continue;
            }
            let mut new_c = 1;
            for &(dx, dy) in DIRS8.iter() {
                let (ni, nj) = (nx + dx, ny + dy);
                if ni < 0 || ni >= m || nj < 0 || nj >= n {
                    continue;
                }
                if seen[nj as usize][ni as usize] != 0 {
                    new_c = 0;
                    break;
                }
            }
            components += new_c;
        }
        robots = new_robots;
        if iter == 99 {
            println!("{}", score(&robots, n, m));
        }
        if components < 200 {
            println!("{}", iter + 1);
            break;
        }
    }
}
