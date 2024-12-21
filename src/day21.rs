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

#[rustfmt::skip]
const DIR_PAD: [[(i32, i32); 3]; 2] = [
    [(0, 0), (-1, 0), (1, 1)], // top left is invalid, top right is A
    [(0,-1), (1,  0), (0, 1)],
];

const DIR_PAD_CHARS: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

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

fn valid_square(i: i32, j: i32, pad_type: &PadType) -> bool {
    match pad_type {
        PadType::Keypad => {
            (0..4).contains(&i) && (0..3).contains(&j) && KEYPAD[i as usize][j as usize] != '#'
        }
        PadType::Dirpad => {
            (0..2).contains(&i) && (0..3).contains(&j) && DIR_PAD[i as usize][j as usize] != (0, 0)
        }
    }
}

fn interpret_sequence(seq: Vec<char>, pad_type: &PadType) -> Vec<char> {
    let mut i = match pad_type {
        PadType::Keypad => 3,
        PadType::Dirpad => 0,
    };
    let mut j = 2;
    let mut result = Vec::new();
    for c in seq {
        let (di, dj) = get_dir(c);
        (i, j) = (i + di, j + dj);
        if !valid_square(i, j, pad_type) {
            panic!("Invalid square");
        }
        if c == 'A' {
            result.push(match pad_type {
                PadType::Keypad => KEYPAD[i as usize][j as usize],
                PadType::Dirpad => DIR_PAD_CHARS[i as usize][j as usize],
            });
        }
    }
    result
}

fn digit_to_pos(digit: char) -> (i32, i32) {
    if digit == 'A' {
        (3, 2)
    } else {
        let (i, j) = KEYPAD_POSITIONS[digit.to_digit(16).unwrap() as usize];
        (i as i32, j as i32)
    }
}

// TODO perf
fn dirpad_char_to_pos(c: char) -> (i32, i32) {
    for i in 0..2 {
        for j in 0..3 {
            if DIR_PAD_CHARS[i][j] == c {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("Invalid char");
}

fn valid_seq(seq: &Vec<char>, mut i: i32, mut j: i32, p: &PadType) -> i32 {
    for (idx, c) in seq.iter().enumerate() {
        let (di, dj) = get_dir(*c);
        (i, j) = (i + di, j + dj);
        if !valid_square(i, j, p) {
            return idx as i32;
        }
    }
    -1
}

fn finish_sequence(i: i32, j: i32, t_i: i32, t_j: i32, p: &PadType) -> Vec<char> {
    let (diffi, diffj) = (t_i - i, t_j - j);
    let (di, dj) = (diffi.signum(), diffj.signum());
    let priority = |c: &char| match c {
        '<' => 0,
        'v' => 1,
        '^' => 2,
        '>' => 3,
        _ => panic!(), // Catch-all for unexpected characters
    };
    let mut seq: Vec<char> = Vec::new();
    for _ in 0..diffj.abs() {
        seq.push(if dj == -1 { '<' } else { '>' });
    }
    for _ in 0..diffi.abs() {
        seq.push(if di == -1 { '^' } else { 'v' });
    }
    seq.sort_by_key(priority);
    let bad_idx = valid_seq(&seq, i, j, p);
    if bad_idx != -1 {
        seq.reverse();
    }
    seq.push('A');
    seq
}

fn keypad_digit_to_seq(digit: char, i: i32, j: i32) -> Vec<char> {
    let (t_i, t_j) = digit_to_pos(digit);
    finish_sequence(i, j, t_i, t_j, &PadType::Keypad)
}

fn dirpad_char_to_seq(c: char, i: i32, j: i32) -> Vec<char> {
    let (t_i, t_j) = dirpad_char_to_pos(c);
    finish_sequence(i, j, t_i, t_j, &PadType::Dirpad)
}

fn cached_dirpad(
    curr_char: char,
    target_char: char,
    robots: i32,
    cache: &mut HashMap<(char, char, i32), i64>,
) -> i64 {
    let (s_i, s_j) = dirpad_char_to_pos(curr_char);
    let (t_i, t_j) = dirpad_char_to_pos(target_char);
    if robots == 25 {
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
            local_result += cached_dirpad(
                if idx == 0 { 'A' } else { perm[idx - 1] },
                perm[idx],
                robots + 1,
                cache,
            );
            i += get_dir(perm[idx]).0;
            j += get_dir(perm[idx]).1;
            if DIR_PAD_CHARS[i as usize][j as usize] == '#' {
                local_result = i64::MAX;
                break;
            }
        }
        if local_result == i64::MAX {
            continue;
        }
        local_result += cached_dirpad(perm[perm.len() - 1], 'A', robots + 1, cache);
        candidates.push(local_result);
    }
    let result = candidates.iter().min().unwrap();
    cache.insert((curr_char, target_char, robots), *result);
    *result
}

fn dirpad_seq_to_seq(seq: &Vec<char>, mut i: i32, mut j: i32) -> (Vec<char>, i32, i32) {
    let mut result = Vec::new();
    for c in seq {
        let seq = dirpad_char_to_seq(*c, i, j);
        result.extend(seq);
        (i, j) = dirpad_char_to_pos(*c);
    }
    (result, i, j)
}

fn keypad_seq_to_seq(seq: &str, mut i: i32, mut j: i32) -> (Vec<char>, i32, i32) {
    let mut result = Vec::new();
    for c in seq.chars() {
        let seq = keypad_digit_to_seq(c, i, j);
        result.extend(seq);
        (i, j) = digit_to_pos(c);
    }
    (result, i, j)
}

fn convert(target: &str) -> Vec<char> {
    let first = keypad_seq_to_seq(target, 3, 2).0;
    let second = dirpad_seq_to_seq(&first, 0, 2).0;
    let third = dirpad_seq_to_seq(&second, 0, 2);
    third.0
}

fn convert_part_two(target: &str) -> i64 {
    let seq = keypad_seq_to_seq(target, 3, 2).0;
    let mut cache = HashMap::new();
    let mut result = 0;
    for i in 0..seq.len() {
        result += cached_dirpad(if i == 0 { 'A' } else { seq[i - 1] }, seq[i], 1, &mut cache);
    }
    result
}

fn verify(start: &str) {
    let char_vec = convert(start);
    let curr = interpret_sequence(char_vec, &PadType::Dirpad);
    let curr = interpret_sequence(curr, &PadType::Dirpad);
    let curr = interpret_sequence(curr, &PadType::Keypad);
    let end_str: String = curr.iter().collect();
    assert_eq!(start, end_str);
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
        let char_vec = convert(line);
        verify(line);
        result += num * char_vec.len() as i64;
        result2 += num * convert_part_two(line)
    });
    println!("{}", result);
    println!("{}", result2);
}
