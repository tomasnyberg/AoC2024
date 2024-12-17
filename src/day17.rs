use std::io::{self, Read};

// 0 = adv, divide A by 2^combo operand
// 1 = bxl, bitwise XOR B
// 2 = bst, modulo 8 combo operand and store in B
// 3 = jnz, nothing if A is 0 else jump to literal (do not increment ip)
// 4 = bxc, bitwise XOR B and C and store in B (ignore operand)
// 5 = out, calculate combo operand mod 8 and output it
// 6 = bdv, adv, but store to B instead of A
// 7 = cdv, adv, but store to C instead of A
//

/*
* 2 4 - B = A % 8
* 1 3 - B ^= 3
* 7 5 - C = A / 2^B
* 0 3 - A = A / 2^3
* 1 5 - B ^= 5
* 4 4 - B ^= C
* 5 5 - output B % 8
* 3 0 - if A != 0 jump to 0
*/

// A == 0 <- not possible since then we would have quit on the last round
// A == 1 <- doesn't work
//

const MAPPED: [&str; 8] = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];
// based on the first 8 comments
const EXPLAINED: [&str; 8] = [
    "Store A/2^combo to A; ",
    "B ^= literal; ",
    "B = combo % 8; ",
    "Jump to literal if A != 0; ",
    "B ^= C (ignore operand); ",
    "Output combo % 8; ",
    "Store A/2^combo to B; ",
    "Store A/2^combo to C; ",
];

// outputs (printed value, i) where printed value is -1 if nothing printed
fn execute_instruction(mut i: usize, instructions: &[i64], registers: &mut [i64]) -> (i64, usize) {
    let mut output = -1;
    let (op, operand) = (instructions[i], instructions[i + 1]);
    let combo = match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => operand,
    };
    //println!("registers {:?}", registers);
    //println!(
    //    "{} combo {} literal {}",
    //    EXPLAINED[op as usize], combo, operand
    //);
    match op {
        0 => registers[0] /= 2_i64.pow(combo as u32),
        1 => registers[1] ^= operand,
        2 => registers[1] = combo % 8,
        3 => {
            if registers[0] != 0 {
                i = operand as usize;
            }
        }
        4 => registers[1] ^= registers[2],
        5 => output = combo % 8,
        6 => registers[1] = registers[0] / 2_i64.pow(combo as u32),
        7 => registers[2] = registers[0] / 2_i64.pow(combo as u32),
        _ => (),
    }
    if op != 3 || registers[0] == 0 {
        return (output, i + 2);
    }
    (output, i)
}

fn part_one(instructions: &[i64], original_registers: &[i64]) -> String {
    let mut registers = original_registers.to_vec();
    let mut i = 0;
    let mut result = Vec::new();
    while i < instructions.len() {
        let (output, new_i) = execute_instruction(i, instructions, &mut registers);
        if output != -1 {
            result.push(output);
        }
        i = new_i;
    }
    result
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn brute_force(instructions: &[i64]) -> i64 {
    let mut best_correct = 0;
    let mut prev_a = 0;
    for div_by in 1..1000 {
        let start = 8_i64.pow(15) + ((8_i64.pow(16) - 8_i64.pow(15)) / div_by * div_by);
        let mut i = 0;
        let mut registers = vec![start, 0, 0];
        let mut correct_idx = 0;
        let mut seen: Vec<i64> = Vec::new();
        while i < instructions.len() {
            let (output, new_i) = execute_instruction(i, instructions, &mut registers);
            if output != -1 {
                seen.push(output);
            }
            i = new_i;
        }
        println!("start: {}", start);
        println!("seen: {:?} len {}", seen, seen.len());
    }
    -1
}

fn try_combo(a: i64, target: i64) -> bool {
    let mut b = a % 8;
    b ^= 3;
    let c = a / 2_i64.pow(b.try_into().unwrap());
    //a /= 2_i64.pow(3);
    b ^= 5;
    b ^= c;
    let output = b % 8;
    output == target
}

fn dfs(idx: i64, instructions: &[i64], curr_a: i64) -> i64 {
    if idx < 0 {
        return curr_a / 8;
    }
    let expected = instructions[idx as usize];
    for i in 0..8 {
        let start = curr_a + i;
        if try_combo(start, expected) {
            let next_a = start * 8;
            let res = dfs(idx - 1, instructions, next_a);
            if res != -1 {
                return res;
            }
        }
    }
    -1
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let mut registers: Vec<i64> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse::<i64>().unwrap())
        .collect();
    let instructions: Vec<i64> = parts
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect();
    println!("Part one: {}", part_one(&instructions, &registers));
    let mut idx = instructions.len() - 1;
    let mut curr_a: i64 = 0;
    println!("dfs result: {}", dfs(idx as i64, &instructions, curr_a));
    //while idx > 0 {
    //    let expected = instructions[idx];
    //    for i in 0..8 {
    //        let start = curr_a + i;
    //        if try_combo(start, expected) {
    //            println!("found next at {} {}", start, i);
    //            curr_a += i;
    //            curr_a *= 8;
    //            idx -= 1;
    //            println!("a is now {} idx is now {}", curr_a, idx);
    //            break;
    //        }
    //    }
    //}
    //for i in 0..8 {
    //    let start = 6 * 8 + i;
    //    println!("try_combo {} {}", i, try_combo(start, 3));
    //}
    //println!("Amount of instructions: {}", instructions.len());
    //brute_force(&instructions);
}
