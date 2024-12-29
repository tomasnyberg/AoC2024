use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    input1: String,
    input1idx: usize,
    input2: String,
    input2idx: usize,
    output: String,
    outputidx: usize,
    gate_type: GateType,
}

fn simulate(
    wire_state: &mut [usize],
    adj: &[Vec<usize>],
    gates: &[Gate],
    dq: &mut VecDeque<usize>,
) -> u64 {
    while !dq.is_empty() {
        let key = dq.pop_front().unwrap();
        for idx in adj[key].iter() {
            let gate = &gates[*idx];
            let (astate, bstate) = (wire_state[gate.input1idx], wire_state[gate.input2idx]);
            if astate == 2 || bstate == 2 {
                continue;
            }
            let value = match gate.gate_type {
                GateType::And => astate & bstate,
                GateType::Or => astate | bstate,
                GateType::Xor => astate ^ bstate,
            };
            wire_state[gate.outputidx] = value;
            dq.push_back(gate.outputidx);
        }
    }
    let mut result = 0;
    for g in gates.iter() {
        if let Some(num_str) = g.output.strip_prefix('z') {
            let num = num_str.parse::<i32>().unwrap();
            result += (wire_state[g.outputidx] as u64) << num;
        }
    }
    result
}

fn get_problematic(types: &HashSet<(String, GateType)>, gates: &[Gate]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for gate in gates {
        match gate.gate_type {
            GateType::And => {
                if gate.input1 != "x00"
                    && gate.input2 != "x00"
                    && !types.contains(&(gate.output.clone(), GateType::Or))
                {
                    parts.push(gate.output.clone());
                }
            }
            GateType::Or => {
                if gate.output.starts_with('z') && gate.output != "z45" {
                    parts.push(gate.output.clone());
                }
                if types.contains(&(gate.output.clone(), GateType::Or)) {
                    parts.push(gate.output.clone());
                }
            }
            GateType::Xor => {
                if gate.input1.starts_with('x') || gate.input2.starts_with('x') {
                    if gate.input1 != "x00"
                        && gate.input2 != "x00"
                        && !types.contains(&(gate.output.clone(), GateType::Xor))
                    {
                        parts.push(gate.output.clone());
                    }
                } else if !gate.output.starts_with('z') {
                    parts.push(gate.output.clone());
                }
            }
        }
    }
    parts.sort_unstable();
    parts.join(",")
}

pub fn solve(input: String) {
    let mut parts = input.split("\n\n");
    let mut wire_state: Vec<usize> = Vec::new(); // 0 is false, 1 is true, 2 is unknown
    let mut gates: Vec<Gate> = Vec::new();
    let mut adj: Vec<Vec<usize>> = vec![];
    let mut indices: HashMap<String, usize> = HashMap::new();
    let mut c = 0;
    parts.next().unwrap().lines().for_each(|line| {
        let mut parts = line.split(": ");
        let key: String = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        indices.insert(key.clone(), c);
        c += 1;
        adj.push(vec![]);
        wire_state.push(value as usize);
    });
    let mut types: HashSet<(String, GateType)> = HashSet::new();
    let mut dq: VecDeque<usize> = VecDeque::new();
    parts.next().unwrap().lines().for_each(|line| {
        let mut parts = line.split(" -> ");
        let gate_parts: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let key_a = gate_parts[0].to_string();
        let key_b = gate_parts[2].to_string();
        let key_c = parts.next().unwrap().to_string();
        for key in [key_a.clone(), key_b.clone(), key_c.clone()].iter() {
            if !indices.contains_key(key) {
                indices.insert(key.clone(), c);
                c += 1;
                adj.push(vec![]);
                wire_state.push(2);
            }
        }
        let ai = indices[&key_a];
        let bi = indices[&key_b];
        let ci = indices[&key_c];
        let gate: Gate = Gate {
            input1: key_a.clone(),
            input1idx: ai,
            input2: key_b.clone(),
            input2idx: bi,
            output: key_c.clone(),
            outputidx: ci,
            gate_type: match gate_parts[1] {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("Invalid gate type"),
            },
        };
        adj[ai].push(gates.len());
        adj[bi].push(gates.len());
        types.insert((key_b.clone(), gate.gate_type));
        types.insert((key_a.clone(), gate.gate_type));
        gates.push(gate);
        if key_a.starts_with('x') || key_a.starts_with('y') {
            dq.push_back(ai);
        }
        if key_b.starts_with('x') || key_b.starts_with('y') {
            dq.push_back(bi);
        }
    });
    let part_one = simulate(&mut wire_state, &adj, &gates, &mut dq);
    let part_two = get_problematic(&types, &gates);
    println!("{}", part_one);
    println!("{}", part_two);
}
