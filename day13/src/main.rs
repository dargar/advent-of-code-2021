use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer:");
    second_answer(input);
}

fn first_answer(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let mut dots = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let folds = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .last()
                .map(|x| {
                    let mut parts = x.split("=");
                    let dir = parts.next().unwrap().to_string();
                    let pos = parts.next().unwrap().parse::<usize>().unwrap();
                    (dir, pos)
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    for (dir, pos) in folds.into_iter().take(1) {
        match dir.as_str() {
            "x" => {
                let mut removed = Vec::new();
                for dot in &dots {
                    if dot[0] > pos {
                        removed.push(dot.clone());
                    }
                }
                dots.retain(|dot| dot[0] < pos);
                for dot in removed {
                    let x = pos - (dot[0] - pos);
                    let y = dot[1];
                    dots.insert(vec![x, y]);
                }
            }
            "y" =>  {
                let mut removed = Vec::new();
                for dot in &dots {
                    if dot[1] > pos {
                        removed.push(dot.clone());
                    }
                }
                dots.retain(|dot| dot[1] < pos);
                for dot in removed {
                    let x = dot[0];
                    let y = pos - (dot[1] - pos);
                    dots.insert(vec![x, y]);
                }
            }
            _ => panic!(),
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            if dots.contains(&vec![x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    dots.len()
}

fn second_answer(input: &str) {
    let mut parts = input.split("\n\n");
    let mut dots = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let folds = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .last()
                .map(|x| {
                    let mut parts = x.split("=");
                    let dir = parts.next().unwrap().to_string();
                    let pos = parts.next().unwrap().parse::<usize>().unwrap();
                    (dir, pos)
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    for (dir, pos) in folds {
        match dir.as_str() {
            "x" => {
                let mut removed = Vec::new();
                for dot in &dots {
                    if dot[0] > pos {
                        removed.push(dot.clone());
                    }
                }
                dots.retain(|dot| dot[0] < pos);
                for dot in removed {
                    let x = pos - (dot[0] - pos);
                    let y = dot[1];
                    dots.insert(vec![x, y]);
                }
            }
            "y" =>  {
                let mut removed = Vec::new();
                for dot in &dots {
                    if dot[1] > pos {
                        removed.push(dot.clone());
                    }
                }
                dots.retain(|dot| dot[1] < pos);
                for dot in removed {
                    let x = dot[0];
                    let y = pos - (dot[1] - pos);
                    dots.insert(vec![x, y]);
                }
            }
            _ => panic!(),
        }
    }
    for y in 0..6 {
        for x in 0..39 {
            if dots.contains(&vec![x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
