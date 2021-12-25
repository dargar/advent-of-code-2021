use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let burrow = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '.' || c.is_alphabetic())
                .map(move |(x, c)| ((x, y), c))
        })
        .collect::<BTreeMap<(usize, usize), char>>();
    let homes = HashMap::from([
        ((3, 2), 'A'),
        ((3, 3), 'A'),
        ((5, 2), 'B'),
        ((5, 3), 'B'),
        ((7, 2), 'C'),
        ((7, 3), 'C'),
        ((9, 2), 'D'),
        ((9, 3), 'D'),
    ]);
    let energies = HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
    let mut seen = HashSet::new();
    let mut pqueue = BinaryHeap::from([(usize::MAX, burrow)]);
    while let Some((energy, burrow)) = pqueue.pop() {
        if !seen.insert(burrow.clone()) {
            continue;
        }
        if homes
            .iter()
            .all(|(pos, home)| burrow.get(pos) == Some(home))
        {
            return usize::MAX - energy;
        }

        let moves = moves(&burrow);
        for (src, dst, steps) in moves {
            let amphipod = burrow.get(&src).unwrap();
            let next_energy = energy - energies.get(&amphipod).unwrap() * steps;
            let mut next_burrow = burrow.clone();
            next_burrow.insert(src, '.');
            next_burrow.insert(dst, *amphipod);
            pqueue.push((next_energy, next_burrow));
        }
    }
    unreachable!();
}

fn second_answer(input: &str) -> usize {
    let mut burrow = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '.' || c.is_alphabetic())
                .map(move |(x, c)| ((x, y), c))
        })
        .collect::<BTreeMap<(usize, usize), char>>();
    let mut moved = HashMap::new();
    for x in [3, 5, 7, 9] {
        let p = (x, 3);
        moved.insert(p, burrow.remove(&p).unwrap());
    }
    burrow.insert((3, 3), 'D');
    burrow.insert((5, 3), 'C');
    burrow.insert((7, 3), 'B');
    burrow.insert((9, 3), 'A');
    burrow.insert((3, 4), 'D');
    burrow.insert((5, 4), 'B');
    burrow.insert((7, 4), 'A');
    burrow.insert((9, 4), 'C');
    for ((x, y), c) in moved {
        burrow.insert((x, y + 2), c);
    }
    let homes = HashMap::from([
        ((3, 2), 'A'),
        ((3, 3), 'A'),
        ((3, 4), 'A'),
        ((3, 5), 'A'),
        ((5, 2), 'B'),
        ((5, 3), 'B'),
        ((5, 4), 'B'),
        ((5, 5), 'B'),
        ((7, 2), 'C'),
        ((7, 3), 'C'),
        ((7, 4), 'C'),
        ((7, 5), 'C'),
        ((9, 2), 'D'),
        ((9, 3), 'D'),
        ((9, 4), 'D'),
        ((9, 5), 'D'),
    ]);
    let energies = HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
    let mut seen = HashSet::new();
    let mut pqueue = BinaryHeap::from([(usize::MAX, burrow)]);
    while let Some((energy, burrow)) = pqueue.pop() {
        if !seen.insert(burrow.clone()) {
            continue;
        }
        if homes
            .iter()
            .all(|(pos, home)| burrow.get(pos) == Some(home))
        {
            return usize::MAX - energy;
        }

        let moves = moves(&burrow);
        for (src, dst, steps) in moves {
            let amphipod = burrow.get(&src).unwrap();
            let next_energy = energy - energies.get(&amphipod).unwrap() * steps;
            let mut next_burrow = burrow.clone();
            next_burrow.insert(src, '.');
            next_burrow.insert(dst, *amphipod);
            pqueue.push((next_energy, next_burrow));
        }
    }
    unreachable!();
}

fn moves(burrow: &BTreeMap<(usize, usize), char>) -> Vec<((usize, usize), (usize, usize), usize)> {
    let open = burrow
        .iter()
        .filter(|(_, c)| **c == '.')
        .map(|(pos, _)| pos.clone())
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    for (pos, c) in burrow.iter().filter(|(_, c)| **c != '.') {
        let mut reachable = HashMap::new();
        let mut queue = VecDeque::from([(*pos, 0)]);
        while let Some((pos, steps)) = queue.pop_front() {
            if open.contains(&pos) {
                reachable.insert(pos, steps);
            }
            let neighbours = vec![
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
            ];
            let candidates = neighbours
                .into_iter()
                .filter(|p| !reachable.contains_key(&p))
                .filter(|p| open.contains(&p))
                .map(|p| (p, steps + 1));
            queue.extend(candidates);
        }
        reachable.retain(|p, _| *p != (3, 1) && *p != (5, 1) && *p != (7, 1) && *p != (9, 1));
        if pos.1 == 1 {
            // Only move to home
            let home_x = match c {
                'A' => 3,
                'B' => 5,
                'C' => 7,
                'D' => 9,
                _ => panic!("Unknown amphipod {}", c),
            };
            if burrow
                .iter()
                .filter(|(_, cc)| *cc != c && **cc != '.')
                .any(|((x, _), _)| home_x == *x)
            {
                reachable.clear();
            } else {
                reachable.retain(|(x, _), _| home_x == *x);
            }
        } else {
            // Only move to hallway
            reachable.retain(|(_, y), _| *y == 1);
        }
        for (p, s) in reachable {
            result.push((*pos, p, s));
        }
    }
    result
}

pub fn print_burrow(burrow: &BTreeMap<(usize, usize), char>) {
    let width = burrow.keys().map(|(x, _)| x).max().unwrap();
    let height = burrow.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..=height + 1 {
        for x in 0..=width + 1 {
            print!("{}", burrow.get(&(x, y)).unwrap_or(&'#'));
        }
        println!();
    }
}
