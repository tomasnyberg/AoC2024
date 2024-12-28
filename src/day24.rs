use std::{
    char,
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
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

fn generate_input(x: i64, y: i64) -> HashMap<(char, char, char), bool> {
    let mut result: HashMap<(char, char, char), bool> = HashMap::new();
    for i in 0..45 {
        let x_key = (
            'x',
            ((i / 10) as u8 + b'0') as char,
            ((i % 10) as u8 + b'0') as char,
        );
        let y_key = (
            'y',
            ((i / 10) as u8 + b'0') as char,
            ((i % 10) as u8 + b'0') as char,
        );
        result.insert(x_key, (1 << i) & x != 0);
        result.insert(y_key, (1 << i) & y != 0);
    }
    result
}

fn print_soln(indices: Vec<usize>, gates: &[Gate]) {
    let mut tuples: Vec<(char, char, char)> = indices.iter().map(|&x| gates[x].output).collect();
    tuples.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1).then(a.2.cmp(&b.2))));
    for tuple in tuples.iter() {
        print!("{}{}{},", tuple.0, tuple.1, tuple.2);
    }
    println!();
}

fn test_gates(
    adj: &HashMap<(char, char, char), Vec<usize>>,
    gates: &[Gate],
    bits: &[usize],
) -> bool {
    for bit in bits.iter() {
        let mut x = generate_input(1 << bit, 0);
        let mut y = generate_input(0, 1 << bit);
        let mut both = generate_input(1 << bit, 1 << bit);
        let x_result = simulate(&mut x, adj, gates);
        let y_result = simulate(&mut y, adj, gates);
        let both_result = simulate(&mut both, adj, gates);
        //if x_result != 1 << bit {
        //    println!("X failed for bit {}", bit);
        //}
        //if y_result != 1 << bit {
        //    println!("Y failed for bit {}", bit);
        //}
        //if both_result != 1 << (bit + 1) {
        //    println!("Both failed for bit {}", bit);
        //}
        if x_result != 1 << bit || y_result != 1 << bit || both_result != 1 << (bit + 1) {
            return false;
        }
    }
    true
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let mut wire_state: HashMap<(char, char, char), bool> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();
    let mut adj: HashMap<(char, char, char), Vec<usize>> = HashMap::new();
    let mut reverse_adj: HashMap<(char, char, char), usize> = HashMap::new();
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
    let mut candidates: Vec<usize> = Vec::new();
    let mut problematic: Vec<usize> = Vec::new();
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
        // All z gates should be XOR (except for the last one)
        if key_c.0 == 'z' && gate.gate_type != GateType::Xor && !(key_c.1 == '4' && key_c.2 == '5')
        {
            problematic.push(gates.len());
        // Don't think we can swap with another one since then another one since then that one is also not
        // XOR? also only take xor outputs
        } else if key_c.0 != 'z' && gate.gate_type == GateType::Xor {
            candidates.push(gates.len());
        }
        if key_c == ('z', '3', '5') {
            println!("{:?}", gates.len());
        }
        if reverse_adj.contains_key(&key_c) {
            panic!();
        }
        reverse_adj.insert(key_c, gates.len());
        adj.entry(key_a).or_default().push(gates.len());
        adj.entry(key_b).or_default().push(gates.len());
        gates.push(gate);
    });
    let part_one = simulate(&mut wire_state, &adj, &gates);
    println!("{}", part_one);
    //let mut count = 0;
    //let bits = vec![10, 15, 25];
    //for permutation in candidates.iter().permutations(3) {
    //    count += 1;
    //    if count < 22000 {
    //        continue;
    //    }
    //    for idx in 0..3 {
    //        let temp = gates[*permutation[idx]].output;
    //        gates[*permutation[idx]].output = gates[problematic[idx]].output;
    //        gates[problematic[idx]].output = temp;
    //    }
    //    if test_gates(&adj, &gates, &bits) {
    //        println!("{:?}", permutation);
    //    }
    //    for idx in 0..3 {
    //        let temp = gates[*permutation[idx]].output;
    //        gates[*permutation[idx]].output = gates[problematic[idx]].output;
    //        gates[problematic[idx]].output = temp;
    //    }
    //    if count > 23000 {
    //        break;
    //    }
    //}
    //let soln = vec![51, 159, 36];
    let soln = [51, 169, 36];
    let bits: Vec<usize> = (0..45).collect();
    for i in 0..3 {
        let temp = gates[soln[i]].output;
        gates[soln[i]].output = gates[problematic[i]].output;
        gates[problematic[i]].output = temp;
    }
    let full = vec![51, 169, 36, 55, 121, 135, 64, 94];
    print_soln(full, &gates);
    for i in 0..gates.len() {
        if gates[i].output.0 == 'z' {
            continue;
        }
        println!("{}", i);
        for j in (i + 1)..gates.len() {
            if gates[j].output.0 == 'z' {
                continue;
            }
            let temp = gates[i].output;
            gates[i].output = gates[j].output;
            gates[j].output = temp;
            // found the solution at 64, 94
            if test_gates(&adj, &gates, &bits) {
                println!("{}, {}", i, j);
            }
            let temp = gates[i].output;
            gates[i].output = gates[j].output;
            gates[j].output = temp;
        }
    }
    //for c in candidates.iter() {
    //    let temp = gates[*c].output;
    //    gates[*c].output = gates[fourth_bad_idx].output;
    //    gates[fourth_bad_idx].output = temp;
    //    if test_gates(&adj, &gates, &bits) {
    //        println!("{:?}", c);
    //    }
    //    let temp = gates[*c].output;
    //    gates[*c].output = gates[fourth_bad_idx].output;
    //    gates[fourth_bad_idx].output = temp;
    //}
    println!("{}", test_gates(&adj, &gates, &bits));
    println!("{}, {}", candidates.len(), problematic.len());
}
