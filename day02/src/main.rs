fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> i32 {
    let (horizontal_position, depth) = input.lines()
        .map(|command| {
            let mut parts = command.split_whitespace();
            match parts.next() {
                Some("forward") => (parts.next().map(|value| value.parse::<i32>().unwrap()).unwrap(), 0),
                Some("down") => (0, parts.next().map(|value| value.parse::<i32>().unwrap()).unwrap()),
                Some("up") => (0, -parts.next().map(|value| value.parse::<i32>().unwrap()).unwrap()),
                _ => panic!(),
            }
        })
        .fold((0, 0), |(dh, dd), (ch, cd)| (ch + dh, cd + dd));
    horizontal_position * depth
}

fn second_answer(input: &str) -> i32 {
    let mut aim = 0;
    let mut horizontal_position = 0;
    let mut depth = 0;
    for command in input.lines() {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().unwrap();
        let val = parts.next().unwrap().parse::<i32>().unwrap();
        match cmd {
            "forward" => {
                horizontal_position += val;
                depth += aim * val;
            },
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!(),
        }
    }
    horizontal_position * depth
}
