use std::io::{self, Read};

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut disk: Vec<i64> = vec![];
    let mut flag = 1;
    let mut file_id = 0;
    input.trim().chars().for_each(|c| {
        let num = c.to_digit(10).unwrap();
        let to_push = match flag {
            0 => -1,
            1 => file_id,
            _ => panic!("Invalid flag"),
        };
        for _ in 0..num {
            disk.push(to_push);
        }
        file_id += flag;
        flag = 1 - flag;
    });
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if disk[left] != -1 {
            left += 1;
            continue;
        }
        if disk[right] == -1 {
            right -= 1;
            continue;
        }
        disk.swap(left, right);
        left += 1;
        right -= 1;
    }
    let end = disk.iter().position(|&x| x == -1).unwrap();
    let result = disk
        .iter()
        .enumerate()
        .take(end)
        .map(|(i, &x)| x * i as i64)
        .sum::<i64>();
    println!("{:?}", result);
}
