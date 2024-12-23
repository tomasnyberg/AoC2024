use std::collections::HashMap;
use std::io::{self, Read};

fn bfs(start: &str, adj: &HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut queue: Vec<&str> = vec![start];
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for _ in 0..3 {
        let mut next: Vec<&str> = vec![];
        for node in queue {
            let val = counts.entry(node).or_default();
            *val += 1;
            for nbr in adj[node].iter() {
                next.push(nbr);
            }
        }
        queue = next;
    }
    let mut biggest = 0;
    for val in counts.values() {
        if val < &biggest {
            continue;
        }
        let mut occurrences = 0;
        for other_val in counts.values() {
            if val <= other_val {
                occurrences += 1;
            }
        }
        if occurrences - 1 == *val {
            biggest = *val;
        }
    }
    if biggest == 0 {
        return vec![];
    }
    let mut result = vec![];
    for (key, val) in counts.iter() {
        if val >= &biggest {
            result.push(key.to_string());
        }
    }
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split('-').collect();
        let (a, b) = (parts[0], parts[1]);
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    });
    let mut result = 0;
    for key in adj.keys() {
        for nbr in adj[key].iter() {
            for other_nbr in adj[key].iter() {
                if nbr != other_nbr && adj[nbr].contains(other_nbr) {
                    result += if key.starts_with('t')
                        || nbr.starts_with('t')
                        || other_nbr.starts_with('t')
                    {
                        1
                    } else {
                        0
                    };
                }
            }
        }
    }
    let mut longest = 0;
    let mut longest_path: Vec<String> = vec![];
    for key in adj.keys() {
        let path = bfs(key, &adj);
        if path.len() > longest {
            longest = path.len();
            longest_path = path;
        }
    }
    longest_path.sort();
    println!("{}", longest_path.join(","));
    println!("{}", result / 6);
}
