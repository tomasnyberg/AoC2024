use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    io::{self, Read},
};

const DIRS4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn bfs(preds: &HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>>, i_s: i32, j_s: i32) -> i32 {
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    q.push_back((i_s, j_s, 0));
    q.push_back((i_s, j_s, 1));
    q.push_back((i_s, j_s, 2));
    q.push_back((i_s, j_s, 3));
    while !q.is_empty() {
        let (i, j, dir) = q.pop_front().unwrap();
        if seen.contains(&(i, j, dir)) {
            continue;
        }
        seen.insert((i, j, dir));
        for &(ni, nj, ndir) in preds.get(&(i, j, dir)).unwrap_or(&vec![]) {
            q.push_back((ni, nj, ndir));
        }
    }
    let seen: HashSet<(i32, i32)> = seen.into_iter().map(|(i, j, _)| (i, j)).collect();
    seen.len() as i32
}

fn dijkstra(matrix: &[Vec<char>], i_s: usize, j_s: usize) -> (i32, i32) {
    let mut seen = vec![vec![vec![-1; 4]; matrix[0].len()]; matrix.len()];
    let mut hq: BinaryHeap<Reverse<(i32, i32, i32, i32, i32, i32, i32)>> = BinaryHeap::new();
    let mut preds: HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    hq.push(Reverse((
        0, i_s as i32, j_s as i32, 0_i32, i_s as i32, j_s as i32, 0,
    )));
    let mut max_c = -1;
    let mut i_e = 0;
    let mut j_e = 0;
    while !hq.is_empty() {
        let Reverse((cost, i, j, dir, p_i, p_j, p_d)) = hq.pop().unwrap();
        if i < 0 || i >= matrix.len() as i32 || j < 0 || j >= matrix[0].len() as i32 {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if seen[i][j][dir as usize] == -1 || seen[i][j][dir as usize] == cost {
            let v = preds.entry((i as i32, j as i32, dir)).or_default();
            v.push((p_i, p_j, p_d));
        }
        if seen[i][j][dir as usize] != -1 || matrix[i][j] == '#' {
            continue;
        }
        if matrix[i][j] == 'E' && max_c == -1 {
            i_e = i;
            j_e = j;
            max_c = cost;
        }
        seen[i][j][dir as usize] = cost;
        let cw = (dir + 1) % 4;
        let ccw = (dir + 4 - 1) % 4;
        let (i, j) = (i as i32, j as i32);
        let (di, dj) = DIRS4[dir as usize];
        hq.push(Reverse((cost + 1000, i, j, cw, i, j, dir)));
        hq.push(Reverse((cost + 1000, i, j, ccw, i, j, dir)));
        hq.push(Reverse((cost + 1, i + di, j + dj, dir, i, j, dir)));
    }
    let unique_squares = bfs(&preds, i_e as i32, j_e as i32);
    (max_c, unique_squares)
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
    let (part_one, part_two) = dijkstra(&matrix, i_s, j_s);
    println!("{}", part_one);
    println!("{}", part_two);
}
