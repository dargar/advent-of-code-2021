fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut syntax_error_score = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for symbol in line.chars() {
            if symbol == '(' || symbol == '{' || symbol == '<' || symbol == '[' {
                stack.push(symbol);
            } else {
                let top = stack.pop().unwrap();
                let score = match (top, symbol) {
                    ('{', ')') | ('<', ')') | ('[', ')') => 3,
                    ('(', ']') | ('{', ']') | ('<', ']') => 57,
                    ('(', '}') | ('<', '}') | ('[', '}') => 1197,
                    ('[', '>') | ('(', '>') | ('{', '>') => 25137,
                    _ => 0,
                };
                if score != 0 {
                    syntax_error_score += score;
                    break;
                }
            }
        }
    }
    syntax_error_score
}

fn second_answer(input: &str) -> usize {
    let mut autocompletion_scores = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        for symbol in line.chars() {
            if symbol == '(' || symbol == '{' || symbol == '<' || symbol == '[' {
                stack.push(symbol);
            } else {
                let top = stack.pop().unwrap();
                let score = match (top, symbol) {
                    ('{', ')') | ('<', ')') | ('[', ')') => 3,
                    ('(', ']') | ('{', ']') | ('<', ']') => 57,
                    ('(', '}') | ('<', '}') | ('[', '}') => 1197,
                    ('[', '>') | ('(', '>') | ('{', '>') => 25137,
                    _ => 0,
                };
                if score != 0 {
                    stack.clear();
                    break;
                }
            }
        }
        let mut score = 0;
        for symbol in stack.iter().rev() {
            score *= 5;
            let value = match symbol {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
            score += value;
        }
        if score != 0 {
            autocompletion_scores.push(score);
        }
    }
    autocompletion_scores.sort();
    autocompletion_scores[autocompletion_scores.len() / 2]
}

