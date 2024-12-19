use std::{
    collections::HashMap,
    io::{self, Read},
};

struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_word: bool,
}

fn insert(root: &mut TrieNode, word: &str) {
    let mut node = root;
    for c in word.chars() {
        let index = c as usize - 'a' as usize;
        if node.children[index].is_none() {
            node.children[index] = Some(Box::new(TrieNode {
                children: (0..26).map(|_| None).collect(),
                is_word: false,
            }));
        }
        node = node.children[index].as_mut().unwrap();
    }
    node.is_word = true;
}

fn search(
    root: &TrieNode,
    mut node: &TrieNode,
    target: &Vec<char>,
    i: usize,
    seen: &mut HashMap<usize, bool>,
) -> bool {
    if i == target.len() {
        return true;
    }
    if seen.contains_key(&i) {
        return seen[&i];
    }
    for j in i..target.len() {
        if node.children[target[j] as usize - 'a' as usize].is_none() {
            seen.insert(i, false);
            return false;
        }
        node = node.children[target[j] as usize - 'a' as usize]
            .as_ref()
            .unwrap();
        if node.is_word && search(root, root, target, j + 1, seen) {
            seen.insert(i, true);
            return true;
        }
    }
    seen.insert(i, false);
    false
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split("\n\n");
    let patterns = parts.next().unwrap().split(", ").collect::<Vec<&str>>();
    let targets: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut root = TrieNode {
        children: (0..26).map(|_| None).collect(),
        is_word: false,
    };
    for pattern in patterns {
        insert(&mut root, pattern);
    }
    let mut result = 0;
    let mut seen = HashMap::new();
    for target in targets {
        if search(&root, &root, &target, 0, &mut seen) {
            result += 1;
        }
        seen.clear();
    }
    println!("{}", result);
}
