use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
}

fn first_answer(_input: &str) -> usize {
    let values = vec![
        (26,  -9,  6),
        (26,  -4, 15),
        (26,  -5, 14),
        ( 1,  12,  4),
        (26,  -9, 14),
        (26,  -6,  8),
        ( 1,  10,  8),
        (26, -11,  1),
        ( 1,  14, 12),
        (26,  -8,  3),
        ( 1,  14,  8),
        ( 1,  10, 16),
        ( 1,  14,  8),
        ( 1,  11,  7),
    ];
    let ds = largest(&values, &vec![0], &mut HashMap::new());
    undigits(&ds)
}

fn largest(values: &[(i32, i32, i32)], targets: &[i32], cache: &mut HashMap<(Vec<(i32, i32, i32)>, Vec<i32>), Vec<i32>>) -> Vec<i32> {
    if let Some(result) = cache.get(&(values.to_vec(), targets.to_vec())) {
        return result.to_vec();
    }

    if values.is_empty() {
        return Vec::new();
    }

    if targets.is_empty() {
        return Vec::new();
    }

    let (a, b, c) = values[0];
    for w in (1..=9).rev() {
        let zs = g(w, a, b, c, targets);
        let mut ds = largest(&values[1..], &zs, cache);
        if ds.len() == values.len() - 1 {
            ds.push(w);
            if ds.len() == 14 {
                if input_f(&ds) != 0 {
                    continue;
                }
            }
            cache.insert((values.to_vec(), targets.to_vec()), ds.clone());
            return ds;
        }
    }

    cache.insert((values.to_vec(), targets.to_vec()), Vec::new());

    Vec::new()
}

fn g(w: i32, a: i32, b: i32, c: i32, ts: &[i32]) -> Vec<i32> {
    let mut results = Vec::new();
    for zz in 0..350_000 {
        let z = zz;
        let x = z % 26;
        let z = z / a;
        let x = x + b;
        let x = if x != w {1} else {0};
        let y = (25 * x) + 1;
        let z = z * y;
        let y = (w + c) * x;
        let z = z + y;
        if ts.contains(&z) {
            results.push(zz);
        }
    }
    results
}

fn undigits(ns: &[i32]) -> usize {
    let mut result = 0;
    for n in ns {
        assert!(0 <= *n && *n <= 9);
        result *= 10;
        result += *n as usize;
    }
    result
}

fn input_f(ws: &Vec<i32>) -> i32 {
    let z = 0;
    let z = f(ws[0],  z,  1,  11,  7);
    let z = f(ws[1],  z,  1,  14,  8);
    let z = f(ws[2],  z,  1,  10, 16);
    let z = f(ws[3],  z,  1,  14,  8);
    let z = f(ws[4],  z, 26,  -8,  3);
    let z = f(ws[5],  z,  1,  14, 12);
    let z = f(ws[6],  z, 26, -11,  1);
    let z = f(ws[7],  z,  1,  10,  8);
    let z = f(ws[8],  z, 26,  -6,  8);
    let z = f(ws[9],  z, 26,  -9, 14);
    let z = f(ws[10], z,  1,  12,  4);
    let z = f(ws[11], z, 26,  -5, 14);
    let z = f(ws[12], z, 26,  -4, 15);
    let z = f(ws[13], z, 26,  -9,  6);
    z
}

fn f(w: i32, z: i32, a: i32, b: i32, c: i32) -> i32 {
    let x = z % 26;
    let z = z / a;
    let x = x + b;
    let x = if x != w {1} else {0};
    let y = (25 * x) + 1;
    let z = z * y;
    let y = (w + c) * x;
    z + y
}
