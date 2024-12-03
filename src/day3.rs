use std::{
    io::{self, Read},
    iter::Peekable,
    str::Chars,
};

const MUL: [char; 3] = ['m', 'u', 'l'];

pub fn parse_num(chars: &mut Peekable<Chars<'_>>) -> i32 {
    let mut num = 0;
    let mut flag: bool = false;
    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            flag = true;
            num = num * 10 + c.to_digit(10).unwrap() as i32;
            chars.next();
        } else {
            break;
        }
    }
    if flag {
        num
    } else {
        -1
    }
}

pub fn num_pair(chars: &mut Peekable<Chars<'_>>) -> i32 {
    if chars.peek().is_none() {
        return 0;
    }
    if *chars.peek().unwrap() != '(' {
        return 0;
    }
    chars.next();
    let num1 = parse_num(chars);
    if num1 == -1 {
        return 0;
    }
    if *chars.peek().unwrap() != ',' {
        return 0;
    }
    chars.next();
    let num2 = parse_num(chars);
    if num2 == -1 {
        return 0;
    }
    if *chars.peek().unwrap() != ')' {
        return 0;
    }
    chars.next();
    num1 * num2
}

pub fn parse_mul(chars: &mut Peekable<Chars<'_>>) -> i32 {
    let mut mul: [char; 3] = ['\0'; 3];
    for i in 0..3 {
        if chars.peek().is_none() {
            return 0;
        }
        mul[i] = *chars.peek().unwrap();
        if mul[i] != MUL[i] {
            return 0;
        }
        chars.next();
    }
    num_pair(chars)
}

pub fn part_one(input: &String) -> i32 {
    let mut chars: Peekable<Chars<'_>> = input.chars().peekable();
    let mut result = 0;
    while chars.peek().is_some() {
        result += match chars.peek().unwrap() {
            'm' => parse_mul(&mut chars),
            _ => {
                chars.next();
                0
            }
        };
    }
    result
}

pub fn part_two() -> i32 {
    0
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let part_one_result = part_one(&input);
    let part_two_result = part_two();
    println!("{}", part_one_result);
    println!("{}", part_two_result);
}
