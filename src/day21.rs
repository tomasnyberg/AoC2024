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

fn finish_sequence(i: i32, j: i32, t_i: i32, t_j: i32, p: &PadType) -> Vec<char> {
    let (diffi, diffj) = (t_i - i, t_j - j);
    let (di, dj) = (diffi.signum(), diffj.signum());
    let mut seqa: Vec<char> = Vec::new();
    for _ in 0..diffi.abs() {
        seqa.push(if di == -1 { '^' } else { 'v' });
    }
    for _ in 0..diffj.abs() {
        seqa.push(if dj == -1 { '<' } else { '>' });
    }
    let mut seqb: Vec<char> = Vec::new();
    for _ in 0..diffj.abs() {
        seqb.push(if dj == -1 { '<' } else { '>' });
    }
    for _ in 0..diffi.abs() {
        seqb.push(if di == -1 { '^' } else { 'v' });
    }
    if !valid_square(t_i, j, p) {
        seqa = seqb;
    }
    seqa.push('A');
    seqa
}

fn keypad_digit_to_seq(digit: char, i: i32, j: i32) -> Vec<char> {
    let (t_i, t_j) = digit_to_pos(digit);
    finish_sequence(i, j, t_i, t_j, &PadType::Keypad)
}

fn dirpad_char_to_seq(c: char, i: i32, j: i32) -> Vec<char> {
    let (t_i, t_j) = dirpad_char_to_pos(c);
    finish_sequence(i, j, t_i, t_j, &PadType::Dirpad)
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
    input.lines().for_each(|line| {
        let num = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let char_vec = convert(line);
        verify(line);
        result += num * char_vec.len() as i32;
    });
    println!("{}", result);
}
