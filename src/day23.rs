use std::collections::HashMap;
use std::io::{self, Read};

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
    println!("{}", result / 6);
}
