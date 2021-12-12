use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let a_to_b = map.entry(a).or_insert(HashSet::new());
        a_to_b.insert(b);
        let b_to_a = map.entry(b).or_insert(HashSet::new());
        b_to_a.insert(a);
    }
    let mut result = 0;
    let mut stack = vec![("start", Vec::new())];
    while let Some((current, path)) = stack.pop() {
        if current == "end" {
            result += 1;
            continue;
        }

        if current.chars().all(|c| c.is_lowercase()) && path.contains(&current) {
            continue;
        }

        let mut path = path.clone();
        path.push(current);
        let connections = map.get(&current).unwrap();
        for connection in connections {
            stack.push((connection, path.clone()));
        }
    }
    result
}

fn second_answer(input: &str) -> usize {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let a_to_b = map.entry(a).or_insert(HashSet::new());
        a_to_b.insert(b);
        let b_to_a = map.entry(b).or_insert(HashSet::new());
        b_to_a.insert(a);
    }
    let mut result = 0;
    let mut stack = vec![("start", Vec::new(), false)];
    while let Some((current, path, twice)) = stack.pop() {
        if current == "end" {
            result += 1;
            continue;
        }

        let mut twice = twice;
        if current.chars().all(|c| c.is_lowercase()) {
            if path.contains(&current) {
                if twice || current == "start" || current == "end" {
                    continue;
                } else {
                    twice = true;
                }
            }
        }

        let mut path = path.clone();
        path.push(current);
        let connections = map.get(&current).unwrap();
        for connection in connections {
            stack.push((connection, path.clone(), twice));
        }
    }
    result
}
