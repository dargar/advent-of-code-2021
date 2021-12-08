use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    input
        .lines()
        .flat_map(|entry| entry.split(" | ").last())
        .flat_map(|output| output.split(" "))
        .filter(|digit| is_easy_digit(digit))
        .count()
}

fn is_easy_digit(digit: &str) -> bool {
    digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
}

fn second_answer(input: &str) -> usize {
    let mut numbers = HashMap::new();
    numbers.insert(vec![0, 1, 2, 4, 5, 6], 0);
    numbers.insert(vec![2, 5], 1);
    numbers.insert(vec![0, 2, 3, 4, 6], 2);
    numbers.insert(vec![0, 2, 3, 5, 6], 3);
    numbers.insert(vec![1, 2, 3, 5], 4);
    numbers.insert(vec![0, 1, 3, 5, 6], 5);
    numbers.insert(vec![0, 1, 3, 4, 5, 6], 6);
    numbers.insert(vec![0, 2, 5], 7);
    numbers.insert(vec![0, 1, 2, 3, 4, 5, 6], 8);
    numbers.insert(vec![0, 1, 2, 3, 5, 6], 9);
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(" | ");
        let signal_patterns = parts
            .next()
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .unwrap();
        let output_digits = parts
            .next()
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .unwrap();
        let ones = signal_patterns
            .iter()
            .filter(|signal_pattern| signal_pattern.len() == 2)
            .flat_map(|signal_pattern| signal_pattern.chars())
            .collect::<HashSet<char>>();
        let fours = signal_patterns
            .iter()
            .filter(|signal_pattern| signal_pattern.len() == 4)
            .flat_map(|signal_pattern| signal_pattern.chars())
            .collect::<HashSet<char>>();
        let sevens = signal_patterns
            .iter()
            .filter(|signal_pattern| signal_pattern.len() == 3)
            .flat_map(|signal_pattern| signal_pattern.chars())
            .collect::<HashSet<char>>();
        let eights = signal_patterns
            .iter()
            .filter(|signal_pattern| signal_pattern.len() == 7)
            .flat_map(|signal_pattern| signal_pattern.chars())
            .collect::<HashSet<char>>();
        assert!(!eights.is_empty());
        let mut candidates = vec![HashSet::new(); 7];

        //  0
        // 1 2
        //  3
        // 4 5
        //  6

        let top = sevens.difference(&ones).cloned().collect::<HashSet<char>>();
        candidates[0].extend(top.clone());

        candidates[2].extend(ones.clone());
        candidates[5].extend(ones.clone());

        let top_left_middle = fours.difference(&ones).cloned().collect::<HashSet<char>>();
        candidates[1].extend(top_left_middle.clone());
        candidates[3].extend(top_left_middle.clone());

        let mut remaining = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        remaining = &remaining - &candidates[0];
        remaining = &remaining - &candidates[1];
        remaining = &remaining - &candidates[2];
        remaining = &remaining - &candidates[3];
        remaining = &remaining - &candidates[5];

        candidates[4] = remaining.clone();
        candidates[6] = remaining.clone();

        let mut stack = vec![candidates];
        let mut tries = Vec::new();
        let mut result = None;
        while let Some(candidate) = stack.pop() {
            if candidate.iter().all(|xs| xs.len() == 1) {
                let mut ok = true;
                for output_digit in output_digits.iter().chain(&signal_patterns) {
                    let mut r = candidate
                        .iter()
                        .map(|xs| xs.iter().next().unwrap())
                        .enumerate()
                        .filter(|(_, x)| output_digit.contains(**x))
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    r.sort();
                    if !numbers.contains_key(&r) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    result = Some(candidate);
                    break;
                }
            }
            if candidate.iter().any(|xs| xs.is_empty()) {
                tries.push(candidate.clone());
                break;
            }
            for (i, xs) in candidate.iter().enumerate() {
                if xs.len() > 1 {
                    for x in xs {
                        let mut c = candidate.clone();
                        for c in c.iter_mut() {
                            c.remove(x);
                        }
                        c[i] = HashSet::from([*x]);
                        if !tries.contains(&c) {
                            stack.push(c);
                        }
                    }
                    break;
                }
            }
        }

        if let Some(r) = result {
            let mut output = 0;
            for output_digit in output_digits {
                output *= 10;
                let mut r = r
                    .iter()
                    .map(|xs| xs.iter().next().unwrap())
                    .enumerate()
                    .filter(|(_, x)| output_digit.contains(**x))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();
                r.sort();
                output += numbers.get(&r).unwrap();
            }
            total += output;
        }
    }
    total
}
