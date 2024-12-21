use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Read},
};

#[rustfmt::skip]
const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['#', '0', 'A'],
];

const KEYPAD_POSITIONS: [(usize, usize); 10] = [
    (3, 1), // '0' at row 3, column 1
    (2, 0), // '1' at row 2, column 0
    (2, 1), // '2' at row 2, column 1
    (2, 2), // '3' at row 2, column 2
    (1, 0), // '4' at row 1, column 0
    (1, 1), // '5' at row 1, column 1
    (1, 2), // '6' at row 1, column 2
    (0, 0), // '7' at row 0, column 0
    (0, 1), // '8' at row 0, column 1
    (0, 2), // '9' at row 0, column 2
];

const DIR_PAD_CHARS: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

#[derive(PartialEq)]
enum PadType {
    Keypad,
    Dirpad,
}

fn get_dir(c: char) -> (i32, i32) {
    let dirs: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (0, 0)),
        ('^', (-1, 0)),
        ('v', (1, 0)),
        ('<', (0, -1)),
        ('>', (0, 1)),
    ]);
    *dirs.get(&c).unwrap()
}
fn digit_to_pos(digit: char) -> (i32, i32) {
    if digit == 'A' {
        (3, 2)
    } else {
        let (i, j) = KEYPAD_POSITIONS[digit.to_digit(16).unwrap() as usize];
        (i as i32, j as i32)
    }
}

fn dirpad_char_to_pos(c: char) -> (i32, i32) {
    for (i, row) in DIR_PAD_CHARS.iter().enumerate() {
        for (j, &char_in_pad) in row.iter().enumerate() {
            if char_in_pad == c {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("Invalid char");
}

fn char_to_pos(c: char, pad_type: &PadType) -> (i32, i32) {
    match pad_type {
        PadType::Keypad => digit_to_pos(c),
        PadType::Dirpad => dirpad_char_to_pos(c),
    }
}

fn dp(
    curr_char: char,
    target_char: char,
    robots: i32,
    max: i32,
    cache: &mut HashMap<(char, char, i32), i64>,
) -> i64 {
    let pad = match robots {
        0 => PadType::Keypad,
        _ => PadType::Dirpad,
    };
    let (s_i, s_j) = char_to_pos(curr_char, &pad);
    let (t_i, t_j) = char_to_pos(target_char, &pad);
    if robots == max {
        return (s_i - t_i).abs() as i64 + (s_j - t_j).abs() as i64 + 1_i64;
    }
    if cache.contains_key(&(curr_char, target_char, robots)) {
        return cache[&(curr_char, target_char, robots)];
    }
    let mut seq: Vec<char> = Vec::new();
    let deltai = s_i - t_i;
    let deltaj = s_j - t_j;
    for _ in 0..(deltai).abs() {
        seq.push(if deltai < 0 { 'v' } else { '^' });
    }
    for _ in 0..(deltaj).abs() {
        seq.push(if deltaj < 0 { '>' } else { '<' });
    }
    if seq.is_empty() {
        return 1;
    }
    let mut candidates = Vec::new();
    let perms = (0..seq.len()).permutations(seq.len());
    for perm_indices in perms {
        let perm: Vec<char> = perm_indices.iter().map(|&i| seq[i]).collect();
        let mut local_result = 0;
        let mut i = s_i;
        let mut j = s_j;
        for idx in 0..perm.len() {
            local_result += dp(
                if idx == 0 { 'A' } else { perm[idx - 1] },
                perm[idx],
                robots + 1,
                max,
                cache,
            );
            i += get_dir(perm[idx]).0;
            j += get_dir(perm[idx]).1;
            if (pad == PadType::Dirpad && DIR_PAD_CHARS[i as usize][j as usize] == '#')
                || (pad == PadType::Keypad && KEYPAD[i as usize][j as usize] == '#')
            {
                local_result = i64::MAX;
                break;
            }
        }
        if local_result == i64::MAX {
            continue;
        }
        local_result += dp(perm[perm.len() - 1], 'A', robots + 1, max, cache);
        candidates.push(local_result);
    }
    let result = candidates.iter().min().unwrap();
    cache.insert((curr_char, target_char, robots), *result);
    *result
}

fn solve_parts(target: &str, max: i32) -> i64 {
    let mut result = dp(
        'A',
        target.chars().next().unwrap(),
        0,
        max,
        &mut HashMap::new(),
    );
    for i in 1..target.len() {
        result += dp(
            target.chars().nth(i - 1).unwrap(),
            target.chars().nth(i).unwrap(),
            0,
            max,
            &mut HashMap::new(),
        );
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut result = 0;
    let mut result2 = 0;
    input.lines().for_each(|line| {
        let num: i64 = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        result += num * solve_parts(line, 2);
        result2 += num * solve_parts(line, 25);
    });
    println!("{}", result);
    println!("{}", result2);
}
