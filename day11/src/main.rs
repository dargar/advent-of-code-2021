use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut squidgrid = vec![vec![0; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            squidgrid[y][x] = c.to_digit(10).unwrap();
        }
    }
    let mut result = 0;
    let mut current = squidgrid;
    for _ in 0..100 {
        let (next, flashes) = step(current);
        result += flashes;
        current = next;
    }
    result
}

fn second_answer(input: &str) -> usize {
    let mut squidgrid = vec![vec![0; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            squidgrid[y][x] = c.to_digit(10).unwrap();
        }
    }
    let mut current = squidgrid;
    for n in 1.. {
        let (next, flashes) = step(current);
        if flashes == 100 {
            return n;
        } else {
            current = next;
        }
    }
    unreachable!();
}

fn step(mut squidgrid: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize) {
    let mut flashing = VecDeque::new();
    for (r, row) in squidgrid.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            *col += 1;
            if *col > 9 {
                flashing.push_back((r, c));
            }
        }
    }
    let mut flashed = HashSet::new();
    while let Some(i) = flashing.pop_front() {
        if !flashed.contains(&i) {
            flashed.insert(i);
            for n in neighbours(i) {
                squidgrid[n.0][n.1] += 1;
                if squidgrid[n.0][n.1] > 9 {
                    flashing.push_back(n);
                }
            }
        }
    }
    for (r, c) in &flashed {
        squidgrid[*r][*c] = 0;
    }
    (squidgrid, flashed.len())
}

fn neighbours((r, c): (usize, usize)) -> Vec<(usize, usize)> {
    let r = r as isize;
    let c = c as isize;
    let mut result = Vec::new();
    for r1 in r - 1..=r + 1 {
        for c1 in c - 1..=c + 1 {
            if (0 <= r1 && r1 < 10) && (0 <= c1 && c1 < 10) && (r1 != r || c1 != c) {
                result.push((r1 as usize, c1 as usize));
            }
        }
    }
    result
}
