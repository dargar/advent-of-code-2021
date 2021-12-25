use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", answer(input, 2));
    println!("First answer: {}", answer(input, 50));
}

fn answer(input: &str, steps: usize) -> usize {
    let mut parts = input.split("\n\n");
    let iea = parts.next().unwrap().trim().chars().collect::<Vec<char>>();
    let mut image = parts
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect::<HashSet<(i64, i64)>>();
    for n in 0..steps {
        let minx = image.iter().map(|(x, _)| x).min().unwrap();
        let maxx = image.iter().map(|(x, _)| x).max().unwrap();
        let miny = image.iter().map(|(_, y)| y).min().unwrap();
        let maxy = image.iter().map(|(_, y)| y).max().unwrap();
        let mut output = HashSet::new();
        for y in miny - 2..=*maxy {
            for x in minx - 2..=*maxx {
                let pixels = vec![
                    (x, y),
                    (x + 1, y),
                    (x + 2, y),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 2, y + 1),
                    (x, y + 2),
                    (x + 1, y + 2),
                    (x + 2, y + 2),
                ];
                let mut index = 0;
                for c in pixels {
                    index <<= 1;
                    if (*minx <= c.0 && c.0 <= *maxx) && (*miny <= c.1 && c.1 <= *maxy) {
                        index |= if image.contains(&c) { 1 } else { 0 };
                    } else {
                        index |= n % 2;
                    }
                }
                if iea[index] == '#' {
                    output.insert((x + 1, y + 1));
                }
            }
        }
        image = output;
    }
    image.len()
}

pub fn print_image(image: &HashSet<(i64, i64)>) {
    let minx = image.iter().map(|(x, _)| x).min().unwrap();
    let maxx = image.iter().map(|(x, _)| x).max().unwrap();
    let miny = image.iter().map(|(_, y)| y).min().unwrap();
    let maxy = image.iter().map(|(_, y)| y).max().unwrap();
    for y in *miny - 1..=*maxy + 1 {
        for x in *minx - 1..=*maxx + 1 {
            if image.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
