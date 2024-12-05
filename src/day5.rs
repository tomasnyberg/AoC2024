use std::collections::HashMap;
use std::io::{self, Read};

fn part_one(lists: &[Vec<i32>], rules: &HashMap<i32, Vec<i32>>) -> (i32, Vec<usize>) {
    let mut result = 0;
    let mut bad_indices: Vec<usize> = vec![];
    lists.iter().enumerate().for_each(|(i, list)| {
        let indices: HashMap<i32, usize> = list.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let flag = list.iter().enumerate().any(|(i, &v)| {
            for y in rules.get(&v).unwrap_or(&vec![]) {
                if indices.get(y).unwrap_or(&1000) < &i {
                    return true;
                }
            }
            false
        });
        if !flag {
            result += list[list.len() / 2];
        } else {
            bad_indices.push(i);
        }
    });
    (result, bad_indices)
}

fn dfs(
    curr: i32,
    visited: &mut Vec<bool>,
    topsort: &mut Vec<i32>,
    rules: &HashMap<i32, Vec<i32>>,
    indices: &HashMap<i32, usize>,
) {
    if visited[*indices.get(&curr).unwrap()] {
        return;
    }
    visited[*indices.get(&curr).unwrap()] = true;
    for nbr in rules.get(&curr).unwrap_or(&vec![]) {
        if !indices.contains_key(nbr) {
            continue;
        }
        dfs(*nbr, visited, topsort, rules, indices);
    }
    topsort.push(curr);
}

fn part_two(lists: &[Vec<i32>], rules: &HashMap<i32, Vec<i32>>, bad_indices: Vec<usize>) -> i32 {
    let mut result: i32 = 0;
    bad_indices.iter().for_each(|&i| {
        let list = &lists[i];
        let indices: HashMap<i32, usize> = list.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut visited: Vec<bool> = vec![false; list.len()];
        let mut topsort: Vec<i32> = vec![];
        for i in 0..indices.len() {
            if !visited[i] {
                dfs(list[i], &mut visited, &mut topsort, rules, &indices);
            }
        }
        result += topsort[topsort.len() / 2];
    });
    result
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let edges: Vec<&str> = parts[0].lines().collect();
    let lists: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    edges.iter().for_each(|edge| {
        let mut parts = edge.split('|');
        let a: i32 = parts.next().unwrap().parse::<i32>().unwrap();
        let b: i32 = parts.next().unwrap().parse::<i32>().unwrap();
        rules.entry(a).or_default().push(b);
    });
    let (result_part_one, bad_indices) = part_one(&lists, &rules);
    let result_part_two = part_two(&lists, &rules, bad_indices);
    println!("{}", result_part_one);
    println!("{}", result_part_two);
}
