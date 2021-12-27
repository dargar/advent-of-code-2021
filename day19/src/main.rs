use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let scanners = input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|beacon| {
                    beacon
                        .split(",")
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut known = HashMap::from([(0, ((0, 0, 0), 0))]);
    let mut unknown = (1..scanners.len()).collect::<VecDeque<_>>();
    while let Some(scanner) = unknown.pop_front() {
        let mut new_known = HashMap::new();
        for (ref_id, (ref_pos, ref_rot)) in &known {
            let reference = scanners[*ref_id].clone();
            let reference = rotations(&reference)[*ref_rot].clone();
            let b = scanners[scanner].clone();
            for (rot_id, rotation) in rotations(&b).into_iter().enumerate() {
                for refpt in &reference {
                    for rotpt in &rotation {
                        let mut diffx = (refpt[0] - rotpt[0]).abs();
                        if refpt[0] < rotpt[0] {
                            diffx = -diffx
                        }
                        let mut diffy = (refpt[1] - rotpt[1]).abs();
                        if refpt[1] < rotpt[1] {
                            diffy = -diffy
                        }
                        let mut diffz = (refpt[2] - rotpt[2]).abs();
                        if refpt[2] < rotpt[2] {
                            diffz = -diffz
                        }
                        let rs = rotation
                            .iter()
                            .map(|r| vec![r[0] + diffx, r[1] + diffy, r[2] + diffz])
                            .collect::<HashSet<_>>();
                        let both = &reference & &rs;
                        if both.len() >= 12 {
                            let offset_pos =
                                (ref_pos.0 + diffx, ref_pos.1 + diffy, ref_pos.2 + diffz);
                            new_known.insert(scanner, (offset_pos, rot_id));
                        }
                    }
                }
            }
        }
        if new_known.contains_key(&scanner) {
            assert!(!known.contains_key(&scanner));
            known.extend(new_known);
        } else {
            unknown.push_back(scanner);
        }
    }
    let mut result = HashMap::new();
    for (scanner, (offset, rotation)) in known {
        let beacons = &scanners[scanner];
        let rotations = &rotations(beacons)[rotation];
        for b in rotations {
            let pt = (offset.0 + b[0], offset.1 + b[1], offset.2 + b[2]);
            *result.entry(pt).or_insert(0) += 1;
        }
    }
    result.len()
}

fn second_answer(input: &str) -> i32 {
    let scanners = input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|beacon| {
                    beacon
                        .split(",")
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut known = HashMap::from([(0, ((0, 0, 0), 0))]);
    let mut unknown = (1..scanners.len()).collect::<VecDeque<_>>();
    while let Some(scanner) = unknown.pop_front() {
        let mut new_known = HashMap::new();
        for (ref_id, (ref_pos, ref_rot)) in &known {
            let reference = scanners[*ref_id].clone();
            let reference = rotations(&reference)[*ref_rot].clone();
            let b = scanners[scanner].clone();
            for (rot_id, rotation) in rotations(&b).into_iter().enumerate() {
                for refpt in &reference {
                    for rotpt in &rotation {
                        let mut diffx = (refpt[0] - rotpt[0]).abs();
                        if refpt[0] < rotpt[0] {
                            diffx = -diffx
                        }
                        let mut diffy = (refpt[1] - rotpt[1]).abs();
                        if refpt[1] < rotpt[1] {
                            diffy = -diffy
                        }
                        let mut diffz = (refpt[2] - rotpt[2]).abs();
                        if refpt[2] < rotpt[2] {
                            diffz = -diffz
                        }
                        let rs = rotation
                            .iter()
                            .map(|r| vec![r[0] + diffx, r[1] + diffy, r[2] + diffz])
                            .collect::<HashSet<_>>();
                        let both = &reference & &rs;
                        if both.len() >= 12 {
                            let offset_pos =
                                (ref_pos.0 + diffx, ref_pos.1 + diffy, ref_pos.2 + diffz);
                            new_known.insert(scanner, (offset_pos, rot_id));
                        }
                    }
                }
            }
        }
        if new_known.contains_key(&scanner) {
            assert!(!known.contains_key(&scanner));
            known.extend(new_known);
        } else {
            unknown.push_back(scanner);
        }
    }
    let mut result = 0;
    for (p0, _) in known.values() {
        for (p1, _) in known.values() {
            let dist = (p0.0 - p1.0).abs() + (p0.1 - p1.1).abs() + (p0.2 - p1.2).abs();
            result = std::cmp::max(result, dist);
        }
    }
    result
}

fn rotations(beacons: &HashSet<Vec<i32>>) -> Vec<HashSet<Vec<i32>>> {
    let mut rotated = vec![HashSet::new(); 24];
    for beacon in beacons {
        let x = beacon[0];
        let y = beacon[1];
        let z = beacon[2];
        let mut rs = Vec::new();
        // +x
        let mut v = vec![x, y, z];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        // -x
        let mut v = vec![-x, y, -z];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        // +y
        let mut v = vec![y, -x, z];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        // -y
        let mut v = vec![-y, x, z];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        // +z
        let mut v = vec![z, y, -x];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        // -z
        let mut v = vec![-z, y, x];
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());
        v = roll(v);
        rs.push(v.clone());

        for (i, r) in rs.into_iter().enumerate() {
            rotated[i].insert(r);
        }
    }
    rotated
}

fn roll(v3: Vec<i32>) -> Vec<i32> {
    let x = v3[0];
    let y = v3[1];
    let z = v3[2];
    vec![x, z, -y]
}
