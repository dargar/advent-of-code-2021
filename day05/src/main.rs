use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let points = input.lines()
        .map(Line::read_line)
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.points())
        .collect::<Vec<_>>();
    let mut plane = HashMap::new();
    for p in points {
        *plane.entry(p).or_insert(0) += 1;
    }
    plane.values()
        .filter(|v| **v > 1)
        .count()
}

fn second_answer(input: &str) -> usize {
    let points = input.lines()
        .map(Line::read_line)
        .flat_map(|line| line.points())
        .collect::<Vec<_>>();
    let mut plane = HashMap::new();
    for p in points {
        *plane.entry(p).or_insert(0) += 1;
    }
    plane.values()
        .filter(|v| **v > 1)
        .count()
}

#[derive(Debug)]
struct Line {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Line {
    fn new(ns: &[usize]) -> Line {
        assert!(ns.len() == 4);
        if ns[0] == ns[2] {
            let x = ns[0];
            let y0 = std::cmp::min(ns[1], ns[3]);
            let y1 = std::cmp::max(ns[1], ns[3]);
            Line { x0: x, y0: y0, x1: x, y1: y1 }
        } else if ns[1] == ns[3] {
            let y = ns[1];
            let x0 = std::cmp::min(ns[0], ns[2]);
            let x1 = std::cmp::max(ns[0], ns[2]);
            Line { x0: x0, y0: y, x1: x1, y1: y }
        } else {
            let x0 = ns[0];
            let y0 = ns[1];
            let x1 = ns[2];
            let y1 = ns[3];
            if x0 < x1 {
                Line { x0, y0, x1, y1 }
            } else {
                Line { x0: x1, y0: y1, x1: x0, y1: y0 }
            }
        }
    }

    fn read_line(s: &str) -> Line {
        let ns = s.split(" -> ")
            .flat_map(|s| s.split(","))
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Line::new(&ns)
    }

    fn is_horizontal(&self) -> bool {
        self.y0 == self.y1
    }

    fn is_vertical(&self) -> bool {
        self.x0 == self.x1
    }

    fn points(&self) -> Vec<(usize, usize)> {
        let points = if self.is_horizontal() {
            let y = self.y0;
            (self.x0..=self.x1).map(|x| (x, y)).collect()
        } else if self.is_vertical() {
            let x = self.x0;
            (self.y0..=self.y1).map(|y| (x, y)).collect()
        } else {
            if self.y0 < self.y1 {
                (0..).take_while(|n| self.x0 + n <= self.x1).map(|n| (self.x0 + n, self.y0 + n)).collect()
            } else {
                (0..).take_while(|n| self.y1 + n <= self.y0).map(|n| (self.x1 - n, self.y1 + n)).collect()
            }
        };
        points
    }
}

