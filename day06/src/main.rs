fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", answer2(input, 80));
    println!("Second answer: {}", answer2(input, 256));
}

// First solution
// fn answer(input: &str, days: usize) -> usize {
//     let mut fish: Vec<u32> = input
//         .trim()
//         .split(',')
//         .map(|s| s.parse().unwrap())
//         .collect();
//     for _ in 0..days {
//         let mut spawned = Vec::new();
//         for f in &mut fish {
//             if *f == 0 {
//                 *f = 6;
//                 spawned.push(8);
//             } else {
//                 *f -= 1;
//             }
//         }
//         fish.extend(spawned);
//     }
//     fish.len()
// }

fn answer2(input: &str, days: usize) -> usize {
    let mut fs = vec![0; 9];
    for f in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        fs[f] += 1;
    }
    for _ in 0..days {
        let spawning = fs[0];
        for i in 0..8 {
            fs[i] = fs[i+1];
        }
        fs[6] += spawning;
        fs[8] = spawning;
    }
    fs.iter().sum()
}

