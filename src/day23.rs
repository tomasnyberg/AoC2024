use std::collections::HashMap;
use std::io::{self, Read};

use itertools::Itertools;

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
    let mut networks: Vec<Vec<&str>> = adj.keys().map(|&key| vec![key]).collect();
    for network in networks.iter_mut() {
        for node in adj.keys() {
            if network.iter().all(|&n| adj[n].contains(node)) {
                network.push(node);
            }
        }
    }
    let longest_path = networks.iter().max_by_key(|network| network.len()).unwrap();
    println!("{}", result / 6);
    println!("{}", longest_path.iter().sorted().join(","));
}
