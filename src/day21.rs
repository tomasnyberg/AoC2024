use std::{
    collections::HashMap,
    io::{self, Read},
};

#[rustfmt::skip]
const KEYPAD: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['#', '0', 'A'],
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

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let test_seq: Vec<char> = "<AvA>A^A".chars().collect();
    let result = interpret_sequence(test_seq, &PadType::Dirpad);
    println!("{:?}", result);
}
