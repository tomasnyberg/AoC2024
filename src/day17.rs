use std::io::{self, Read};

// 0 = adv, divide A by 2^combo operand
// 1 = bxl, bitwise XOR B
// 2 = bst, modulo 8 combo operand and store in B
// 3 = jnz, nothing if A is 0 else jump to literal (do not increment ip)
// 4 = bxc, bitwise XOR B and C and store in B (ignore operand)
// 5 = out, calculate combo operand mod 8 and output it
// 6 = bdv, adv, but store to B instead of A
// 7 = cdv, adv, but store to C instead of A

const MAPPED: [&str; 8] = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let mut registers: Vec<i32> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse::<i32>().unwrap())
        .collect();
    let instructions: Vec<i32> = parts
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    let mut i = 0;
    let mut result: Vec<i32> = Vec::new();
    while i < instructions.len() {
        let (op, operand) = (instructions[i], instructions[i + 1]);
        let combo = match operand {
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => operand,
        };
        match op {
            0 => registers[0] /= 2_i32.pow(combo as u32),
            1 => registers[1] ^= operand,
            2 => registers[1] = combo % 8,
            3 => {
                if registers[0] != 0 {
                    i = operand as usize;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => result.push(combo % 8),
            6 => registers[1] = registers[0] / 2_i32.pow(combo as u32),
            7 => registers[2] = registers[0] / 2_i32.pow(combo as u32),
            _ => (),
        }
        if op != 3 || registers[0] == 0 {
            i += 2;
        }
    }
    println!(
        "{}",
        result
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
