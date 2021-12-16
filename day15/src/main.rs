use std::collections::{BinaryHeap, HashSet};
fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let dstx = (map[0].len() - 1) as isize;
    let dsty = (map.len() - 1) as isize;
    let dst = (dstx, dsty);
    let mut prio_queue = BinaryHeap::from([(u32::MAX, (0, 0))]);
    let mut visited = HashSet::new();
    while let Some((risk, pos)) = prio_queue.pop() {
        if pos == dst {
            return u32::MAX - risk;
        }

        if !visited.insert(pos) {
            continue;
        }

        for n in neighbours(pos) {
            if (0 <= n.0 && n.0 <= dstx) && (0 <= n.1 && n.1 <= dsty) {
                if !visited.contains(&n) {
                    let r = map[n.1 as usize][n.0 as usize];
                    prio_queue.push((risk - r, n));
                }
            }
        }
    }
    unreachable!();
}

fn second_answer(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| {
            let ys = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            std::iter::repeat(ys)
                .take(5)
                .enumerate()
                .map(|(i, ys)| {
                    let mut ys = ys.clone();
                    for y in &mut ys {
                        *y += i as u32;
                        if *y > 9 {
                            *y -= 9;
                        }
                    }
                    ys
                })
                .flat_map(|ys| ys)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = std::iter::repeat(map)
        .take(5)
        .enumerate()
        .map(|(i, map)| {
            let mut map = map.clone();
            for ys in &mut map {
                for x in ys.iter_mut() {
                    *x += i as u32;
                    if *x > 9 {
                        *x -= 9;
                    }
                }
            }
            map
        })
        .flat_map(|map| map)
        .collect::<Vec<_>>();
    let dstx = (map[0].len() - 1) as isize;
    let dsty = (map.len() - 1) as isize;
    let dst = (dstx, dsty);
    let mut prio_queue = BinaryHeap::from([(u32::MAX, (0, 0))]);
    let mut visited = HashSet::new();
    while let Some((risk, pos)) = prio_queue.pop() {
        if pos == dst {
            return u32::MAX - risk;
        }

        if !visited.insert(pos) {
            continue;
        }

        for n in neighbours(pos) {
            if (0 <= n.0 && n.0 <= dstx) && (0 <= n.1 && n.1 <= dsty) {
                if !visited.contains(&n) {
                    let r = map[n.1 as usize][n.0 as usize];
                    prio_queue.push((risk - r, n));
                }
            }
        }
    }
    unreachable!();
}

fn neighbours(pos: (isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}
