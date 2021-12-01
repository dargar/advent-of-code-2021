fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count()
}

fn second_answer(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count()
}
