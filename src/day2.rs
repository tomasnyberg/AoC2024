pub fn safe_report(xs: Vec<i32>) -> i32 {
    let increasing: bool = xs.windows(2).all(|x| x[0] < x[1]);
    let decreasing: bool = xs.windows(2).all(|x| x[0] > x[1]);
    let max_d: bool = xs.windows(2).all(|x| (x[0] - x[1]).abs() <= 3);
    ((increasing || decreasing) && max_d) as i32
}
pub fn part_one(xss: &Vec<Vec<i32>>) -> i32 {
    xss.iter().map(|xs| safe_report(xs.to_vec())).sum()
}

pub fn part_two(xss: Vec<Vec<i32>>) -> i32 {
    xss.iter()
        .map(|xs| {
            (0..xs.len()).any(|i| {
                let xs_without: Vec<i32> = xs
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect();
                safe_report(xs_without) == 1
            }) as i32
        })
        .sum()
}

pub fn solve(input: String) {
    let xss: Vec<Vec<i32>> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!("{}", part_one(&xss));
    println!("{}", part_two(xss));
}
