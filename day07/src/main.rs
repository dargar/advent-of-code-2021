fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> isize {
    let positions = input
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|p| positions.iter().map(|c| c - p).map(|c| c.abs()).sum())
        .min()
        .unwrap()
}

fn second_answer(input: &str) -> isize {
    let positions = input
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|p|
            positions
                .iter()
                .map(|c| c - p)
                .map(|c| c.abs())
                .map(|c| (1..=c).sum::<isize>())
                .sum()
        )
        .min()
        .unwrap()
}
