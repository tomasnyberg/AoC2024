use std::{
    char,
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

enum GateType {
    And,
    Or,
    Xor,
}

struct Gate {
    input1: (char, char, char),
    input2: (char, char, char),
    output: (char, char, char),
    gate_type: GateType,
}

fn simulate(
    wire_state: &mut HashMap<(char, char, char), bool>,
    adj: &HashMap<(char, char, char), Vec<usize>>,
    gates: &[Gate],
) -> u64 {
    let mut dq: VecDeque<(char, char, char)> = VecDeque::new();
    for i in 0..45 {
        for c in ['x', 'y'].iter() {
            let key = (
                *c,
                ((i / 10) as u8 + b'0') as char,
                ((i % 10) as u8 + b'0') as char,
            );
            dq.push_back(key);
        }
    }
    while !dq.is_empty() {
        let (a, b, c) = dq.pop_front().unwrap();
        if !adj.contains_key(&(a, b, c)) {
            continue;
        }
        for idx in adj[&(a, b, c)].iter() {
            let gate = &gates[*idx];
            if wire_state.contains_key(&gate.input1) && wire_state.contains_key(&gate.input2) {
                let output = &gates[*idx].output;
                let value = match gate.gate_type {
                    GateType::And => wire_state[&gate.input1] & wire_state[&gate.input2],
                    GateType::Or => wire_state[&gate.input1] | wire_state[&gate.input2],
                    GateType::Xor => wire_state[&gate.input1] ^ wire_state[&gate.input2],
                };
                wire_state.insert(*output, value);
                dq.push_back((output.0, output.1, output.2));
            }
        }
    }
    let mut result = 0;
    for (key, value) in wire_state.iter() {
        if key.0 == 'z' {
            let num = key.1.to_string() + &key.2.to_string();
            let num = num.parse::<i32>().unwrap();
            result += (*value as u64) << num;
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let mut wire_state: HashMap<(char, char, char), bool> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();
    let mut adj: HashMap<(char, char, char), Vec<usize>> = HashMap::new();
    let mut dq: VecDeque<(char, char, char, bool)> = VecDeque::new();
    parts.next().unwrap().lines().for_each(|line| {
        let mut parts = line.split(": ");
        let key_vec: Vec<char> = parts.next().unwrap().chars().collect();
        let key = (key_vec[0], key_vec[1], key_vec[2]);
        let value = match parts.next().unwrap().parse::<i32>().unwrap() {
            0 => false,
            1 => true,
            _ => panic!("Invalid value"),
        };
        wire_state.insert(key, value);
        dq.push_back((key_vec[0], key_vec[1], key_vec[2], value));
    });
    parts.next().unwrap().lines().for_each(|line| {
        let mut parts = line.split(" -> ");
        let gate_parts: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let key_vec_a: Vec<char> = gate_parts[0].chars().collect();
        let key_vec_b: Vec<char> = gate_parts[2].chars().collect();
        let key_vec_c: Vec<char> = parts.next().unwrap().chars().collect();
        let key_a = (key_vec_a[0], key_vec_a[1], key_vec_a[2]);
        let key_b = (key_vec_b[0], key_vec_b[1], key_vec_b[2]);
        let key_c = (key_vec_c[0], key_vec_c[1], key_vec_c[2]);
        let gate: Gate = Gate {
            input1: key_a,
            input2: key_b,
            output: key_c,
            gate_type: match gate_parts[1] {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("Invalid gate type"),
            },
        };
        adj.entry(key_a).or_default().push(gates.len());
        adj.entry(key_b).or_default().push(gates.len());
        gates.push(gate);
    });
    let part_one = simulate(&mut wire_state, &adj, &gates);
    println!("{}", part_one);
}
