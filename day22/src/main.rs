use std::collections::HashSet;
use std::ops::Sub;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!(
        "Second answer: {} (Not correct, solution unfinished)",
        second_answer(input)
    );
}

fn first_answer(input: &str) -> usize {
    let mut reactor_core = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let state = parts.next().unwrap();
        let ranges = parts
            .next()
            .unwrap()
            .split(",")
            .map(|coord| {
                coord
                    .split("=")
                    .last()
                    .unwrap()
                    .split("..")
                    .flat_map(|n| n.parse())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<_>>>();
        if ranges
            .iter()
            .flat_map(|range| range.iter())
            .all(|n| -50 <= *n && *n <= 50)
        {
            for x in ranges[0][0]..=ranges[0][1] {
                for y in ranges[1][0]..=ranges[1][1] {
                    for z in ranges[2][0]..=ranges[2][1] {
                        match state {
                            "on" => {
                                reactor_core.insert((x, y, z));
                            }
                            "off" => {
                                reactor_core.remove(&(x, y, z));
                            }
                            _ => {
                                panic!();
                            }
                        }
                    }
                }
            }
        }
    }
    reactor_core.len()
}

fn second_answer(input: &str) -> i64 {
    let mut reactor_core: Vec<Cuboid> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let state = parts.next().unwrap();
        let ranges = parts
            .next()
            .unwrap()
            .split(",")
            .flat_map(|coord| {
                coord
                    .split("=")
                    .last()
                    .unwrap()
                    .split("..")
                    .flat_map(|n| n.parse())
            })
            .collect::<Vec<_>>();
        let cuboid = Cuboid::new(&ranges);
        match state {
            "on" => {
                reactor_core.push(cuboid);
            }
            "off" => {
                let mut next_reactor_core = Vec::new();
                for other in reactor_core.into_iter() {
                    let splits = other - cuboid;
                    next_reactor_core.extend(splits);
                }
                reactor_core = next_reactor_core;
            }
            _ => panic!("Unknown state: {}", state),
        }
    }
    reactor_core.iter().map(Cuboid::volume).sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cuboid {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
}

impl Cuboid {
    fn new(points: &Vec<i64>) -> Self {
        Self {
            x0: points[0],
            x1: points[1],
            y0: points[2],
            y1: points[3],
            z0: points[4],
            z1: points[5],
        }
    }

    fn volume(&self) -> i64 {
        (self.x0 - self.x1).abs() * (self.y0 - self.y1).abs() * (self.z0 - self.z1).abs()
    }
}

impl Sub for Cuboid {
    type Output = Vec<Cuboid>;

    fn sub(self, other: Self) -> Self::Output {
        // TODO: Split self into smaller cuboids if other intersects self.
        vec![self]
    }
}
