use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
}

fn first_answer(input: &str) -> usize {
    let sea_cucumbers = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (x, y, c == '>'))
        })
        .collect::<Vec<_>>();
    let width = 1 + sea_cucumbers.iter()
        .map(|(x, _, _)| x)
        .max()
        .unwrap();
    let height = 1 + sea_cucumbers.iter()
        .map(|(_, y, _)| y)
        .max()
        .unwrap();
    let mut current_east_facing = sea_cucumbers.iter()
        .filter(|(_, _, is_east_facing)| *is_east_facing)
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<_>>();
    let mut current_south_facing = sea_cucumbers.iter()
        .filter(|(_, _, is_east_facing)| !*is_east_facing)
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<_>>();
    for n in 1.. {
        let mut next_east_facing = HashSet::new();
        let mut next_south_facing = HashSet::new();
        for (x, y) in current_east_facing.iter() {
            let next_x = (*x + 1) % width;
            if current_east_facing.contains(&(next_x, *y)) || current_south_facing.contains(&(next_x, *y)) {
                next_east_facing.insert((*x, *y));
            } else {
                next_east_facing.insert((next_x, *y));
            }
        }
        for (x, y) in current_south_facing.iter() {
            let next_y = (*y + 1) % height;
            if next_east_facing.contains(&(*x, next_y)) || current_south_facing.contains(&(*x, next_y)) {
                next_south_facing.insert((*x, *y));
            } else {
                next_south_facing.insert((*x, next_y));
            }
        }
        if current_east_facing == next_east_facing && current_south_facing == next_south_facing {
            return n;
        } else {
            current_east_facing = next_east_facing;
            current_south_facing = next_south_facing;
        }
    }
    unreachable!();
}

pub fn print_sea_cucumbers(east_facing: &HashSet<(usize, usize)>, south_facing: &HashSet<(usize, usize)>) {
    let width = 1 + east_facing.iter()
        .chain(south_facing.iter())
        .map(|(x, _)| x)
        .max()
        .unwrap();
    let height = 1 + east_facing.iter()
        .chain(south_facing.iter())
        .map(|(_, y)| y)
        .max()
        .unwrap();
    for y in 0..height {
        for x in 0..width {
            let c = if east_facing.contains(&(x, y)) {
                '>'
            } else if south_facing.contains(&(x, y)) {
                'v'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}
