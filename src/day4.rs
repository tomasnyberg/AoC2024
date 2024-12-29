const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRS8: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIRS_UP_DOWN: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

fn find_xmas(matrix: &[Vec<char>], i: i32, j: i32, di: i32, dj: i32) -> i32 {
    for k in 0..4 {
        let oi = i + di * k;
        let oj = j + dj * k;
        if oi < 0 || oi >= matrix.len() as i32 || oj < 0 || oj >= matrix[0].len() as i32 {
            return 0;
        }
        if matrix[oi as usize][oj as usize] != XMAS[k as usize] {
            return 0;
        }
    }
    1
}

fn find_x_mas(matrix: &[Vec<char>], i: i32, j: i32) -> i32 {
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let in_bounds = |oi, oj| oi >= 0 && oi < n && oj >= 0 && oj < m;
    (DIRS_UP_DOWN
        .into_iter()
        .map(|((di1, dj1), (di2, dj2))| {
            let (oi1, oj1) = (i + di1, j + dj1);
            let (oi2, oj2) = (i + di2, j + dj2);
            if !in_bounds(oi1, oj1) || !in_bounds(oi2, oj2) {
                return 0;
            }
            let m = matrix[oi1 as usize][oj1 as usize] == 'M'
                || matrix[oi2 as usize][oj2 as usize] == 'M';
            let s = matrix[oi1 as usize][oj1 as usize] == 'S'
                || matrix[oi2 as usize][oj2 as usize] == 'S';
            (m && s) as i32
        })
        .sum::<i32>()
        == 2) as i32
}

fn find_results(matrix: &[Vec<char>]) -> (i32, i32) {
    let mut part_one_result = 0;
    let mut part_two_result = 0;
    for i in 0..matrix.len() as i32 {
        for j in 0..matrix[0].len() as i32 {
            part_one_result += DIRS8
                .iter()
                .map(|(di, dj)| find_xmas(matrix, i, j, *di, *dj))
                .sum::<i32>();
            part_two_result +=
                (matrix[i as usize][j as usize] == 'A') as i32 * find_x_mas(matrix, i, j)
        }
    }
    (part_one_result, part_two_result)
}

pub fn solve(input: String) {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (part_one_result, part_two_result) = find_results(&matrix);
    println!("{:?}", part_one_result);
    println!("{:?}", part_two_result);
}
