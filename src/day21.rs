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

fn keypad_digit_to_seq(digit: char, i: i32, j: i32) -> Vec<char> {
    let (t_i, t_j) = digit_to_pos(digit);
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
    // If moving vertically straight to it is invalid, then move horizontally first
    if !valid_square(t_i, j, &PadType::Keypad) {
        seqa = seqb;
    }
    seqa.push('A');
    seqa
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let test_dirpad_seq: Vec<char> =
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
            .chars()
            .collect();
    let expected: Vec<char> = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect();
    let result = interpret_sequence(test_dirpad_seq, &PadType::Dirpad);
    assert_eq!(result, expected);
    let test_keypad_seq: Vec<char> = "<A^A>^^AvvvA".chars().collect();
    let result = interpret_sequence(test_keypad_seq, &PadType::Keypad);
    assert_eq!(result, vec!['0', '2', '9', 'A']);
    let (mut i, mut j) = (3, 2);
    let mut full_seq: Vec<char> = Vec::new();
    let target = "029A";
    for target_c in target.chars() {
        let seq = keypad_digit_to_seq(target_c, i, j);
        full_seq.extend(seq);
        (i, j) = digit_to_pos(target_c);
    }
    assert_eq!(
        full_seq,
        vec!['<', 'A', '^', 'A', '^', '^', '>', 'A', 'v', 'v', 'v', 'A']
    );
}
