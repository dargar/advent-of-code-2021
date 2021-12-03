fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut result = vec![0; 12];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                result[i] -= 1;
            } else if c == '1' {
                result[i] += 1;
            }
        }
    }
    let s = result.into_iter()
        .map(|n| if n <= 0 { '0' } else { '1' })
        .collect::<String>();
    let gamma = usize::from_str_radix(&s, 2).unwrap();
    let epsilon = 0b111111111111 & (!gamma);
    gamma * epsilon
}

fn second_answer(input: &str) -> usize {
    o2(input) * co2(input)
}

fn o2(input: &str) -> usize {
    let mut ns = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut bit = 0;
    loop {
        let mut x = 0;
        for n in &ns {
            if n[bit] == '0' {
                x -= 1;
            } else {
                x += 1;
            }
        }
        let most_common = if x < 0 { '0' } else { '1' };

        let mut index = 0;
        while index < ns.len() && ns.len() > 1 {
            let n = ns[index].clone();
            if n[bit] != most_common {
                ns.remove(index);
            } else {
                index += 1;
            }
        }
        if ns.len() == 1 {
            return usize::from_str_radix(&ns[0].iter().collect::<String>(), 2).unwrap();
        }

        bit += 1;
    }
}

fn co2(input: &str) -> usize {
    let mut ns = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut bit = 0;
    loop {
        let mut x = 0;
        for n in &ns {
            if n[bit] == '0' {
                x -= 1;
            } else {
                x += 1;
            }
        }
        let least_common = if x < 0 { '1' } else { '0' };

        let mut index = 0;
        while index < ns.len() && ns.len() > 1 {
            let n = ns[index].clone();
            if n[bit] != least_common {
                ns.remove(index);
            } else {
                index += 1;
            }
        }
        if ns.len() == 1 {
            return usize::from_str_radix(&ns[0].iter().collect::<String>(), 2).unwrap();
        }

        bit += 1;
    }
}
