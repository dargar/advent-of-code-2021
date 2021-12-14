use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let template = parts.next().unwrap().chars().collect::<Vec<char>>();
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let pair = parts.next().unwrap().chars().collect::<Vec<char>>();
            let insertion = parts.next().unwrap().chars().next().unwrap();
            ((pair[0], pair[1]), insertion)
        })
        .collect::<HashMap<(char, char), char>>();
    let mut current = template;
    for _ in 0..10 {
        let mut next = Vec::new();
        for pair in current.windows(2) {
            let a = pair[0];
            let b = pair[1];
            next.push(a);
            next.push(*rules.get(&(a, b)).unwrap());
        }
        next.push(*current.last().unwrap());
        current = next;
    }
    let mut freqs = HashMap::new();
    for c in current {
        *freqs.entry(c).or_insert(0) += 1;
    }
    let min = freqs.values().min().unwrap();
    let max = freqs.values().max().unwrap();
    max - min
}

fn second_answer(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let template = parts.next().unwrap().chars().collect::<Vec<char>>();
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let pair = parts.next().unwrap().chars().collect::<Vec<char>>();
            let insertion = parts.next().unwrap().chars().next().unwrap();
            ((pair[0], pair[1]), insertion)
        })
        .collect::<HashMap<(char, char), char>>();
    let mut result = HashMap::new();
    let mut cache = HashMap::new();
    for pair in template.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let fs = freqs(&rules, (a, b), 40, &mut cache);
        for (k, v) in fs {
            *result.entry(k).or_insert(0) += v;
        }
    }
    *result.entry(*template.last().unwrap()).or_insert(0) += 1;
    let min = result.values().min().unwrap();
    let max = result.values().max().unwrap();
    max - min
}

fn freqs(
    rules: &HashMap<(char, char), char>,
    (a, b): (char, char),
    steps: usize,
    cache: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if let Some(fs) = cache.get(&(a, b, steps)) {
        return fs.clone();
    }

    if steps == 0 {
        return [(a, 1)].into();
    }

    let x = *rules.get(&(a, b)).unwrap();

    let lhs = freqs(rules, (a, x), steps - 1, cache);
    let rhs = freqs(rules, (x, b), steps - 1, cache);

    let mut merged = lhs;
    for (k, v) in rhs {
        *merged.entry(k).or_insert(0) += v;
    }
    if !cache.contains_key(&(a, b, steps)) {
        cache.insert((a, b, steps), merged.clone());
    }
    merged
}
