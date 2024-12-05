use std::collections::HashMap;
use std::io::{self, Read};

fn part_one(lists: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut result = 0;
    for list in lists {
        let indices: HashMap<i32, usize> = list.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut flag = false;
        list.iter().enumerate().for_each(|(i, &v)| {
            for y in rules.get(&v).unwrap_or(&vec![]) {
                if indices.get(y).unwrap_or(&1000) < &i {
                    flag = true;
                    break;
                }
            }
        });
        if !flag {
            result += list[list.len() / 2];
        }
    }
    result
}

fn part_two() {}

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
    let result = part_one(&lists, &rules);
    println!("{}", result);
}
